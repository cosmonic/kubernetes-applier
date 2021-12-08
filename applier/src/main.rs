//! Kubernetes applier capability provider
//!
//!
use kubernetes_applier_interface::{
    DeleteRequest, KubernetesApplier, KubernetesApplierReceiver, OperationResponse,
};
use wasmbus_rpc::provider::prelude::*;

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Actually grab a level from configuration on startup
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        .init();
    provider_main(ApplierProvider::default())?;

    eprintln!("applier provider exiting");
    Ok(())
}

/// applier capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(KubernetesApplier)]
struct ApplierProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for ApplierProvider {}
impl ProviderHandler for ApplierProvider {}

#[async_trait]
impl KubernetesApplier for ApplierProvider {
    async fn apply(&self, ctx: &Context, arg: &Vec<u8>) -> RpcResult<OperationResponse> {
        todo!()
    }

    async fn delete(&self, ctx: &Context, arg: &DeleteRequest) -> RpcResult<OperationResponse> {
        todo!()
    }
}
