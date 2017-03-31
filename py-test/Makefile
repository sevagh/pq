test:
	protoc --python_out . person.proto
	protoc --python_out . dog.proto
	protoc -o dog.fdset dog.proto
	protoc -o person.fdset person.proto

copy:
	cp *.fdset ~/.pq/

.PHONY: clean
clean:
	-rm -rf ./*.fdset
