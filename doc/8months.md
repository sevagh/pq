8 months of pq
==============

[2017-10-30] A post announcing the 1.0 release of pq
----------------------------------------------------

8 months ago, I embarked on a challenging project - a command-line protobuf parser - and I chose to write it in Rust. This is a blog post celebrating the 1.0 release of [pq](https://github.com/sevagh/pq).

The name pq is inspired by [jq](https://stedolan.github.io/jq/), the well-known json tool.

I chose Rust because we had a half-working implementation written in C++ lying around (written by a coworker years ago) and getting it to compile was a nightmare.

### Background - what is protobuf?

Protobuf, a.k.a. [Google Protocol Buffers](https://developers.google.com/protocol-buffers/) is a method for serializing data into binary streams compactly. Google describe it better:

>Protocol buffers are Google's language-neutral, platform-neutral, extensible mechanism for serializing structured data – think XML, but smaller, faster, and simpler.

Here's a basic example. Let's write a file called `dog.proto` containing:

```java
package com.example.dog;

message Dog {
  required string breed = 1;
  required int32 age = 2;
  optional string temperament = 3;
}
```

After compiling this to a language-specific file (e.g. in Python), you get a file like `dog_pb2.py` from which you can import and instantiate `Dog` instances:

```python
my_dog = dog_pb2.Dog(age=5, breed='rottweiler', temperament='excited')
sys.stdout.buffer.write(my_dog.SerializeToString())
```

The output will seem like gibberish because it must be parsed with the same `com.example.dog.Dog` message descriptor defined above:

```text
gsdexcited
```

Now, given that we still have a copy of `dog_pb2.py`, we can easily parse this back to the original dog object.

### The problem - different message descriptors

What if I told you that you could be receiving hundreds of different types of protobuf messages with different schemas, and you need a way to eyeball these as they pass through your infrastructure (Kafka, SSH streaming, whatever)?

The naive option is:

1. Discuss which message is arriving with the person/engineering team sending it ("Oh yeah, these are com.example.dog.Dog messages, here's the proto file")
2. Compile that specific proto file to your language of choice and write custom logic to print the fields

This is too much labor. Ideally, you could have some compiled binary that is able to take in a list of non-language-specific `.proto` files, take a stream of incoming data, and apply each deserialization until it works.

Unfortunately, this doesn't work great with protobuf, since the fields are numbered and not named. For example, the following `com.example.person.Person` message descriptor is practically interchangeable with the above `com.example.dog.Dog` descriptor:

```java
package com.example.person;

message Person {
  required string name = 1;
  required int32 id = 2;
}
```
Dog has an extra field if you recall, but dogs can be parsed as people and the third field is just dropped.

### Message guessing

[Early versions of pq](https://github.com/sevagh/pq/commit/98a4e470e2ef8229e03b03b853f4c48e5836d8b1) supported the `guess` feature. The blurb taken from the README:

```markdown
**Guessing strategy:**
 * For every message type discovered in `~/.pq/*.fdset`, try to decode the message with it
 * If the decode attempt has an error, skip this type
 * If any fields are empty/null (`serde_value::Value::Unit` in the codebase), skip this type
 * If the decode is successful, store the decoded `BTreeMap` in a vector
 * Display the element from the vector which has the most fields
```

This is hacky since not all fields are required. Therefore, you could have a dog with an unspecified third-field, which kinda looks exactly like a person (which has no third field to begin with). I chose to deprecate the guess feature.

### The final prototype of pq

`pq` finally looked like this:

1. Load all the message descriptors from all the `.proto` files that you have
2. Feed the stream to pq
3. Specify the message type you're expecting so that pq will use the correct message descriptor

### The early days of pq - discovering serde-protobuf

A huge [turning point](https://github.com/sevagh/pq/commit/c8cfbaa0b04bf0240f393d71a9745f632a579320) for pq actually working (instead of causing me to tear my hair out) was when I discovered [serde-protobuf](https://crates.io/crates/serde-protobuf). This is a library which wraps the low level [rust-protobuf](https://github.com/stepancheg/rust-protobuf) library.

Here's some funny code at this point:

```rust
pub fn process_stream(read: &mut Read) {
    let mut stream = protobuf::stream::CodedInputStream::new(read);

    loop {
        match stream.eof() {
            Err(e) => panic!(e),
            Ok(true) => break,
            Ok(false) => break, 
            //todo: actually do deserialization here
        }
    }
}
```

TODO: everything.

### Using fdsets instead of proto files

Earlier I mentioned loading message descriptors from `.proto` files; I lied.

There's a sort of intermediate, non-language-specific compiled form of protobuf `.proto` files called [FileDescriptorSets](https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/DescriptorProtos.FileDescriptorSet?csw=1).

You can produce these using `protoc -o person.fdset person.proto`. Now, `.fdset` files are usable with serde-protobuf.

Great [commit message](https://github.com/sevagh/pq/commit/9d10d70c7acef6fad34f5d53c92a2a4aa58bac4b) here: "apparently it's impossible to decode without fdsets".

### Switching to musl

In the early days I had no concrete plans for the distribution of pq. As it turns out, `scp`ing stuff from your laptop is not clever. I settled on musl to create static binaries that could (hypothetically) work anywhere.

The [commit](https://github.com/sevagh/pq/commit/c06dffa28a2ea253954ebe4bde8ad610846c72b9) actually installed a musl Rust toolchain. Eventually I discovered the [clux/muslrust](https://github.com/clux/muslrust) Docker container and [switched to using it for pq](https://github.com/sevagh/pq/commit/1f55ea9d00131a60e4c337ff7e8171e6bfbb4fdf#diff-b67911656ef5d18c4ae36cb6741b7965) buried deep in a commit that doesn't mention it at all.

### Adding support for Kafka

Protobuf's native streaming mechanisms are [length-delimited](https://developers.google.com/protocol-buffers/docs/encoding). However, we receive protobuf messages in Kafka where each compiled binary message is encapsulated in Kafka messages.

I added [Kafka](https://github.com/sevagh/pq/commit/1f55ea9d00131a60e4c337ff7e8171e6bfbb4fdf) support in a large commit. I was initially using [rdkafka](https://github.com/fede1024/rust-rdkafka), but I realized I didn't need the fine-grained consumer options and switched to [kafka-rust](https://github.com/spicavigo/kafka-rust) in [this commit](https://github.com/sevagh/pq/commit/f629ce04e2a9d91b6269b085d1b23efca0b8e718).

### Switching from docopt to clap

At the time, I made the choice because of the [rustc-serialize deprecation announcement](https://www.reddit.com/r/rust/comments/66rl8p/official_deprecation_of_rustcserialize_in_favor/) and the fact that [clap](https://github.com/kbknapp/clap-rs) had already switched to serde while [docopt.rs](https://github.com/docopt/docopt.rs) was still waiting for it (yes, I know, open-source so I could have done it but at the time I did not have the skill required to write that PR).

### Switching to error-chain

I was able to [save some LOC](https://github.com/sevagh/pq/commit/a12bdbfd5a106cfe563f58ce83ba082c7bfdb052) by switching to [error chain](https://github.com/rust-lang-nursery/error-chain).

### Writing a type-erased trait

[Full article on this](https://sevagh.github.io/post/erase/). I had an ugly bit of code:

```rust
if is_tty {
   let formatter = NewlineFormatter::default();
   Ok(value
       .serialize(&mut Serializer::with_formatter(out, formatter))
       .chain_err(|| "Ser error")?)
else {
   Ok(value.serialize(&mut Serializer::new(out)).chain_err(
       || "Serr error",
   )?)
}
```

The reason I call it ugly is that `Serializer::new()` uses the serde_json [CompactFormatter](https://docs.serde.rs/serde_json/ser/struct.CompactFormatter.html) trait, while my output for a tty used my own `PrettyFormatter` trait. Both of these impl the Formatter trait, which is [not object-safe](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md).

The full article is worth a read but I did this to gain a small visual win:

```rust
Ok(value
    .serialize(&mut Serializer::with_formatter(
	out,
	CustomFormatter::new(is_tty),
    ))
    .chain_err(|| "Ser error")?)
 ```
[Here's the commit](https://github.com/sevagh/pq/commit/5228d7f76d05c831217f6f3865555ad4319144d2).


### Using assert_cli for tests

I initially had my own hand-spun wrapper around std::process for testing pq, inspired heavily by [xsv's test suite](https://github.com/BurntSushi/xsv). I discovered [assert_cli](https://github.com/killercup/assert_cli) recently and switched to using it in [this commit](https://github.com/sevagh/pq/commit/66d0d8bd3a56dd636a87dafe484c6ea40e28a450).

I also made a small contribution to assert_cli. Friendly bunch of maintainers.

### Conclusion

This article is a bit rambly but here are the chief takeaways:

* Writing Rust is hard. Please [dive into my commits and see how the code evolved over time](https://github.com/sevagh/pq/commits/master?after=fab2658a435af34a2da8cd617fdba9bd72446d6e+244)
* The Rust ecosystem is rich. Every time I discovered a new crate, it added huge improvements to pq
* Using the combination of clap, error_chain, and assert_cli, one can write a well-tested, well-documented binary crate easily in Rust

Unfortunately, what I'm missing the most is benchmarks, but that requires me to write non-Rust protobuf parsers first so I'm not holding my breath on having the time to do that just yet.
