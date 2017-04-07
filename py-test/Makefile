all: py

py:
	protoc --python_out . *.proto

fdset:
	protoc -o dog.fdset dog.proto
	protoc -o person.fdset person.proto

rs:
	protoc --rust_out . *.proto

copy:
	mkdir -p ~/.pq/
	cp *.fdset ~/.pq/

.PHONY: clean
clean:
	-rm -rf ./*.fdset ./*_pb2.py ./*_pb3.py ./*.rs
