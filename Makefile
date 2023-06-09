WORKSPACES="./" "./stream-delimit/"
DOCKER_IMAGE=docker.io/clux/muslrust
DOCKER_ARGS=run -v $(PWD):/volume:Z -w /volume -t $(DOCKER_IMAGE)
CARGO_TOKEN:=$(shell grep 'token' ~/.cargo/credentials.toml | cut -d'"' -f2)

all: debug

docker:
	podman pull $(DOCKER_IMAGE)

debug: docker
	podman $(DOCKER_ARGS) sh -c "cargo build --verbose"

release: docker
	podman $(DOCKER_ARGS) sh -c "cargo build --verbose --release"

test: docker
	podman $(DOCKER_ARGS) sh -c "cargo test --verbose"

publish: docker
	podman $(DOCKER_ARGS) sh -c "cargo login $(CARGO_TOKEN) && cd stream-delimit && cargo publish ; cd ../ && cd erased-serde-json && cargo publish ; cd ../ && cargo publish"

fmt:
	-cargo fmt --all
	-black utils/*.py

clippy:
	-cargo clippy --all

package: release
	cd target/x86_64-unknown-linux-musl/release;\
		tar -czvf pq-bin.tar.gz pq;\
		cd -;\
		mv target/x86_64-unknown-linux-musl/release/pq-bin.tar.gz ./pq-bin.tar.gz 

.PHONY: all debug release package
