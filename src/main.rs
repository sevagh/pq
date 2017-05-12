#![crate_type = "bin"]

extern crate rustc_serialize;
extern crate atty;
extern crate docopt;
extern crate protobuf;
extern crate serde;
extern crate serde_protobuf;
extern crate serde_value;
extern crate serde_json;
extern crate stream_delimit;

mod fdset_discovery;
mod newline_pretty_formatter;
mod error;
mod decode;
#[macro_use]
mod macros;

use std::fs::File;
use docopt::Docopt;
use decode::PqrsDecoder;
use stream_delimit::StreamDelimiter;
use std::io::{self, Read, BufReader, Write, stderr};
use std::process;
use error::PqrsError;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
pq - protobuf to json

Usage:
  pq [<infile>] [--msgtype=<msgtype>] [--stream=<delim>] [--count=<count>]
  pq kafka <topic> --brokers=<brokers> [--from-beginning] [--msgtype=<msgtype>] [--count=<count>]
  pq (--help | --version)

Options:
  --stream=<delim>      Stream delimiter e.g. \"varint\", \"leb128\"
  --msgtype=<msgtype>   Message type e.g. com.example.Type
  --brokers=<brokers>   1.2.3.4:9092,5.6.7.8:9092
  --from-beginning      Consume kafka from beginning
  --count=<count>       Stop after count messages
  --help                Show this screen.
  --version             Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub cmd_kafka: bool,
    pub arg_infile: Option<String>,
    pub arg_topic: Option<String>,
    pub flag_msgtype: Option<String>,
    pub flag_count: Option<usize>,
    pub flag_stream: Option<String>,
    pub flag_from_beginning: bool,
    pub flag_brokers: Option<String>,
    flag_version: bool,
}

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let out_is_tty = atty::is(atty::Stream::Stdout);

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(String::from(VERSION))).decode())
        .unwrap_or_else(|e| e.exit());

    let pqrs_decoder = match PqrsDecoder::new(args.flag_msgtype) {
        Ok(x) => x,
        Err(e) => errexit!(e),
    };

    if args.cmd_kafka {
        if let (Some(brokers), Some(topic)) = (args.flag_brokers, args.arg_topic) {
            match StreamDelimiter::for_kafka(&brokers, &topic, args.flag_from_beginning) {
                Ok(delim) => {
                    for (ctr, item) in delim.enumerate() {
                        if let Some(count) = args.flag_count {
                            if ctr >= count {
                                process::exit(0);
                            }
                        }
                        match pqrs_decoder.decode_message(&item, &mut stdout.lock(), out_is_tty) {
                            Ok(_) => (),
                            Err(e) => errexit!(e),
                        }
                    }
                }
                Err(e) => errexit!(e),
            }
        } else {
            errexit!(PqrsError::ArgumentError)
        }
    } else {
        let mut infile: Box<Read> = match args.arg_infile {
            Some(x) => {
                let file = match File::open(&x) {
                    Ok(x) => x,
                    Err(e) => errexit!(e),
                };
                Box::new(BufReader::new(file))
            }
            None => {
                if atty::is(atty::Stream::Stdin) {
                    writeln!(stdout, "pq expects input to be piped from stdin").unwrap();
                    process::exit(0);
                }
                Box::new(stdin.lock())
            }
        };

        if let Some(lead_delim) = args.flag_stream {
            let delim = StreamDelimiter::new(&lead_delim, &mut infile);
            for (ctr, item) in delim.enumerate() {
                if let Some(count) = args.flag_count {
                    if ctr >= count {
                        process::exit(0);
                    }
                }
                match pqrs_decoder.decode_message(&item, &mut stdout.lock(), out_is_tty) {
                    Ok(_) => (),
                    Err(e) => errexit!(e),
                }
            }
        } else {
            let mut buf = Vec::new();
            match infile.read_to_end(&mut buf) {
                Ok(_) => (),
                Err(e) => errexit!(e),
            }
            match pqrs_decoder.decode_message(&buf, &mut stdout.lock(), out_is_tty) {
                Ok(_) => (),
                Err(e) => errexit!(e),
            }
        }
    }
}
