#![crate_type = "bin"]

extern crate rustc_serialize;
extern crate docopt;
extern crate protobuf;
extern crate serde;
extern crate serde_protobuf;
extern crate serde_value;
extern crate serde_json;
extern crate stream_delimit;

mod fdset_discovery;
mod error;
mod decode;
#[macro_use]
mod macros;

use docopt::Docopt;
use decode::PqrsDecoder;
use stream_delimit::{StreamDelimiter, Parse};
use std::fs::File;
use std::io::{self, Read, BufReader, Write, stderr};
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
pq - Protobuf to json

Usage:
  pq [--msgtype=<msgtype>] [<infile>] [--stream]
  pq (--help | --version)

Options:
  --stream              Varint size-delimited stream
  --msgtype=<msgtype>   Message type e.g. com.example.Type
  --help                Show this screen.
  --version             Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub arg_infile: Option<String>,
    pub flag_msgtype: Option<String>,
    pub flag_stream: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(String::from(VERSION))).decode())
        .unwrap_or_else(|e| e.exit());

    let stdin = io::stdin();
    let stdout = io::stdout();

    let pqrs_decoder = match PqrsDecoder::new(args.flag_msgtype) {
        Ok(x) => x,
        Err(e) => errexit!(e),
    };

    let mut infile: Box<Read> = match args.arg_infile {
        Some(x) => {
            let file = match File::open(&x) {
                Ok(x) => x,
                Err(e) => errexit!(e),
            };
            Box::new(BufReader::new(file))
        }
        None => Box::new(stdin.lock()),
    };

    if !args.flag_stream {
        let mut buf = Vec::new();
        match infile.read_to_end(&mut buf) {
            Ok(_) => (),
            Err(e) => errexit!(e),
        }
        match pqrs_decoder.decode_message(&buf, &mut stdout.lock()) {
            Ok(_) => (),
            Err(e) => errexit!(e),
        }
    } else {
        let mut delim = StreamDelimiter::Varint(16);
        let mut msg_size: usize = 0;
        loop {
            delim.parse(&mut infile, &mut msg_size).unwrap();
            let mut msg_buf = vec![0; msg_size as usize];
            infile.read_exact(&mut msg_buf).unwrap();
            match pqrs_decoder.decode_message(&msg_buf, &mut stdout.lock()) {
                Ok(_) => (),
                Err(e) => errexit!(e),
            }
        }
    }
}
