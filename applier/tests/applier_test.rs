use k8s_openapi::api::core::v1::Service;
use kube::Api;
use kubernetes_applier_interface::*;
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_test_util::{
    check,
    cli::print_test_results,
    provider_test::test_provider,
    testing::{TestOptions, TestResult},
};
#[allow(unused_imports)]
use wasmcloud_test_util::{run_selected, run_selected_spawn};

#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(&opts, health_check, create_update_delete_happy_path);
    print_test_results(&res);

    let passed = res.iter().filter(|tr| tr.passed).count();
    let total = res.len();
    assert_eq!(passed, total, "{} passed out of {}", passed, total);

    // try to let the provider shut down gracefully
    let provider = test_provider().await;
    let _ = provider.shutdown().await;
}

/// test that health check returns healthy
async fn health_check(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // health check
    let hc = prov.health_check().await;
    check!(hc.is_ok())?;
    Ok(())
}

const VALID_MANIFEST: &str = r#"apiVersion: v1
kind: Service
metadata:
  name: foo-applier-test-happy
spec:
  selector:
    wasmcloud.dev/test: "true"
    app.kubernetes.io/name: foo-applier
  ports:
    - protocol: TCP
      port: 8080
      targetPort: 8080"#;

const VALID_MANIFEST_WITH_LABELS: &str = r#"apiVersion: v1
kind: Service
metadata:
  name: foo-applier-test-happy
  labels:
    wasmcloud.dev/test: "true"
    foo: happy
spec:
  selector:
    app.kubernetes.io/name: foo-applier
  ports:
    - protocol: TCP
      port: 8080
      targetPort: 8080"#;

/// Test the happy path of creating updating and deleting
async fn create_update_delete_happy_path(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;
    let svc_name = "foo-applier-test-happy";

    let client = kube::Client::try_default()
        .await
        .expect("Unable to get client");
    let api: Api<Service> = Api::default_namespaced(client);

    // The test scaffolding doesn't wait for an ack from the link, so wait for a bit
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let actor_id = prov.origin().public_key();
    // create client and ctx
    let client = KubernetesApplierSender::via(prov);
    let ctx = Context {
        actor: Some(actor_id),
        ..Default::default()
    };

    let resp = client
        .apply(&ctx, &VALID_MANIFEST.as_bytes().to_vec())
        .await?;
    assert!(resp.succeeded, "Create should have succeeded");

    // Validate service exists
    api.get(svc_name)
        .await
        .unwrap_or_else(|_| panic!("Service {} does not exist", svc_name));

    let resp = client
        .apply(&ctx, &VALID_MANIFEST_WITH_LABELS.as_bytes().to_vec())
        .await?;
    assert!(resp.succeeded, "Update should have succeeded");

    let svc = api
        .get(svc_name)
        .await
        .unwrap_or_else(|_| panic!("Service {} does not exist", svc_name));

    assert_eq!(
        svc.metadata
            .labels
            .expect("Should have labels present")
            .get("foo")
            .expect("foo label doesn't exist"),
        "happy",
        "Label value should be set correctly"
    );

    let resp = client
        .delete(
            &ctx,
            &DeleteRequest {
                group: String::new(),
                kind: "Service".into(),
                version: "v1".into(),
                name: svc_name.into(),
                ..Default::default()
            },
        )
        .await?;
    assert!(resp.succeeded, "Delete should have succeeded");
    if api.get(svc_name).await.is_ok() {
        panic!("Service {} should be deleted", svc_name)
    }
    Ok(())
}

// TODO: Test base64 config and file path config once https://github.com/wasmCloud/wasmcloud-test/issues/6 is fixed
