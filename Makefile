FDSET := $(patsubst %.proto,%.fdset,$(wildcard proto/*.proto))

all: clean $(FDSET) cargo

$(FDSET): %.fdset: %.proto
	protoc $^ -o $@

cargo:
	cargo build

.PHONY: clean

clean:
	-rm -rf proto/*.fdset
	#hack until i get build script to listen on proto/ dir
	-rm -rf src/proto/*.rs
	-rm -rf src/protob.rs
