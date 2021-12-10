use std::{
    collections::{BTreeMap, HashMap},
    net::SocketAddr,
};

use k8s_openapi::{
    api::core::v1::{Service, ServicePort, ServiceSpec},
    apimachinery::pkg::{apis::meta::v1::ObjectMeta, util::intstr::IntOrString},
    Resource,
};
use kubernetes_applier_interface::{DeleteRequest, KubernetesApplier, KubernetesApplierSender};
use wasmbus_rpc::{actor::prelude::*, core::LinkDefinition};
use wasmcloud_interface_logging::debug;
use wasmcloud_interface_messaging::{MessageSubscriber, MessageSubscriberReceiver, SubMessage};

const LINKDEF_SET_EVENT_TYPE: &str = "com.wasmcloud.lattice.linkdef_set";
const LINKDEF_DELETED_EVENT_TYPE: &str = "com.wasmcloud.lattice.linkdef_deleted";
const EXPECTED_CONTRACT_ID: &str = "wasmcloud:httpserver";

const DATA_KEY: &str = "data";
const ADDRESS_KEY: &str = "address";
const PORT_KEY: &str = "port";
const LABEL_PREFIX: &str = "wasmcloud.dev";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MessageSubscriber)]
struct ServiceApplierActor {}

struct EventWrapper {
    raw: serde_json::Value,
}

// TODO: How do we handle configuring existing links/deleting links that no longer exist? A re-sync event?

impl EventWrapper {
    fn ty(&self) -> RpcResult<&str> {
        unwrap_the_thingz(&self.raw, "type")
    }

    fn contract_id(&self) -> RpcResult<&str> {
        let data = self.raw.get(DATA_KEY).ok_or_else(|| {
            RpcError::InvalidParameter(format!("Event does not have key {}", DATA_KEY))
        })?;
        unwrap_the_thingz(data, "contract_id")
    }

    /// Returns the linkdef values by deserializing them. This allows for lazy
    /// deserialization only when the type and contract ID match. This will normalize the value keys
    /// to lowercase
    fn linkdef(&self) -> RpcResult<LinkDefinition> {
        let value = self.raw.get(DATA_KEY).ok_or_else(|| {
            RpcError::InvalidParameter(format!("Event does not have key {}", DATA_KEY))
        })?;

        let mut ld: LinkDefinition =
            serde_json::from_value(value.to_owned()).map_err(|e| RpcError::Deser(e.to_string()))?;

        ld.values = ld
            .values
            .into_iter()
            .map(|(k, v)| (k.to_lowercase(), v))
            .collect();

        Ok(ld)
    }
}

fn unwrap_the_thingz<'a>(thing: &'a serde_json::Value, key: &str) -> RpcResult<&'a str> {
    thing
        .get(key)
        .ok_or_else(|| RpcError::InvalidParameter(format!("Event does not have key {}", key)))?
        .as_str()
        .ok_or_else(|| RpcError::InvalidParameter(format!("Event does not have key {}", key)))
}

#[async_trait]
impl MessageSubscriber for ServiceApplierActor {
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        let raw: serde_json::Value = serde_json::from_slice(&msg.body)
            .map_err(|e| RpcError::Deser(format!("Invalid JSON data in message: {}", e)))?;
        let evt = EventWrapper { raw };

        let event_type = evt.ty()?;
        match event_type {
            LINKDEF_SET_EVENT_TYPE if evt.contract_id()? == EXPECTED_CONTRACT_ID => {
                debug!("Found new link definition for HTTP server");
                handle_apply(ctx, evt.linkdef()?).await
            }
            LINKDEF_DELETED_EVENT_TYPE if evt.contract_id()? == EXPECTED_CONTRACT_ID => {
                debug!("Link definition for HTTP server deleted");
                handle_delete(ctx, evt.linkdef()?).await
            }
            _ => {
                debug!("Skipping non-linkdef event {}", event_type);
                Ok(())
            }
        }
    }
}

