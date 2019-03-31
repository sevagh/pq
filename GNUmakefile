WORKSPACES="./" "./stream-delimit/"
CHOWN_CMD=; chown -R 1000:1000 ./
DOCKER_ARGS=run -v $(PWD):/volume:Z -w /volume -t clux/muslrust
CARGO_TOKEN:=$(shell grep 'token' ~/.cargo/credentials | cut -d'"' -f2)

all: debug

docker:
	docker pull clux/muslrust

debug: docker
	docker $(DOCKER_ARGS) sh -c "cargo build --verbose $(CHOWN_CMD)"

release: docker
	docker $(DOCKER_ARGS) sh -c "cargo build --verbose --release $(CHOWN_CMD)"

test: docker
	docker $(DOCKER_ARGS) sh -c "cargo test --verbose $(CHOWN_CMD)"

publish: docker
	docker $(DOCKER_ARGS) sh -c "cargo login $(CARGO_TOKEN) && cd stream-delimit && cargo publish ; cd ../ && cd erased-serde-json && cargo publish ; cd ../ && cargo publish $(CHOWN_CMD)"

fmt:
	-cargo fmt --all

clippy:
	-cargo clippy --all

package: release
	cd target/x86_64-unknown-linux-musl/release;\
		tar -czvf pq-bin.tar.gz pq;\
		cd -;\
		mv target/x86_64-unknown-linux-musl/release/pq-bin.tar.gz ./pq-bin.tar.gz 

.PHONY: all debug release lint package
