WORKSPACES="./" "./stream-delimit/"
CHOWN_CMD=&& chown -R 1000:1000 ./
DOCKER_ARGS=run -v cargo-cache:/root/.cargo -v $(PWD):/volume:Z -w /volume -t clux/muslrust

all: debug

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
		cargo +nightly fmt;\
		cargo +nightly clippy;\
		cd -;\
	)

package: release
	cd target/x86_64-unknown-linux-musl/release;\
		tar -czvf pq-bin.tar.gz pq;\
		cd -;\
		mv target/x86_64-unknown-linux-musl/release/pq-bin.tar.gz ./pq-bin.tar.gz 

fdset:
	$(MAKE) -C ./tests/ fdset

regen:
	$(MAKE) -C ./tests/ regen

.PHONY: all debug release lint package regen
