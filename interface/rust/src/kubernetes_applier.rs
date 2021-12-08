// This file is generated automatically using wasmcloud/weld-codegen and smithy model definitions
//

#![allow(unused_imports, clippy::ptr_arg, clippy::needless_lifetimes)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Write, string::ToString};
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Timestamp, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeleteRequest {
    /// The group of the object you are deleting (e.g. "networking.k8s.io"). This will be an empty
    /// string if part of `core`
    #[serde(default)]
    pub group: String,
    /// The kind of the object you are deleting (e.g. Pod)
    #[serde(default)]
    pub kind: String,
    /// The name of the object you are deleting
    #[serde(default)]
    pub name: String,
    /// The namespace where the object you want to delete is located. If not specified, the default
    /// namespace for the context should be used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// The API version of the object you are deleting (e.g. v1)
    #[serde(default)]
    pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationResponse {
    /// An optional message describing the error if one occurred
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Whether or not the operation succeeded
    #[serde(default)]
    pub succeeded: bool,
}

/// The KubernetesApplier service has a two methods, one to apply an object (that can be a create or
/// update) and to delete an object
/// wasmbus.contractId: cosmonic:kubernetes_applier
/// wasmbus.providerReceive
#[async_trait]
pub trait KubernetesApplier {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "cosmonic:kubernetes_applier"
    }
    /// Attempts to create or update the arbitrary object it is given
    async fn apply(&self, ctx: &Context, arg: &Vec<u8>) -> RpcResult<OperationResponse>;
    /// Attempts to delete an object with the given GVK (group, version, kind), name, and namespace.
    /// This should be idempotent, meaning that it should return successful if the object doesn't exist
    async fn delete(&self, ctx: &Context, arg: &DeleteRequest) -> RpcResult<OperationResponse>;
}

/// KubernetesApplierReceiver receives messages defined in the KubernetesApplier service trait
/// The KubernetesApplier service has a two methods, one to apply an object (that can be a create or
/// update) and to delete an object
#[doc(hidden)]
#[async_trait]
pub trait KubernetesApplierReceiver: MessageDispatch + KubernetesApplier {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "Apply" => {
                let value: Vec<u8> = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KubernetesApplier::apply(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "KubernetesApplier.Apply",
                    arg: Cow::Owned(buf),
                })
            }
            "Delete" => {
                let value: DeleteRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KubernetesApplier::delete(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "KubernetesApplier.Delete",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "KubernetesApplier::{}",
                message.method
            ))),
        }
    }
}

/// KubernetesApplierSender sends messages to a KubernetesApplier service
/// The KubernetesApplier service has a two methods, one to apply an object (that can be a create or
/// update) and to delete an object
/// client for sending KubernetesApplier messages
#[derive(Debug)]
pub struct KubernetesApplierSender<T: Transport> {
    transport: T,
}

impl<T: Transport> KubernetesApplierSender<T> {
    /// Constructs a KubernetesApplierSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl KubernetesApplierSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a KubernetesApplier provider
    /// implementing the 'cosmonic:kubernetes_applier' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "cosmonic:kubernetes_applier",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a KubernetesApplier provider
    /// implementing the 'cosmonic:kubernetes_applier' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "cosmonic:kubernetes_applier",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> KubernetesApplier
    for KubernetesApplierSender<T>
{
    #[allow(unused)]
    /// Attempts to create or update the arbitrary object it is given
    async fn apply(&self, ctx: &Context, arg: &Vec<u8>) -> RpcResult<OperationResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "KubernetesApplier.Apply",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Apply", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Attempts to delete an object with the given GVK (group, version, kind), name, and namespace.
    /// This should be idempotent, meaning that it should return successful if the object doesn't exist
    async fn delete(&self, ctx: &Context, arg: &DeleteRequest) -> RpcResult<OperationResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "KubernetesApplier.Delete",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Delete", e)))?;
        Ok(value)
    }
}
