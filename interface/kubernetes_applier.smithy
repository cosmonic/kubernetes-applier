// kubernetes_applier.smithy
// An interface that allows you to send Kubernetes manifests to a Kubernetes API. Basically the
// equivalent of `kubectl apply -f`

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "com.cosmonic.kubernetesapplier", crate: "kubernetes_applier_interface" } ]

namespace com.cosmonic.kubernetesapplier

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

/// The KubernetesApplier service has a two methods, one to apply an object (that can be a create or
/// update) and to delete an object
@wasmbus(
    contractId: "cosmonic:kubernetes_applier",
    providerReceive: true )
service KubernetesApplier {
  version: "0.1",
  operations: [ Apply, Delete ]
}

/// Attempts to create or update the arbitrary object it is given
operation Apply {
  input: Blob,
  output: OperationResponse
}

/// Attempts to delete an object with the given GVK (group, version, kind), name, and namespace.
/// This should be idempotent, meaning that it should return successful if the object doesn't exist
operation Delete {
  input: DeleteRequest,
  output: OperationResponse
}

structure OperationResponse {
  /// Whether or not the operation succeeded
  @required
  succeeded: Boolean,
  /// An optional message describing the error if one occurred
  error: String,
}

structure DeleteRequest {
  /// The group of the object you are deleting (e.g. "networking.k8s.io"). This will be an empty
  /// string if part of `core`
  @required 
  group: String,

  /// The API version of the object you are deleting (e.g. v1)
  @required
  version: String,

  /// The kind of the object you are deleting (e.g. Pod)
  @required
  kind: String,

  /// The name of the object you are deleting
  @required
  name: String,

  /// The namespace where the object you want to delete is located. If not specified, the default
  /// namespace for the context should be used
  namespace: String,
}
