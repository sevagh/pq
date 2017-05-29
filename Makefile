WORKSPACES="./" "./stream_delimit/"
CHOWN_CMD=&& chown -R 1000:1000 ./
DOCKER_ARGS=run -v cargo-cache:/root/.cargo -v $(PWD):/volume:Z -w /volume -t clux/muslrust

all: build-debug

docker:
	docker pull clux/muslrust

debug: docker
	docker $(DOCKER_ARGS) sh -c "cargo build --verbose $(CHOWN_CMD)"

release: docker
	docker $(DOCKER_ARGS) sh -c "cargo build --verbose --release $(CHOWN_CMD)"

test: docker
	docker $(DOCKER_ARGS) sh -c "cargo test --verbose $(CHOWN_CMD)"

lint:
	@- $(foreach WORKSPACE,$(WORKSPACES), \
		cd $(WORKSPACE) ;\
		rustup default stable && cargo fmt -- --write-mode=diff ;\
		rustup default nightly && cargo clippy ;\
		rustup default stable ;\
	)

package: release
	cd target/x86_64-unknown-linux-musl/release;\
		tar -czvf pq-bin.tar.gz pq;\
		cd -;\
		mv target/x86_64-unknown-linux-musl/release/pq-bin.tar.gz ./pq-bin.tar.gz 

.PHONY: all debug release lint package
