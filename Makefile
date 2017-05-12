WORKSPACES = "./" "./stream_delimit/"

all: build

build-debug:
	docker pull clux/muslrust
	docker run -v $(PWD):/volume:Z -e USERID=1000 -w /volume -t clux/muslrust cargo build

build-release:
	docker pull clux/muslrust
	docker run -v $(PWD):/volume:Z -e USERID=1000 -w /volume -t clux/muslrust cargo build --release

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
