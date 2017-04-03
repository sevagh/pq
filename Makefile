CARGO = rust/bin/cargo
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

.PHONY: all message build test clean distclean
