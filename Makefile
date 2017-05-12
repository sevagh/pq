WORKSPACES = "./" "./stream_delimit/"

all: build

docker:
	docker pull clux/muslrust

build-debug: docker
	docker run -v $(PWD):/volume:Z -e USERID=1000 -w /volume -t clux/muslrust cargo build

build-release: docker
	docker run -v $(PWD):/volume:Z -e USERID=1000 -w /volume -t clux/muslrust cargo build --release

test: docker
	docker run -v $(PWD):/volume:Z -e USERID=1000 -w /volume -t clux/muslrust cargo test --verbose
	
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
