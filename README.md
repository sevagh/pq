# pqrs [![Travis](https://img.shields.io/travis/sevagh/pq.svg)](https://travis-ci.org/sevagh/pq) [![license](https://img.shields.io/github/license/sevagh/pq.svg)](https://github.com/sevagh/pq/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/pq.svg)](https://crates.io/crates/pq)

### protobuf to json deserializer, written in Rust

`pq` is a tool which deserializes compiled protobuf messages given a set of pre-compiled `.fdset` files.

### Download

pq is on [crates.io](https://crates.io/crates/pq): `cargo install pq`. You can also download a static binary from the [releases page](https://github.com/sevagh/pq/releases).

### Usage

**Read the [manpage!](https://sevagh.github.io/pq/)**

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

Pipe a `varint`-delimited stream with trailing newlines:

```
$ testbench.py "trail(trail=b'\n',limit=2)" | pq --stream=varint --trail=1 | jq
{
  "age": 16,
  "breed": "gsd",
  "temperament": "chill"
}
{
  "id": 3,
  "name": "raffi"
}
```
