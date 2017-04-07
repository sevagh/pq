CARGO = $(PWD)/rust/bin/cargo
export RUSTC = $(PWD)/rust/bin/rustc
TARGET = x86_64-unknown-linux-musl
CARGO_FLAGS += --target=$(TARGET)

RUSTUP_URL = https://static.rust-lang.org/rustup.sh

include $(CONFIG)

all: build

cargo: rust/bin/cargo

build: cargo
	$(CARGO) build $(CARGO_FLAGS)

clean:
	$(CARGO) clean $(CARGO_FLAGS)

distclean: clean
	-rm -rf rust

rust/rustup.sh:
	mkdir -p rust
	curl -sSf -o $@ $(RUSTUP_URL)
	chmod +x $@

rust/bin/cargo: rust/rustup.sh
	$< --disable-sudo --disable-ldconfig --yes --prefix=rust \
		--with-target=$(TARGET)

docs:
	mandoc -Thtml pqrs.1 >docs/index.html

lint:
	rustup default stable && cargo fmt -- --write-mode=diff
	rustup default nightly && cargo clippy
	rustup default stable

package: build
	cd target/$(TARGET)/debug;\
		tar -czvf pq-bin.tar.gz pq;\
		cd -;\
		mv target/$(TARGET)/debug/pq-bin.tar.gz ./pq-bin.tar.gz 

.PHONY: all message build test clean distclean docs
