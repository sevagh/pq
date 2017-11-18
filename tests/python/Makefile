all: py

py:
	protoc --python_out . ../schemata/*.proto -I../schemata

.PHONY: clean
clean:
	-rm -rf ./*.fdset ./*_pb2.py ./*_pb3.py __pycache__
