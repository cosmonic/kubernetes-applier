// This file is generated automatically using wasmcloud/weld-codegen 0.4.2

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
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

// Encode DeleteRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_delete_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &DeleteRequest,
) -> RpcResult<()> {
    e.map(5)?;
    e.str("group")?;
    e.str(&val.group)?;
    e.str("kind")?;
    e.str(&val.kind)?;
    e.str("name")?;
    e.str(&val.name)?;
    if let Some(val) = val.namespace.as_ref() {
        e.str("namespace")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("version")?;
    e.str(&val.version)?;
    Ok(())
}

// Decode DeleteRequest from cbor input stream
#[doc(hidden)]
pub fn decode_delete_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<DeleteRequest, RpcError> {
    let __result = {
        let mut group: Option<String> = None;
        let mut kind: Option<String> = None;
        let mut name: Option<String> = None;
        let mut namespace: Option<Option<String>> = Some(None);
        let mut version: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct DeleteRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct DeleteRequest: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => group = Some(d.str()?.to_string()),
                    1 => kind = Some(d.str()?.to_string()),
                    2 => name = Some(d.str()?.to_string()),
                    3 => {
                        namespace = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    4 => version = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct DeleteRequest: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "group" => group = Some(d.str()?.to_string()),
                    "kind" => kind = Some(d.str()?.to_string()),
                    "name" => name = Some(d.str()?.to_string()),
                    "namespace" => {
                        namespace = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "version" => version = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        DeleteRequest {
            group: if let Some(__x) = group {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field DeleteRequest.group (#0)".to_string(),
                ));
            },

            kind: if let Some(__x) = kind {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field DeleteRequest.kind (#1)".to_string(),
                ));
            },

            name: if let Some(__x) = name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field DeleteRequest.name (#2)".to_string(),
                ));
            },
            namespace: namespace.unwrap(),

            version: if let Some(__x) = version {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field DeleteRequest.version (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
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

// Encode OperationResponse as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_operation_response<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &OperationResponse,
) -> RpcResult<()> {
    e.map(2)?;
    if let Some(val) = val.error.as_ref() {
        e.str("error")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("succeeded")?;
    e.bool(val.succeeded)?;
    Ok(())
}

// Decode OperationResponse from cbor input stream
#[doc(hidden)]
pub fn decode_operation_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<OperationResponse, RpcError> {
    let __result = {
        let mut error: Option<Option<String>> = Some(None);
        let mut succeeded: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct OperationResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct OperationResponse: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    1 => succeeded = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct OperationResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "error" => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "succeeded" => succeeded = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        OperationResponse {
            error: error.unwrap(),

            succeeded: if let Some(__x) = succeeded {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field OperationResponse.succeeded (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
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
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "Apply" => {
                let value: Vec<u8> = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'Blob': {}", e)))?;
                let resp = KubernetesApplier::apply(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "KubernetesApplier.Apply",
                    arg: Cow::Owned(buf),
                })
            }
            "Delete" => {
                let value: DeleteRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'DeleteRequest': {}", e)))?;
                let resp = KubernetesApplier::delete(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
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
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
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
        let buf = wasmbus_rpc::common::serialize(arg)?;
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

        let value: OperationResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': OperationResponse", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Attempts to delete an object with the given GVK (group, version, kind), name, and namespace.
    /// This should be idempotent, meaning that it should return successful if the object doesn't exist
    async fn delete(&self, ctx: &Context, arg: &DeleteRequest) -> RpcResult<OperationResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
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

        let value: OperationResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': OperationResponse", e)))?;
        Ok(value)
    }
}
