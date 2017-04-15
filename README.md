# pqrs [![Travis](https://img.shields.io/travis/sevagh/pqrs.svg)](https://travis-ci.org/sevagh/pqrs) [![GitHub release](https://img.shields.io/github/release/sevagh/pqrs.svg)](https://github.com/sevagh/pqrs/releases) [![GitHub tag](https://img.shields.io/github/tag/sevagh/pqrs.svg)](https://github.com/sevagh/pqrs/releases) [![license](https://img.shields.io/github/license/sevagh/pqrs.svg)](https://github.com/sevagh/pqrs/blob/master/LICENSE)
### protobuf to json deserializer, written in Rust

`pqrs` is a tool which deserializes compiled protobuf messages given a set of pre-compiled `.fdset` files.

### Now on crates.io [![Crates.io](https://img.shields.io/crates/d/pq.svg)](https://crates.io/crates/pq) [![Crates.io](https://img.shields.io/crates/v/pq.svg)](https://crates.io/crates/pq)

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
$ ./py-test/testbench.py single | pq | jq
{
  "age": 4,
  "breed": "poodle",
  "temperament": "excited"
}
$ ./py-test/testbench.py single | pq | jq
{
  "id": 2,
  "name": "raffi"
}
```

Pipe a dirty (extra leading/trailing chars) to pq:

```
$ ./py-test/testbench.py dirty | pq --force | jq
{
  "id": 1,
  "name": "vahaken"
}
```

Pipe a `varint32`-delimited stream to pq:

```
$ ./py-test/testbench.py stream | pq --stream | jq
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

### Portability with musl

First, clone and compile `musl-gcc` on your system:

```
$ git clone git://git.musl-libc.org/musl
$ ./configure && make && sudo make install
```

Then, run `make` in this repo - this downloads a local `./rust` toolchain with the `x86_64-unknown-linux-musl` target and runs `./rust/bin/cargo --target=x86_64-unknown-linux-musl` to build `pqrs`.

The result is a static binary:

```
$ ldd ./target/x86_64-unknown-linux-musl/debug/pq
        not a dynamic executable
$
$ file ./target/x86_64-unknown-linux-musl/debug/pq
./target/x86_64-unknown-linux-musl/debug/pq: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=3aa843efe79d0082aacb674a28e8d1ed8105a5e5, not stripped
```

Alternatively, you can download a static `musl`-compiled binary from the [releases page](https://github.com/sevagh/pqrs/releases).

### Tests

The testing tools are [./py-test](./py-test) for a Python random compiled protobuf generator ([py-test README](./py-test/README.md)), and [./tests](./tests) for Rust integration tests. The integration tests invoke the `pqrs` binary using `std::process` and checks return codes, stdout, etc. - inspired by [the xsv test suite](https://github.com/BurntSushi/xsv/tree/master/tests).

There is no linting in the Travis-CI job because it takes too long, but there is a make target (`make lint`). This runs `rustfmt` and `clippy`.
