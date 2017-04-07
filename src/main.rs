#![crate_type = "bin"]

extern crate rustc_serialize;
extern crate docopt;
extern crate protobuf;
extern crate serde;
extern crate serde_protobuf;
extern crate serde_value;
extern crate serde_json;

mod protob;
mod error;
mod discovery;
mod force;

use discovery::discover_fdsets;
use docopt::Docopt;
use protob::{named_message, guess_message};
use error::PqrsError;
use std::boxed::Box;
use std::fs::File;
use std::io::{self, BufWriter, Write, BufReader, Read};
use std::process;

const USAGE: &'static str = "
pq - Protobuf to json

Usage:
  pq [--msgtype=<msgtype>] [--outfile=<path>] [--fdsets=<path>] [--force] [<infile>]
  pq (-h | --help)
  pq --version

Options:
  --msgtype=<msgtype>   Message type e.g. com.example.Type
  --outfile=<path>      Output file path
  --fdsets=<path>       Alternative path to fdsets
  --force               Decode by deleting chars until success
  -h --help             Show this screen.
  --version             Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub arg_infile: Option<String>,
    pub flag_outfile: Option<String>,
    pub flag_msgtype: Option<String>,
    pub flag_fdsets: Option<String>,
    pub flag_force: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    let stdin = io::stdin();
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

    let mut buf = Vec::new();
    match infile.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(_) => {
            writeln!(&mut stderr, "Could not real file to end").unwrap();
            process::exit(-1);
        }
    }

    let stdout = io::stdout();
    let mut outfile: Box<Write> = match args.flag_outfile {
        Some(x) => {
            let file = match File::create(&x) {
                Ok(y) => y,
                Err(_) => {
                    writeln!(&mut stderr, "Could not create file: {} for writing", x).unwrap();
                    process::exit(-1);
                }
            };
            Box::new(BufWriter::new(file))
        }
        None => Box::new(stdout.lock()),
    };

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

    if args.flag_force {
        let cloned_buf = buf.clone();
        match args.flag_msgtype {
            Some(x) => match named_message(&cloned_buf, &x, &mut outfile, fdsets) {
                while !cloned_buf.is_empty() {
            }
            None => match guess_message(&buf, &mut outfile, fdsets) {
            }
        }
    }
}