async fn handle_apply(ctx: &Context, ld: LinkDefinition) -> RpcResult<()> {
    let sender = KubernetesApplierSender::new();
    let port = get_port(ld.values)?;
    let svc_name = ld.actor_id.to_lowercase();

    let mut labels = BTreeMap::new();
    labels.insert(format!("{}/{}", LABEL_PREFIX, "actor-id"), ld.actor_id);
    // We can't put in the full contract ID because it contains `:`, which isn't allowed in k8s
    labels.insert(
        format!("{}/{}", LABEL_PREFIX, "contract"),
        // SAFETY: We can unwrap because the contract ID is something we own and we know it has a `:`
        EXPECTED_CONTRACT_ID.rsplit_once(':').unwrap().1.to_owned(),
    );
    labels.insert(format!("{}/{}", LABEL_PREFIX, "link-name"), ld.link_name);
    labels.insert(
        format!("{}/{}", LABEL_PREFIX, "provider-id"),
        ld.provider_id,
    );

    let mut selector = BTreeMap::new();
    // Select pods that have a label of `wasmcloud.dev/route-to=true`
    selector.insert(
        format!("{}/{}", LABEL_PREFIX, "route-to"),
        "true".to_string(),
    );

    debug!(
        "Applying new kubernetes resource with name {}, listening on port {}, with labels {:?}, and selecting pods with labels matching {:?}",
        svc_name,
        port,
        labels,
        selector
    );

    // NOTE: If you have more than one type of contract you are handling, you'll likely want to have
    // some sort of data store that maps a unique service name to the full link definition. For
    // here, you can only have one linkdef of this type for an actor, so we just use the lowercased
    // actor key
    let resp = sender
        .apply(
            ctx,
            &serde_yaml::to_vec(&Service {
                metadata: ObjectMeta {
                    name: Some(svc_name),
                    labels: Some(labels),
                    ..Default::default()
                },
                spec: Some(ServiceSpec {
                    selector: Some(selector),
                    ports: Some(vec![ServicePort {
                        protocol: Some("TCP".to_string()),
                        port,
                        target_port: Some(IntOrString::Int(port)),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .expect("Unable to serialize Service to yaml. This is programmer error"),
        )
        .await?;

    if !resp.succeeded {
        return Err(RpcError::ActorHandler(format!(
            "Unable to apply kubernetes service: {}",
            resp.error.unwrap_or_default()
        )));
    }

    Ok(())
}

async fn handle_delete(ctx: &Context, ld: LinkDefinition) -> RpcResult<()> {
    let sender = KubernetesApplierSender::new();
    let svc_name = ld.actor_id.to_lowercase();

    debug!(
        "Deleting Kubernetes service with name {} from related linkdef {}-{}-{}",
        svc_name, ld.actor_id, ld.provider_id, ld.link_name
    );

    let resp = sender
        .delete(
            ctx,
            &DeleteRequest {
                group: Service::GROUP.to_owned(),
                kind: Service::KIND.to_owned(),
                version: Service::VERSION.to_owned(),
                name: svc_name,
                namespace: None,
            },
        )
        .await?;

    if !resp.succeeded {
        return Err(RpcError::ActorHandler(format!(
            "Unable to delete kubernetes service: {}",
            resp.error.unwrap_or_default()
        )));
    }

    Ok(())
}

fn get_port(values: HashMap<String, String>) -> RpcResult<i32> {
    let port = if let Some(p) = values.get(PORT_KEY) {
        p.parse()
            .map_err(|_| RpcError::InvalidParameter("Port value is malformed".to_string()))?
    } else if let Some(p) = values.get(ADDRESS_KEY) {
        let addr: SocketAddr = p
            .parse()
            .map_err(|_| RpcError::InvalidParameter("Address value is malformed".to_string()))?;
        addr.port() as i32
    } else {
        // The default port from the HTTP server is 8080, so we are going to default it here as well
        8080
    };

    Ok(port)
}
