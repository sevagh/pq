# pqrs [![Travis](https://img.shields.io/travis/sevagh/pq.svg)](https://travis-ci.org/sevagh/pq) [![license](https://img.shields.io/github/license/sevagh/pq.svg)](https://github.com/sevagh/pq/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/pq.svg)](https://crates.io/crates/pq) [![GitHub release](https://img.shields.io/github/release/sevagh/pq.svg)](https://github.com/sevagh/pq)

### protobuf to json deserializer, written in Rust

`pq` is a tool which deserializes compiled protobuf messages given a set of pre-compiled `.fdset` files.

### Download

pq is on [crates.io](https://crates.io/crates/pq): `cargo install pq`. You can also download a static binary from the [releases page](https://github.com/sevagh/pq/releases).

### Usage

```
pq - protobuf to json

Usage:
  pq <infile> [--msgtype=<msgtype>] [--stream=<delim>] [--count=<count>]
  pq kafka <topic> --brokers=<brokers> [--from-beginning] [--count=<count>]
  pq (--help | --version)

Options:
  --stream=<delim>      Stream delimiter e.g. "varint", "leb128"
  --msgtype=<msgtype>   Message type e.g. com.example.Type
  --brokers=<brokers>   1.2.3.4:9092,5.6.7.8:9092
  --from-beginning      Consume kafka from beginning
  --count=<count>       Stop after count messages
  --help                Show this screen.
  --version             Show version.
```

To set up, put your `*.fdset` files in `~/.pq`:

```
$ protoc -o dog.fdset dog.proto
$ protoc -o person.fdset person.proto
$ cp *.fdset ~/.pq/
```

Pipe a single compiled protobuf message:

```
$ testbench.py "single()" | pq | jq
{
  "age": 4,
  "breed": "poodle",
  "temperament": "excited"
}
```

Pipe a `varint`-delimited stream:

```
$ testbench.py "stream(limit=2)" | pq --stream="varint" | jq
{
  "age": 10,
  "breed": "gsd",
  "temperament": "aggressive"
}
{
  "id": 3,
  "name": "khosrov"
}
```

Consume from a Kafka stream:

```
$ pq kafka my_topic --brokers=192.168.0.1:9092 --from-beginning --count=1 | jq
{
  "age": 10,
  "breed": "gsd",
  "temperament": "aggressive"
}
```