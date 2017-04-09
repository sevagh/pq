# pqrs [![Travis](https://img.shields.io/travis/sevagh/pqrs.svg)](https://travis-ci.org/sevagh/pqrs) [![GitHub release](https://img.shields.io/github/release/sevagh/pqrs.svg)](https://github.com/sevagh/pqrs/releases) [![GitHub tag](https://img.shields.io/github/tag/sevagh/pqrs.svg)](https://github.com/sevagh/pqrs/releases) [![license](https://img.shields.io/github/license/sevagh/pqrs.svg)](https://github.com/sevagh/pqrs/blob/master/LICENSE)
### protobuf to json deserializer, written in Rust

`pqrs` is a tool which deserializes compiled protobuf messages given a set of pre-compiled `.fdset` files.

### Now on crates.io [![Crates.io](https://img.shields.io/crates/d/pq.svg)](https://crates.io/crates/pq) [![Crates.io](https://img.shields.io/crates/v/pq.svg)](https://crates.io/crates/pq)

### Contents
1. [Usage](#usage)
2. [Forced decoding](#forced-decoding)
3. [Message guessing](#message-guessing)
4. [Portability with musl](#portability-with-musl)
5. [Dependencies](#dependencies)
6. [Tests](#tests)
7. [Goals](#goals)
8. [Todo](#todo)

### Usage

1. Put your `*.fdset` files in `~/.pq`:

```
$ protoc -o dog.fdset dog.proto
$ protoc -o person.fdset person.proto
$ cp *.fdset ~/.pq/
```

2. Pipe a single compiled protobuf message to pq:

```
sevag:pqrs $ ./py-test/generate_random_proto.py | pq | jq
{
  "age": 4,
  "breed": "poodle",
  "temperament": "excited"
}
sevag:pqrs $ ./py-test/generate_random_proto.py | pq | jq
{
  "id": 2,
  "name": "raffi"
}
```

[Read the manpage](https://sevagh.github.io/pqrs/).

### Forced decoding

The default behavior of `pqrs` is to try to brute-force decode a message. Given a 20-byte message:

```
while (don't have decode result)
    if (decode [0:19]) == success: return

    # try chopping off 1 byte
    if (decode([0:18]) == success or\
        decode([1:19]) == success): return

    # try chopping off 2 consecutive bytes
    if (decode([0:17]) == success or\
        decode([1:18]) == success) or\
        decode([2:19]) == success): return

    # repeat until success or no bytes left
```

I thought of making this option toggleable, but in practise it's rare to have a pristine proto message. It could have a leading varint, a trailing EOF, anything. Therefore, I'd prefer `pqrs` to be robust by default.

This will lead to terrible performance for any protos that are off by more than 2 bytes.

### Message guessing

`pqrs` by default will guess the message type. You can make it use a specific type by passing the fully qualified message name, e.g. `pq --msgtype="com.example.dog.Dog"`.

**Guessing strategy:**

* For every message type discovered in `~/.pq/*.fdset`, try to decode the message with it
* If the decode attempt has an error, skip this type
* If any fields are empty/null (`serde_value::Value::Unit` in the codebase), discard the decoded result
* If the decode is successful, store the decoded `BTreeMap` in a vector
* Display the `BTreeMap` from the vector which has the most fields

Since protobuf treats fields as positional, similar protos (e.g. Dog: <Int: age, String: breed>, Person: <Int: ssn, String: name>) are indistinguishable.

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

### Dependencies

```
[dependencies]
docopt = "0.7"
rustc-serialize = "0.3"
serde = "0.9.12"
serde-value = "0.4.0"
serde_json = "0.9.9"
serde-protobuf = "0.5"
protobuf = "1.2.1"
```

### Tests

The testing tools are [./py-test](./py-test) for a Python random compiled protobuf generator ([py-test README](./py-test/README.md)), and [./tests](./tests) for Rust integration tests. The integration tests invoke the `pqrs` binary using `std::process` and checks return codes, stdout, etc. - inspired by [the xsv test suite](https://github.com/BurntSushi/xsv/tree/master/tests).

There is no linting in the Travis-CI job because it takes too long, but there is a make target (`make lint`). This is a bit hacky - it switches to rust-stable to run `cargo fmt`, rust-nightly to run `cargo clippy` (and then back to rust-stable). Run this before submitting a PR, or alternatively, run `cargo fmt` and `cargo clippy` however you prefer.

### Goals

The original goal was to make a UNIX-y tool for generalized protobuf pretty-printing. Since `jq` already exists, I dropped the pretty-printing requirement and just output ugly JSON.

A new goal is handling a stream of protobuf data.

### Todo

* Proper testing. CI with `py-test/`, Rust tests, etc.
* Figure out how to handle streams (delimiters, etc.?)
* Release on `crates.io`
* Host static binary on github releases for download
