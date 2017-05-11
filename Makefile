all: build

build:
	docker pull clux/muslrust
	docker run -v $(PWD):/volume:Z -e USERID=1000 -w /volume -t clux/muslrust cargo build --release

docs:
	mandoc -Thtml docs/_pqrs.1 >docs/index.html

lint:
	@- $(foreach WORKSPACE,$(WORKSPACES), \
		cd $(WORKSPACE) ;\
		rustup default stable && cargo fmt -- --write-mode=diff ;\
		rustup default nightly && cargo clippy ;\
		rustup default stable ;\
	)

package: build
	cd target/$(TARGET)/release;\
		tar -czvf pq-bin.tar.gz pq;\
		cd -;\
		mv target/$(TARGET)/release/pq-bin.tar.gz ./pq-bin.tar.gz 

.PHONY: all build docs lint package
