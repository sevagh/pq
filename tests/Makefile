PROTO_DIR="./schemata"
FDSET_DIR="./fdsets"
PROTO_FILES := $(shell ls $(PROTO_DIR)/*.proto)

all: fdset

fdset:
	$(foreach protofile, $(PROTO_FILES), protoc -o $(FDSET_DIR)/$(shell basename $(protofile) .proto).fdset $(protofile);)

regen:
	-rm -rf $(FDSET_DIR)/*
	$(MAKE) fdset

.PHONY: regen
