# pqrs [![GitHub release](https://img.shields.io/github/release/sevagh/pqrs.svg)](https://github.com/sevagh/pqrs/releases/tag/0.1.0) [![GitHub tag](https://img.shields.io/github/tag/sevagh/pqrs.svg)](https://github.com/sevagh/pqrs/tree/0.1.0) [![GitHub commits](https://img.shields.io/github/commits-since/sevagh/pqrs/0.1.0.svg)](https://github.com/sevagh/pqrs/commits/master)
### protobuf to json deserializer, written in Rust

`pqrs` is a tool which deserializes compiled protobuf messages given a set of pre-compiled `.fdset` files.

### Contents
1. [Usage](#usage)
    1. [Files](#files)
2. [Message guessing](#message-guessing)
3. [Portability with musl](#portability-with-musl)
4. [Dependencies](#dependencies)
5. [Goals](#goals)
6. [Todo](#todo)

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

#### Files

`pqrs` operates on stdin/stdout by default but also works with files:

* Pass the input file as the first positional argument: `pq /path/to/input.bin`
* Output to a file instead of stdout: `pq -o /path/to/output.json`

### Message guessing

`pqrs` by default will guess the message type. You can make it use a specific type by passing the fully qualified message name, e.g. `pq --type="com.example.dog.Dog"`.

**Guessing strategy:**

* For every message type discovered in `~/.pq/*.fdset`, try to decode the message with it
* If the decode attempt has an error, skip this type
* If any fields are empty/null (`serde_value::Value::Unit` in the codebase), skip this type
* If the decode is successful, store the decoded `BTreeMap` in a vector
* Display the element from the vector which has the most fields

So, deconstructing the above example for the null field case:

```
sevag:pqrs $ ./py-test/generate_random_proto.py | pq | jq
# this is a com.example.person.Person message
#
# first attempt: decode with com.example.dog.Dog:
# {"age": 4,"breed": "raffi","temperament": null}
#
# second attempt: decode with com.example.person.Person:
# {"id": 4,"name": "raffi"}
```

What happens here is that Dog and Person have similar definitions. A Dog ([see your yourself](./py-test)) is defined as `Age: Int, Breed: String, Temperament: String`, while a Person is `Id: Int, Name: String`.

Since protobuf treats fields as positional, the only thing that matters is that a Dog is `Int, String, String` and a Person is `Int, String`. However, since Dog has an extra third field which is decoded as null, `pqrs` decides that this message couldn't have been a Dog or else it would have had a non-null third string.

Result:

```
# pqrs guesses that this is a Person
{
  "id": 4,
  "name": "raffi"
}
```

Now for the other case, the winner by number of fields:

```
sevag:pqrs $ ./py-test/generate_random_proto.py | pq | jq
# this is a com.example.dog.Dog message
#
# first attempt: decode with com.example.dog.Dog:
# {"age": 4,"breed": "poodle","temperament": "excited"}
#
# second attempt: decode with com.example.person.Person:
# {"id": 4,"name": "poodle"}
```

In this case, there are no null fields. However, the Person-decoded `BTreeMap` was unable to extract the third string from the message because the Person type only has 2 fields. Dog, however, has `Int, String, String`, and therefore got more information from the message.

`pqrs` chooses Dog as the winner:

```
{
  "age": 4,
  "breed": "poodle",
  "temperament": "excited"
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

### Goals

The original goal was to make a UNIX-y tool for generalized protobuf pretty-printing. Since `jq` already exists, I dropped the pretty-printing requirement and just output ugly JSON.

A new goal is handling a stream of protobuf data.

### Todo

* Proper testing. CI with `py-test/`, Rust tests, etc.
* Figure out how to handle streams (delimiters, etc.?)
* Release on `crates.io`
* Host static binary on github releases for download
