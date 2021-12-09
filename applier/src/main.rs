//! Kubernetes applier capability provider
//!
//!
use kube::{
    api::{DeleteParams, DynamicObject, PatchParams, PostParams},
    config::{KubeConfigOptions, Kubeconfig},
    core::{params::Patch, ApiResource, GroupVersionKind},
    Api, Client, Config,
};
use kubernetes_applier_interface::{
    DeleteRequest, KubernetesApplier, KubernetesApplierReceiver, OperationResponse,
};
use tokio::sync::RwLock;
use tracing::{debug, info, instrument, trace};
use wasmbus_rpc::provider::prelude::*;

use std::collections::HashMap;
use std::sync::Arc;

/// Loading a kubeconfig from a file
const CONFIG_FILE_KEY: &str = "config_file";
/// Passing a kubeconfig as a base64 encoding string. This config should contain embedded
/// certificates rather than paths to certificates
const CONFIG_B64_KEY: &str = "config_b64";

const CERT_PATH_ERROR: &str =
    "Certificate and key paths are not allowed for base64 encoded configs. Offending entry:";
const FIELD_MANAGER: &str = "kubernetes-applier-provider";

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting provider process");
    provider_main(ApplierProvider::default())?;

    info!("Applier provider exiting");
    Ok(())
}

/// applier capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(KubernetesApplier)]
struct ApplierProvider {
    clients: Arc<RwLock<HashMap<String, Client>>>,
}

impl ProviderDispatch for ApplierProvider {}
#[async_trait]
impl ProviderHandler for ApplierProvider {
    #[instrument(level = "debug", skip(self, ld), fields(actor_id = %ld.actor_id))]
    async fn put_link(&self, ld: &LinkDefinition) -> Result<bool, RpcError> {
        debug!("Got link request");
        // Normalize keys to lowercase
        let values: HashMap<String, String> = ld
            .values
            .iter()
            .map(|(k, v)| (k.to_lowercase(), v.to_owned()))
            .collect();

        // Attempt to load the config. If nothing it passed attempt to infer it from the pod or the
        // default kubeconfig path
        let config = if let Some(p) = values.get(CONFIG_FILE_KEY) {
            let path = p.to_owned();
            debug!(%path, "Loading kubeconfig from file");
            let conf = tokio::task::spawn_blocking(move || Kubeconfig::read_from(path))
                .await
                .map_err(|e| {
                    RpcError::ProviderInit(format!(
                        "Internal error occured while loading kubeconfig: {}",
                        e
                    ))
                })?
                .map_err(|e| format!("Invalid kubeconfig from file {}: {}", p, e))?;
            Config::from_custom_kubeconfig(conf, &KubeConfigOptions::default())
                .await
                .map_err(|e| {
                    RpcError::ProviderInit(format!("Invalid kubeconfig from file {}: {}", p, e))
                })?
        } else if let Some(raw) = values.get(CONFIG_B64_KEY) {
            debug!("Loading config from base64 encoded string");
            let decoded = base64::decode(raw).map_err(|e| {
                RpcError::ProviderInit(format!("Invalid base64 config given: {}", e))
            })?;
            // NOTE: We do not support multiple yaml documents in the same file. We shouldn't need
            // this, but if we do, we can borrow some of the logic from the `kube` crate
            let conf: Kubeconfig = serde_yaml::from_slice(&decoded).map_err(|e| {
                RpcError::ProviderInit(format!("Invalid kubeconfig data given: {}", e))
            })?;
            // Security: check that cert paths are not set as they could access certs on the host
            // runtime
            trace!("Ensuring base64 encoded config does not contain paths");
            for cluster in conf.clusters.iter() {
                ensure_no_path(
                    &cluster.cluster.certificate_authority,
                    "cluster",
                    &cluster.name,
                )?;
            }
            for user in conf.auth_infos.iter() {
                ensure_no_path(
                    &user.auth_info.client_certificate,
                    "client_certificate",
                    &user.name,
                )?;
                ensure_no_path(&user.auth_info.client_key, "client_key", &user.name)?;
                ensure_no_path(&user.auth_info.token_file, "token_file", &user.name)?;
            }
            Config::from_custom_kubeconfig(conf, &KubeConfigOptions::default())
                .await
                .map_err(|e| {
                    RpcError::ProviderInit(format!("Invalid kubeconfig from base64: {}", e))
                })?
        } else {
            debug!("No config given, inferring config from environment");
            // If no config was manually specified we try to infer it from local pod variables or
            // the default kubeconfig path
            Config::infer().await.map_err(|e| RpcError::ProviderInit(format!("No config given and unable to infer config from environment or default config file: {}", e)))?
        };

        tracing::trace!(?config, "Attempting to create client and connect to server");
        // Now create the client and make sure it works
        let client = Client::try_from(config).map_err(|e| {
            RpcError::ProviderInit(format!(
                "Unable to create client from loaded kubeconfig: {}",
                e
            ))
        })?;

        // NOTE: In the future, we may want to improve this with a retry
        client.apiserver_version().await.map_err(|e| {
            RpcError::ProviderInit(format!(
                "Unable to connect to the Kubernetes API server: {}",
                e
            ))
        })?;
        tracing::trace!("Successfully connected to server");

        let mut clients = self.clients.write().await;
        clients.insert(ld.actor_id.clone(), client);
        Ok(true)
    }

