PROTOS := $(wildcard *.proto)
FDSETS := $(patsubst %.proto,%.fdset,$(PROTOS))

all: $(FDSETS)

py:
	@protoc --python_out . ./*.proto -I./

%.fdset: %.proto
	@protoc -o ./$@ ./$^ -I./

.PHONY: clean

clean:
	-rm -rf ./*.fdset ./*_pb2.py ./*_pb3.py __pycache__
