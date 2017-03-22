#![crate_type = "bin"]

extern crate protobuf;
extern crate rustc_serialize;
extern crate docopt;

mod protob;
mod unknown;

use docopt::Docopt;

use std::io::{self, Read, BufReader};
use std::fs::File;

use protob::{process_single, process_stream};

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
            let mut read = stdin.lock();
            f(&mut read);
        },
        _ => { 
            let file = match File::open(filepath) {
                Ok(x) => x,
                Err(e) => panic!(e),
            };
            let mut read = BufReader::new(file);
            f(&mut read);
        }
    }
}
