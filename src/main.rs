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
mod force;
mod protob;

use discovery::discover_fdsets;
use docopt::Docopt;
use error::PqrsError;
use force::forcefully_decode;
use std::io::{self, Write, Read};
use std::process;

const USAGE: &'static str = "
pq - Protobuf to json

Usage:
  pq [--msgtype=<msgtype>] [--fdsets=<path>]
  pq (--help | --version)

Options:
  --msgtype=<msgtype>   Message type e.g. com.example.Type
  --fdsets=<path>       Alternative path to fdsets
  --help                Show this screen.
  --version             Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub flag_msgtype: Option<String>,
    pub flag_fdsets: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let stdin = io::stdin();
    let stdout = io::stdout();
    let stderr = io::stderr();

    let mut stderr = stderr.lock();

    let mut buf = Vec::new();
    match stdin.lock().read_to_end(&mut buf) {
        Ok(_) => (),
        Err(_) => {
            writeln!(&mut stderr, "Could not real file to end").unwrap();
            process::exit(-1);
        }
    }

    let fdsets = match discover_fdsets(args.flag_fdsets) {
        Ok(x) => x,
        Err(PqrsError::InitError(msg)) => {
            writeln!(&mut stderr, "Could not find fdsets: {}", msg).unwrap();
            process::exit(-1);
        }
        Err(PqrsError::EmptyFdsetError(msg)) => {
            writeln!(&mut stderr, "Could not find fdsets: {}", msg).unwrap();
            process::exit(-1);
        }
        Err(e) => panic!(e),
    };

    forcefully_decode(buf, args.flag_msgtype, &mut stdout.lock(), fdsets).unwrap();
}
