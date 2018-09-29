# pq [![license](https://img.shields.io/github/license/sevagh/pq.svg)](https://github.com/sevagh/pq/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/pq.svg)](https://crates.io/crates/pq)

### protobuf to json deserializer, written in Rust

`pq` is a tool which deserializes protobuf messages given a set of pre-compiled `.fdset` files. It can understand varint/leb128-delimited streams, and it can connect to Kafka.

`pq` will pretty-print when outputting to a tty, but you should pipe it to `jq` for more fully-featured json handling.

### Download

pq is on [crates.io](https://crates.io/crates/pq): `cargo install pq`. You can also download a static binary from the [releases page](https://github.com/sevagh/pq/releases).

### Quick start

Put your `*.fdset` files in `~/.pq`:

```
$ protoc -o dog.fdset dog.proto
$ protoc -o person.fdset person.proto
$ cp *.fdset ~/.pq/
```

Decode a single protobuf message:

```
$ pq --msgtype com.example.dog.Dog <./tests/samples/dog
{
  "age": 4,
  "breed": "poodle",
  "temperament": "excited"
}
```

Decode a `varint`-delimited stream:

```
$ pq --msgtype com.example.dog.Dog --stream varint <./tests/samples/dog_stream
{
  "age": 10,
  "breed": "gsd",
  "temperament": "aggressive"
}
```

[More usage](./USAGE.md)
