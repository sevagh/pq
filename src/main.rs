#![crate_type = "bin"]

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
use std::env;
use std::fs::{File, read_dir};
use std::io::{self, BufWriter, Write, BufReader, Read};
use std::path::PathBuf;

const USAGE: &'static str = "
pq - Protobuf to json

Usage:
  pq [<infile>] [--type=<string>] [-o=<outfile>] [-f=<fdsetpath>]
  pq (-h | --help)
  pq --version

Options:
  --type=<msg_type>     Message type e.g. com.example.Type 
  -o, --outfile         Output file path
  -f, --fdsetpath       Alternative path to fdsets
  -h --help             Show this screen.
  --version             Show version.
";

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    let infile = args.get_str("<infile>");
    let msg_type = args.get_str("--type");
    let outfile = args.get_str("-o");
    let fdset_path = args.get_str("-f");

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

    let fdsets = discover_fdsets(fdset_path);

    match msg_type {
        "" => guess_message(&buf, &mut outfile, fdsets).unwrap(),
        _ => named_message(&buf, msg_type, &mut outfile, fdsets).unwrap(),
    }
}

fn discover_fdsets(fdsetpath: &str) -> Vec<PathBuf> {
    let path = match fdsetpath {
        "" => {
            let mut home = env::home_dir().expect("Could not find $HOME");
            home.push(".pq");
            home
        },
        _ => PathBuf::from(fdsetpath),
    };

    read_dir(path.as_path()).unwrap()
        .map(|x| x.unwrap().path()).collect::<Vec<_>>()
}
