#![crate_type = "bin"]

extern crate rustc_serialize;
extern crate docopt;
extern crate protobuf;
extern crate serde;
extern crate serde_protobuf;
extern crate serde_value;
extern crate serde_json;

mod error;
mod discovery;
mod protob;
mod stream;

use discovery::discover_fdsets;
use docopt::Docopt;
use error::PqrsError;
use protob::PqrsDecoder;
use stream::decode_leading_varint;
use std::fs::File;
use std::io::{self, Write, Read, BufReader};
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
pq - Protobuf to json

Usage:
  pq [--msgtype=<msgtype>] [--fdsets=<path>] [<infile>] [--stream]
  pq (--help | --version)

Options:
  --stream              Varint size-delimited stream
  --msgtype=<msgtype>   Message type e.g. com.example.Type
  --fdsets=<path>       Alternative path to fdsets
  --help                Show this screen.
  --version             Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub arg_infile: Option<String>,
    pub flag_msgtype: Option<String>,
    pub flag_fdsets: Option<String>,
    pub flag_stream: bool,
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

    let mut buf = match args.flag_stream {
        false => {
            let mut buf = Vec::new();
            match infile.read_to_end(&mut buf) {
                Ok(_) => (),
                Err(_) => {
                    writeln!(&mut stderr, "Could not real file to end").unwrap();
                    process::exit(-1);
                }
            }
            buf
        }
        true => {
            let mut next_proto_size = 0;
            let mut buf = Vec::new();
            while !decode_leading_varint(&buf, &mut next_proto_size).is_ok() {
                let mut tmpbuf = vec![0; 1];
                infile.read_exact(&mut tmpbuf);
                buf.append(&mut tmpbuf);
            }
            println!("RESULTING SIZE: {:#?}", next_proto_size);
            process::exit(0);
        }
    };

    let fdsets = match discover_fdsets(args.flag_fdsets) {
        Ok(x) => x,
        Err(PqrsError::InitError(_)) |
        Err(PqrsError::EmptyFdsetError()) => {
            writeln!(&mut stderr, "Could not find fdsets").unwrap();
            process::exit(-1);
        }
        Err(e) => panic!(e),
    };

    let pqrs_decoder = PqrsDecoder::new(&args.flag_msgtype, &fdsets).unwrap();
    forcefully_decode(&pqrs_decoder, &buf, &mut stdout.lock()).unwrap();
}

fn forcefully_decode(pqrs_decoder: &PqrsDecoder,
                     buf: &[u8],
                     mut out: &mut Write)
                     -> Result<(), PqrsError> {
    let mut offset = 0;
    let buflen = buf.len();
    while offset < buflen {
        for n in 0..offset + 1 {
            if pqrs_decoder
                   .decode_message(&buf[n..(buflen - offset + n)], &mut out)
                   .is_ok() {
                return Ok(());
            }
        }
        offset += 1;
    }
    Err(PqrsError::CouldNotDecodeError())
}