    async fn delete_link(&self, actor_id: &str) {
        self.clients.write().await.remove(actor_id);
    }
}

#[async_trait]
impl KubernetesApplier for ApplierProvider {
    #[instrument(level = "debug", skip(self, ctx, arg), fields(actor_id = ?ctx.actor, object_name = tracing::field::Empty))]
    async fn apply(&self, ctx: &Context, arg: &Vec<u8>) -> RpcResult<OperationResponse> {
        trace!(body_len = arg.len(), "Decoding object for apply");
        let object: DynamicObject = serde_yaml::from_slice(arg).map_err(|e| {
            RpcError::InvalidParameter(format!("Unable to parse data as kubernetes object: {}", e))
        })?;

        let obj_name = object
            .metadata
            .name
            .as_ref()
            .ok_or_else(|| {
                RpcError::InvalidParameter("The given object is missing a name".to_string())
            })?
            .as_str();

        tracing::span::Span::current().record("object_name", &tracing::field::display(obj_name));

        let type_data = object.types.as_ref().ok_or_else(|| {
            RpcError::InvalidParameter(
                "The given manifest does not contain type information".to_string(),
            )
        })?;
        // Decompose api_version into the parts we need to type the request
        let (group, version) = match type_data.api_version.split_once('/') {
            Some((g, v)) => (g.to_owned(), v.to_owned()),
            None => (String::new(), type_data.api_version.to_owned()),
        };
        let gvk = GroupVersionKind {
            group,
            version,
            kind: type_data.kind.clone(),
        };
        let resource = ApiResource::from_gvk(&gvk);

        trace!(?gvk, "Inferred object type from data");

        let client = self.get_client(ctx).await?;

        let api: Api<DynamicObject> = if let Some(ns) = object.metadata.namespace.as_ref() {
            Api::namespaced_with(client, ns.as_str(), &resource)
        } else {
            Api::default_namespaced_with(client, &resource)
        };

        debug!("Attempting to apply object to api");

        trace!("Checking if object already exists");
        let exists = match api.get(obj_name).await {
            Ok(_) => true,
            Err(kube::Error::Api(e)) if e.code == 404 => false,
            // TODO: retries in case of flakiness?
            Err(e) => {
                return Ok(OperationResponse {
                    succeeded: false,
                    error: Some(format!("Unable to fetch object from API: {}", e)),
                })
            }
        };

        let resp = if exists {
            trace!("Object already exists, attempting server-side apply");
            api.patch(
                obj_name,
                &PatchParams {
                    field_manager: Some(FIELD_MANAGER.to_string()),
                    ..Default::default()
                },
                &Patch::Apply(&object),
            )
            .await
        } else {
            trace!("Object does not exist, creating");
            api.create(
                &PostParams {
                    field_manager: Some(FIELD_MANAGER.to_string()),
                    ..Default::default()
                },
                &object,
            )
            .await
        };

        if let Err(e) = resp {
            return Ok(OperationResponse {
                succeeded: false,
                error: Some(e.to_string()),
            });
        }

        Ok(OperationResponse {
            succeeded: true,
            error: None,
        })
    }

    #[instrument(level = "debug", skip(self, ctx), fields(actor_id = ?ctx.actor))]
    async fn delete(&self, ctx: &Context, arg: &DeleteRequest) -> RpcResult<OperationResponse> {
        let client = self.get_client(ctx).await?;

        let resource = ApiResource::from_gvk(&GroupVersionKind {
            group: arg.group.clone(),
            version: arg.version.clone(),
            kind: arg.kind.clone(),
        });

        let api: Api<DynamicObject> = if let Some(ns) = arg.namespace.as_ref() {
            Api::namespaced_with(client, ns.as_str(), &resource)
        } else {
            Api::default_namespaced_with(client, &resource)
        };
        debug!("Attempting to delete object");
        match api
            .delete(arg.name.as_str(), &DeleteParams::default())
            .await
        {
            // If it is ok or returns not found, that means we are ok
            Ok(_) => Ok(OperationResponse {
                succeeded: true,
                error: None,
            }),
            Err(kube::Error::Api(e)) if e.code == 404 => Ok(OperationResponse {
                succeeded: true,
                error: None,
            }),
            Err(e) => Ok(OperationResponse {
                succeeded: false,
                error: Some(e.to_string()),
            }),
        }
    }
}

impl ApplierProvider {
    async fn get_client(&self, ctx: &Context) -> RpcResult<Client> {
        let actor_id = ctx.actor.as_ref().ok_or_else(|| {
            RpcError::InvalidParameter("Actor ID does not exist on request".to_string())
        })?;
        Ok(self
            .clients
            .read()
            .await
            .get(actor_id.as_str())
            .ok_or_else(|| {
                RpcError::InvalidParameter(format!("No link registered for actor {}", actor_id))
            })?
            .clone())
    }
}

fn ensure_no_path(item: &Option<String>, entity: &str, name: &str) -> Result<(), RpcError> {
    if item.is_some() {
        return Err(RpcError::ProviderInit(format!(
            "{} {} {}",
            CERT_PATH_ERROR, entity, name
        )));
    }
    Ok(())
}
