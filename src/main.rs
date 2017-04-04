#![crate_type = "bin"]

extern crate rustc_serialize;
extern crate docopt;
extern crate protobuf;
extern crate serde;
extern crate serde_protobuf;
extern crate serde_value;
extern crate serde_json;

mod protob;

use docopt::Docopt;
use protob::{named_message, guess_message};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::fs::File;

const USAGE: &'static str = "
pq - Protobuf to json

Usage:
  pq [<filepath>] [--type=<string>] [-o=<filepath>]
  pq (-h | --help)
  pq --version

Options:
  --type=<msg_type>     Message type e.g. com.example.Type 
  -o, --outfile         Output file path
  -h --help             Show this screen.
  --version             Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub filepath: String,
}

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    let filepath = args.get_str("<filepath>");
    let msg_type = args.get_str("--type");
    let outfile = args.get_str("-o");

    let buf = match filepath {
        "" => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf).unwrap();
            buf
        },
        _ => { 
            let mut buf = Vec::new();
            let file = File::open(filepath).expect("Could not open file!");
            let mut file_read = BufReader::new(file);
            file_read.read_to_end(&mut buf).unwrap();
            buf
        }
    };

    let stdout_ = io::stdout();
    let mut stdout = stdout_.lock();
    let file: File;
    let mut file_write: BufWriter<File>;

    let mut write = match outfile {
        "" => &mut stdout as &mut Write,
        _ => {
            file = File::create(outfile).expect("Could not create out file!");
            file_write = BufWriter::new(file);
            &mut file_write as &mut Write
        }
    };

    match msg_type {
        "" => {
            match guess_message(&buf, &mut write) {
                Ok(_) => return,
                Err(e) => panic!(e),
            }
        },
        _ => {
            match named_message(&buf, msg_type, &mut write) {
                Ok(_) => return,
                Err(e) => panic!(e),
            }
        },
    }
}
