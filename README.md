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

Pipe a single compiled protobuf message to pq:

```
$ ./tests/python/testbench.py single | pq | jq
{
  "age": 4,
  "breed": "poodle",
  "temperament": "excited"
}
$ ./tests/python/testbench.py single | pq | jq
{
  "id": 2,
  "name": "raffi"
}
```

Pipe a dirty (extra leading/trailing chars) to pq:

```
$ ./tests/python/testbench.py dirty | pq | jq
{
  "id": 1,
  "name": "vahaken"
}
```

Pipe a `varint32`-delimited stream to pq:

```
$ ./tests/python/testbench.py stream | pq --stream | jq
{
  "age": 10,
  "breed": "gsd",
  "temperament": "aggressive"
}
{
  "id": 3,
  "name": "khosrov"
}
{
  "age": 8,
  "breed": "rottweiler",
  "temperament": "aggressive"
}
{
  "id": 2,
  "name": "vahaken"
}
```
