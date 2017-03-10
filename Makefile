FDSET := $(patsubst %.proto,%.fdset,$(wildcard proto/*.proto))

all: $(FDSET) cargo

$(FDSET): %.fdset: %.proto
	protoc $^ -o $@

cargo:
	cargo build

.PHONY: clean

clean:
	-rm -rf proto/*.fdset
	-cargo clean
