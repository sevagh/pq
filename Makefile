WORKSPACES = "./" "./stream_delimit/"

all: build-debug

docker:
	docker pull clux/muslrust

build-debug: docker
	docker run \
		-v $(PWD):/volume:Z -w /volume \
		-t clux/muslrust \
		cargo build --verbose &&\
		chown -R 1000:1000 ./

build-release: docker
	docker run \
		-v $(PWD):/volume:Z -w /volume \
		-t clux/muslrust \
		cargo build --verbose --release &&\
		chown -R 1000:1000 ./

test: docker
	docker run \
		-v $(PWD):/volume:Z -w /volume \
		-t clux/muslrust \
		-e PQ_TESTS_PATH=/volume/tests \
		cargo test --verbose &&\
		chown -R 1000:1000 ./

lint:
	@- $(foreach WORKSPACE,$(WORKSPACES), \
		cd $(WORKSPACE) ;\
		rustup default stable && cargo fmt -- --write-mode=diff ;\
		rustup default nightly && cargo clippy ;\
		rustup default stable ;\
	)

package: build-release
	cd target/x86_64-unknown-linux-musl/release;\
		tar -czvf pq-bin.tar.gz pq;\
		cd -;\
		mv target/x86_64-unknown-linux-musl/release/pq-bin.tar.gz ./pq-bin.tar.gz 

.PHONY: all build-debug build-release docs lint package
