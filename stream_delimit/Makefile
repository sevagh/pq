CARGO_FLAGS=$(CARGO_FLAGS)

lint:
	rustup default stable && cargo fmt -- --write-mode=diff $(CARGO_FLAGS);\
	rustup default nightly && cargo clippy $(CARGO_FLAGS);\
	rustup default stable ;\

.PHONY: all build-debug build-release docs package
