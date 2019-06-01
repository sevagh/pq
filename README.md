# pq [![license](https://img.shields.io/github/license/sevagh/pq.svg)](https://github.com/sevagh/pq/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/pq.svg)](https://crates.io/crates/pq)

### protobuf to json deserializer, written in Rust

`pq` is a tool which deserializes protobuf messages given a set of pre-compiled `.fdset` files. It can understand varint/leb128-delimited/i32be streams, and it can connect to Kafka.

`pq` will pretty-print when outputting to a tty, but you should pipe it to `jq` for more fully-featured json handling.

### Download

pq is on [crates.io](https://crates.io/crates/pq): `cargo install pq`. You can also download a static binary from the [releases page](https://github.com/sevagh/pq/releases).

### Quick start

Put your `*.fdset` files in `~/.pq`:

```
$ protoc -o dog.fdset dog.proto
$ protoc -o person.fdset person.proto
$ cp *.fdset ~/.pq/
```

Decode a single protobuf message:

```
$ pq --msgtype com.example.dog.Dog <./tests/samples/dog
{
  "age": 4,
  "breed": "poodle",
  "temperament": "excited"
}
```

Decode a `varint`-delimited stream:

```
$ pq --msgtype com.example.dog.Dog --stream varint <./tests/samples/dog_stream
{
  "age": 10,
  "breed": "gsd",
  "temperament": "aggressive"
}
```

[More usage](./USAGE.md)

### Compiling for Windows

1. Install [Visual Studio Installer](https://visualstudio.microsoft.com/downloads/) Community edition
2. Run the installer and install `Visual Studio Build Tools 2019`. You need the `C++ Build Tools` workload. Note that you can't just install the minimal package, you also need `MSVC C++ x64/86 Build Tools` and `Windows 10 SDK`.
3. Open `x64 Native Tools Command Prompt` from the start menu.
4. Download and run [`rustup-init.exe`](https://win.rustup.rs/x86_64)
5. Close and reopen your terminal (so `cargo` will be in your path)
6. Run `cargo install --no-default-features pq`

Note that this will disable the Kafka feature. Kafka is not currently supported on Windows.
