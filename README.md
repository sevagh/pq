# pqrs
### protobuf to json deserializer, written in Rust

`pqrs` is a tool which deserializes compiled protobuf messages given a set of pre-compiled `.fdset` files.

### Usage

1. Have your `.proto` files:

```
$ head py-test/dog.proto -n3
package com.example.dog;

message Dog {
$
$ head py-test/person.proto -n3
package com.example.person;

message Person {
```

2. Compile them into `.fdset` files:

```
$ protoc -o dog.fdset dog.proto
$ protoc -o person.fdset person.proto
```

3. Copy the `.fdset` files into `~/.pq`:

```
$ ls ~/.pq/
dog.fdset person.fdet
```

4. Pipe a single compiled protobuf message to pq:

```
sevag:pqrs $ ./py-test/generate_random_proto.py | pq --type="com.example.dog.Dog" | jq
{
  "age": 4,
  "breed": "poodle"
}
```

`pqrs` operates on stdin/stdout by default but also works with files.

* Pass the input file as the first positional argument:

`pq --type="com.example.dog.Dog" /path/to/input.bin`

* Output to a file instead of `stdout`:

`pq --type="com.example.dog.Dog" -o /path/to/output.json`

### Goal

The goal was to make a UNIX-y tool for generalized protobuf pretty-printing. Since `jq` already exists, I dropped the pretty-printing requirement and just output ugly JSON.

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

### Todo

* Proper testing. CI with `py-test/`, Rust tests, etc.
* Figure out how to handle streams (delimiters, etc.?)
