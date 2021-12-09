# Kubernetes Applier Capability Provider

This is a capability provider implementation of the `cosmonic:kubernetes_applier` contract. It's
purpose is to take arbitrary manifests from an actor and do the equivalent of a `kubectl apply` to
create the object.

## Using the provider

TODO: Put in the OCI reference here once we push

The only configuration required for linking to this provider is a valid kubeconfig. There are 3 ways
of doing this:

- The default (if no config is specified) will attempt to infer the kubeconfig from the default
  location (e.g. `$HOME/.kube/config`) or, if it is running in a pod, from the environment variables
  in the pod. This option is great for local testing and for running this provider within a host
  running in a pod.
- The `config_b64` key: The value of this key should be the base64 encoded kubeconfig the provider
  should use. Please note that this kubeconfig should have all certs and tokens embedded within the
  kubeconfig (i.e. `client-certificate-data`). If any file paths are used, the link will be
  rejected.
- The `config_file` key: A specific path where the kubeconfig should be loaded from. This option
  does allow file paths and is recommended when you have full control over the host and are storing
  the kubeconfig in a location other than the default

## Contributing

We welcome all contributions! If you would like to submit changes, please open a [Pull
Request](https://github.com/cosmonic/kubernetes-applier/pulls) and one of the maintainers will
review it

### Prerequisites

In order to build this module, you will need to have the following tools installed:

- `make`
- [`wash`](https://wasmcloud.dev/overview/installation/#install-wash)
- `jq`

### Building

To build the binary, simply run `make build`. To build and sign the provider for use with a
wasmCloud host, run `make`

### Testing

Before running the test, you need to have a valid kubeconfig pointing at a running Kubernetes
cluster (we recommend using [kind](https://kind.sigs.k8s.io/)).

For ease of testing, we use NATS in a docker image. The tests can be run manually by running `cargo
test --tests` if you wish to setup your own NATS server. Otherwise, you can just run `make test` to
run all tests.

#### Troubleshooting

For maximum compatibility, we use rustls for the TLS stack. However, this can cause issues with
kubeconfigs that contain a server IP address rather than a FQDN (such as those created by `kind`).
If you see an error about an unrecognized domain name, make sure the server entry in your kubeconfig
is using a domain name (e.g. switching `127.0.0.1` to `localhost`).
