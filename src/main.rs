#![crate_type = "bin"]

extern crate protobuf;
extern crate rustc_serialize;
extern crate docopt;

mod unknown;

use protobuf::CodedInputStream;

use docopt::Docopt;

use unknown::Unknown;

use std::io::{self, Read};

const USAGE: &'static str = "
pq

Usage:
  pq [-s]
  pq (-h | --help)
  pq --version

Options:
  -s, --single  Consume single message.
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
}

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    match args.get_bool("-s")  {
        true => process_single(&mut handle),
        false => process_stream(&mut handle),
    }

}

fn process_stream(stdin_stream: &mut Read) {
    let mut stream = CodedInputStream::new(stdin_stream);

    loop {
        match stream.eof() {
            Err(e) => panic!(e),
            Ok(true) => break,
            Ok(false) => {
				match stream.read_message::<Unknown>() {
                    Err(e) => println!("{}", e),
                    Ok(x) => println!("{:?}", x),
                }
			}
        }
    }
}

fn process_single(stdin_stream: &mut Read) {
    let mut buffer = Vec::new();
    stdin_stream.read(&mut buffer).unwrap();

    let mut byte_is = CodedInputStream::from_bytes(&buffer);

    match byte_is.read_message::<Unknown>() {
        Err(e) => panic!(e),
        Ok(x) => println!("{:?}", x),
    }
}
