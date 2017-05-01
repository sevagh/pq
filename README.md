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
$ ./tests/python/testbench.py stream | pq --stream="varint" | jq
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

### Experimental feature - trailing delimiter

Sometimes protobuf is sent in ways that add their own delimiters. E.g. if you send varint-delimited Protobuf over kafka, to use `pqrs`, you would need to deal with the message delimiters inserted by `kafka-console-consumer.sh` or whatever your command-line kafka consumer is that you are piping to pq.

Tentative support for this is added in the form of a `--trail="\n\n"` command-line option.
