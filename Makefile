WORKSPACES="./" "./stream-delimit/"
DOCKER_BIN ?= docker
DOCKER_IMAGE=docker.io/clux/muslrust
DOCKER_ARGS=run -v $(PWD):/volume:Z -w /volume -t $(DOCKER_IMAGE)
CARGO_TOKEN:=$(shell grep 'token' ~/.cargo/credentials.toml | cut -d'"' -f2)

all: debug

docker:
	$(DOCKER_BIN) pull $(DOCKER_IMAGE)

debug: docker
	$(DOCKER_BIN) $(DOCKER_ARGS) sh -c "cargo build --verbose"

release: docker
	$(DOCKER_BIN) $(DOCKER_ARGS) sh -c "cargo build --verbose --release"

test: docker
	$(DOCKER_BIN) $(DOCKER_ARGS) sh -c "cargo test --verbose"

publish: docker
	$(DOCKER_BIN) $(DOCKER_ARGS) sh -c "cargo login $(CARGO_TOKEN) && cd stream-delimit && cargo publish ; cd ../ && cd erased-serde-json && cargo publish ; cd ../ && cargo publish"

fmt:
	-cargo fmt --all
	-black utils/*.py

clippy:
	-cargo clippy --all

package: release
	tar -C target/x86_64-unknown-linux-musl/release -czvf pq-bin.tar.gz pq

.PHONY: all debug release package
