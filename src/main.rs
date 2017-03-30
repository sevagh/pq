#![crate_type = "bin"]

extern crate rustc_serialize;
extern crate docopt;
extern crate protobuf;

mod protob;
mod unknown;

use docopt::Docopt;
use protob::{process_single, process_stream};
use std::io::{self, Read, BufReader};
use std::fs::File;

const USAGE: &'static str = "
pq - Protobuf pretty-printer

Usage:
  pq [-s] [<filepath>]
  pq (-h | --help)
  pq --version

Options:
  -s, --single  Consume single message.
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub filepath: String,
}

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    let f: fn(&mut Read) = match args.get_bool("-s")  {
        true => process_single,
        false => process_stream,
    };

    let filepath = args.get_str("<filepath>");

    match filepath {
        "" => {
            let stdin = io::stdin();
            f(&mut stdin.lock());
        },
        _ => { 
            let file = File::open(filepath).expect("Could not open file!");
            f(&mut BufReader::new(file));
        }
    }
}
