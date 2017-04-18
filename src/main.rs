#![crate_type = "bin"]

extern crate rustc_serialize;
extern crate docopt;
extern crate protobuf;
extern crate serde;
extern crate serde_protobuf;
extern crate serde_value;
extern crate serde_json;
extern crate stream_delimit;

mod error;
mod discovery;
mod decode;

use discovery::discover_fdsets;
use docopt::Docopt;
use error::PqrsError;
use decode::PqrsDecoder;
use stream_delimit::{StreamDelimiter, Parse};
use std::fs::File;
use std::io::{self, Write, Read, BufReader};
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
pq - Protobuf to json

Usage:
  pq [--msgtype=<msgtype>] [<infile>] [--stream] [--force]
  pq (--help | --version)

Options:
  --stream              Varint size-delimited stream
  --force               Force decode message
  --msgtype=<msgtype>   Message type e.g. com.example.Type
  --help                Show this screen.
  --version             Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub arg_infile: Option<String>,
    pub flag_msgtype: Option<String>,
    pub flag_stream: bool,
    pub flag_force: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(String::from(VERSION))).decode())
        .unwrap_or_else(|e| e.exit());

    let stdin = io::stdin();
    let stdout = io::stdout();
    let stderr = io::stderr();

    let mut stderr = stderr.lock();

    let fdsets = match discover_fdsets() {
        Ok(x) => x,
        Err(PqrsError::InitError(_)) |
        Err(PqrsError::EmptyFdsetError()) => {
            writeln!(&mut stderr, "Could not find fdsets").unwrap();
            process::exit(-1);
        }
        Err(e) => panic!(e),
    };

    let pqrs_decoder = PqrsDecoder::new(&args.flag_msgtype, &fdsets, args.flag_force).unwrap();

    let mut infile: Box<Read> = match args.arg_infile {
        Some(x) => {
            let file = match File::open(&x) {
                Ok(x) => x,
                Err(_) => {
                    writeln!(&mut stderr, "Could not open file: {}", x).unwrap();
                    process::exit(-1);
                }
            };
            Box::new(BufReader::new(file))
        }
        None => Box::new(stdin.lock()),
    };

    if !args.flag_stream {
        let mut buf = Vec::new();
        match infile.read_to_end(&mut buf) {
            Ok(_) => (),
            Err(_) => {
                writeln!(&mut stderr, "Could not real file to end").unwrap();
                process::exit(-1);
            }
        }
        pqrs_decoder
            .decode_message(&buf, &mut stdout.lock())
            .unwrap();
    } else {
        let mut delim = StreamDelimiter::Varint();
        let mut msg_size: usize = 0;
        loop {
            delim.parse(&mut infile, &mut msg_size).unwrap();
            let mut msg_buf = vec![0; msg_size as usize];
            infile.read_exact(&mut msg_buf).unwrap();
            pqrs_decoder
                .decode_message(&msg_buf, &mut stdout.lock())
                .unwrap();
        }
    }
}
