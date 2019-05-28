pq - advanced usage
===================

To set up, put your `*.fdset` files in `~/.pq` or `/etc/pq` or an alternate directory specified with the `FDSET_PATH=` env var:

```
$ protoc -o dog.fdset dog.proto
$ protoc -o person.fdset person.proto
$ cp *.fdset ~/.pq/
```

You can specify additional fdset directories or files via options:

```
$ pq --msgtype com.example.dog.Dog --fdsetdir ./tests/fdsets <./tests/samples/dog
$ pq --msgtype com.example.dog.Dog --fdsetfile ./tests/fdsets/dog.fdset <./tests/samples/dog
```

Pipe a single compiled protobuf message:

```
$ pq --msgtype com.example.dog.Dog <./tests/samples/dog
{
  "age": 4,
  "breed": "poodle",
  "temperament": "excited"
}
```

Pipe a `varint` or `leb128` delimited stream:

```
$ pq --msgtype com.example.dog.Dog --stream varint <./tests/samples/dog_stream
{
  "age": 10,
  "breed": "gsd",
  "temperament": "aggressive"
}
```

Consume from a Kafka stream:

```
$ pq kafka my_topic --brokers 192.168.0.1:9092 --beginning --count 1 --msgtype com.example.dog.Dog
{
  "age": 10,
  "breed": "gsd",
  "temperament": "aggressive"
}
```

Convert a Kafka stream to varint-delimited:

```
$ pq kafka my_topic --brokers=192.168.0.1:9092 --count 1 --convert varint |\
> pq --msgtype com.example.dog.Dog --stream varint
{
  "age": 10,
  "breed": "gsd",
  "temperament": "aggressive"
}
```

Pipe `kafkacat` output to it:
```
$ kafkacat -b 192.168.0.1:9092 -C -u -q -f "%R%s" -t my_topic |\
> pq --msgtype=com.example.dog.Dog --stream i32be
{
  "age": 10,
  "breed": "gsd",
  "temperament": "aggressive"
}
```

### Compile without kafka

To compile `pq` without kafka support, run:

```
$ cargo build --no-default-features
```
