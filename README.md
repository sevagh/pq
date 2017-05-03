# pqrs [![Travis](https://img.shields.io/travis/sevagh/pqrs.svg)](https://travis-ci.org/sevagh/pqrs) [![license](https://img.shields.io/github/license/sevagh/pqrs.svg)](https://github.com/sevagh/pqrs/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/pq.svg)](https://crates.io/crates/pq)

### protobuf to json deserializer, written in Rust

`pqrs` is a tool which deserializes compiled protobuf messages given a set of pre-compiled `.fdset` files.

### Download

pqrs is on [crates.io](https://crates.io/crates/pq): `cargo install pq`. You can also download a static binary from the [releases page](https://github.com/sevagh/pqrs/releases).

### Usage

**Read the [manpage!](https://sevagh.github.io/pqrs/)**

To set up, put your `*.fdset` files in `~/.pq`:

```
$ protoc -o dog.fdset dog.proto
$ protoc -o person.fdset person.proto
$ cp *.fdset ~/.pq/
```

Pipe a single compiled protobuf message:

```
$ ./tests/python/testbench.py "single()" | pq | jq
{
  "age": 4,
  "breed": "poodle",
  "temperament": "excited"
}
```

Pipe a dirty (extra leading/trailing chars):

```
$ (printf hello && ./tests/python/testbench.py "single()") | pq | jq
{
  "id": 1,
  "name": "vahaken"
}
```

Pipe a `varint`-delimited stream:

```
$ ./tests/python/testbench.py "stream(limit=2)" | pq --stream="varint" | jq
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
$ ./tests/python/testbench.py "trail(trail=b'\n',limit=2)" | ./target/debug/pq --stream=varint --trail=1 | jq
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
