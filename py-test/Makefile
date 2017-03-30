test:
	protoc --python_out . person.proto
	protoc --python_out . dog.proto
	protoc -o dog.fdset dog.proto
	protoc -o person.fdset person.proto
