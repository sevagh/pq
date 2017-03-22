# pq-rs

## JQ for protobuf, written in Rust

This is heavily in the WIP stage. Both Rust and Protobuf are totally new to me.

## Goal

The goal is to make a UNIX-y tool for generalized protobuf pretty-printing:

```
$ /stream/of/compiled/protobuf | pq | jq
{
    "Name": "John Smith",
    "Profession": "Underwater Basket Weaver"
}
{
    "Name": "Alice Chang",
    "Profession": "Politician"
}
{
    "Name": "Wolfgang Mozart",
    "Profession": "Composer"
}
```

## Strategy

1. Create an empty protobuf object:

```
$ cat schemata/unknown.proto 
syntax = "proto3";

message Unknown {
}
```

2. Compile it to a `.rs` file with the [Rust protobuf](https://github.com/stepancheg/rust-protobuf) package:

```
$ protoc --rust_out ./src/ ./schemata/unknown.proto
$ cat src/unknown.rs
[...]
// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unknown {}

impl Unknown {
    pub fn new() -> Unknown {
        ::std::default::Default::default()
    }
[...]
```

3. Deserialize a stream of compiled protobuf message data from stdin with the `Unknown` object

4. [Optional?] Compare it to user-provided `.fdset` files to determine what type of message it is

5. Print the JSON representation
