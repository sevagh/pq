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
use std::boxed::Box;
use std::io::{self, BufWriter, Write, BufReader, Read};
use std::fs::File;

const USAGE: &'static str = "
pq - Protobuf to json

Usage:
  pq [<infile>] [--type=<string>] [-o=<outfile>]
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
    pub infile: String,
}

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    let infile = args.get_str("<infile>");
    let msg_type = args.get_str("--type");
    let outfile = args.get_str("-o");

    let stdin = io::stdin();
    let mut infile: Box<Read> = match infile {
        "" => Box::new(stdin.lock()),
        _ => {
            let file = File::open(infile).expect("Could not open file!");
            Box::new(BufReader::new(file))
        }
    };

    let mut buf = Vec::new();
    infile.read_to_end(&mut buf).unwrap();
        
    let stdout = io::stdout();
    let mut outfile: Box<Write> = match outfile {
        "" => Box::new(stdout.lock()),
        _ => {
            let file = File::create(outfile).unwrap();
            Box::new(BufWriter::new(file))
        }
    };

    match msg_type {
        "" => guess_message(&buf, &mut outfile).unwrap(),
        _ => named_message(&buf, msg_type, &mut outfile).unwrap(),
    }
}
