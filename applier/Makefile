# applier Makefile

CAPABILITY_ID = "cosmonic:kubernetes_applier"
NAME = "applier"
VENDOR = "cosmonic"
PROJECT = applier
VERSION = 0.1.0
REVISION = 0

include ./provider.mk

test-cleanup:
	docker stop applier-nats || true
	ps -ax | grep applier | grep -i -v make | awk '{print $1}' | xargs kill -9 || true
	kubectl delete svc -l wasmcloud.dev/test || true

test:: build test-cleanup
	cargo clippy --all-targets --all-features
	docker run --rm -d -p 4222:4222 --name applier-nats nats:2
	cargo test --tests
	$(MAKE) test-cleanup
