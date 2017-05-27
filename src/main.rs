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
use stream_delimit::stream_consumer::StreamConsumer;
use stream_delimit::stream_type::string_to_stream_type;
use stream_delimit::stream_converter::StreamConverter;
use std::io::{self, Read, BufReader, Write, stderr};
use std::process;
use error::PqrsError;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, RustcDecodable)]
struct Args {
    pub cmd_kafka: bool,
    pub arg_infile: Option<String>,
    pub arg_topic: Option<String>,
    pub flag_dump: Option<String>,
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

    let args: Args = Docopt::new(include_str!("usage.txt"))
        .and_then(|d| d.version(Some(String::from(VERSION))).decode())
        .unwrap_or_else(|e| e.exit());

    let pqrs_decoder = match PqrsDecoder::new(args.flag_msgtype) {
        Ok(x) => x,
        Err(e) => errexit!(e),
    };

    let mut infile: Option<Box<Read>>;
    if args.cmd_kafka {
        infile = None;
    } else {
        infile = match args.arg_infile {
            Some(x) => {
                let file = match File::open(&x) {
                    Ok(x) => x,
                    Err(e) => errexit!(e),
                };
                Some(Box::new(BufReader::new(file)))
            }
            None => {
                if atty::is(atty::Stream::Stdin) {
                    writeln!(stdout, "pq expects input to be piped from stdin").unwrap();
                    process::exit(0);
                }
                Some(Box::new(stdin.lock()))
            }
        }
    }

    let consumer: StreamConsumer;

    if args.cmd_kafka {
        if let (Some(brokers), Some(topic)) = (args.flag_brokers, args.arg_topic) {
            match StreamConsumer::for_kafka(brokers, topic, args.flag_from_beginning) {
                Ok(x) => consumer = x,
                Err(e) => errexit!(e),
            }
        } else {
            errexit!(PqrsError::ArgumentError);
        }
    } else {
        match infile {
            Some(ref mut x) => {
                consumer =
                    StreamConsumer::for_byte(string_to_stream_type(args.flag_stream
                                                                       .unwrap_or_default()
                                                                       .as_str()),
                                             x);
            }
            None => errexit!(PqrsError::ArgumentError),
        }
    }

    if let Some(ref dump_type) = args.flag_dump {
        let converter = StreamConverter::new(consumer, string_to_stream_type(dump_type));
        let stdout_ = &mut stdout.lock();
        for (ctr, item) in converter.enumerate() {
            if let Some(count) = args.flag_count {
                if ctr >= count {
                    process::exit(0);
                }
            }
            stdout_.write_all(&item).unwrap();
        }
    } else {
        for (ctr, item) in consumer.enumerate() {
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
}
