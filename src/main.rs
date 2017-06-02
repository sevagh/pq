#![crate_type = "bin"]

#[macro_use]
extern crate clap;
extern crate libc;
extern crate protobuf;
extern crate stream_delimit;

mod discovery;
mod error;
mod decode;

use std::fs::File;
use decode::PqrsDecoder;
use stream_delimit::stream_consumer::StreamConsumer;
use stream_delimit::stream_type::string_to_stream_type;
use stream_delimit::stream_converter::StreamConverter;
use std::io::{self, Read, BufReader, Write, stderr};
use std::process;
use error::PqrsError;
use clap::ArgMatches;

macro_rules! errexit {
    ($error:expr) => ({
        writeln!(&mut stderr(), "{}", $error).unwrap();
        process::exit(255);
    });
}

fn main() {
    include_str!("../Cargo.toml");
    let matches = clap_app!(
        @app (app_from_crate!())
        (@arg MSGTYPE: +required "Sets protobuf message type")
        (@arg INPUT: --input +takes_value "Sets the input file to use")
        (@arg STREAM: --stream +takes_value "Enables stream + sets stream type")
        (@arg COUNT: --count +takes_value +global "Stop after count messages")
        (@arg CONVERT: --convert +takes_value +global "Convert to different stream type")
        (@subcommand kafka =>
            (@arg TOPIC: +required "Sets the kafka topic")
            (@arg BROKERS: +required --brokers +takes_value "Comma-separated kafka brokers")
            (@arg FROMBEG: --beginning "Consume topic from beginning")
        )
    )
            .get_matches();

    match matches.subcommand() {
        ("kafka", Some(m)) => run_kafka(m),
        _ => run_byte(&matches),
    }
}

fn run_kafka(matches: &ArgMatches) {
    if let (Some(brokers), Some(topic)) = (matches.value_of("BROKERS"), matches.value_of("TOPIC")) {
        match StreamConsumer::for_kafka(String::from(brokers),
                                        String::from(topic),
                                        matches.is_present("FROMBEG")) {
            Ok(x) => decode_or_convert(x, matches),
            Err(e) => errexit!(e),
        }
    } else {
        errexit!(PqrsError::ArgumentError);
    }
}

fn run_byte(matches: &ArgMatches) {
    let stdin = io::stdin();
    let mut input: Box<Read> = match matches.value_of("INPUT") {
        Some(x) => {
            let file = match File::open(&x) {
                Ok(x) => x,
                Err(e) => errexit!(e),
            };
            Box::new(BufReader::new(file))
        }
        None => {
            if unsafe { libc::isatty(libc::STDIN_FILENO) != 0 } {
                println!("pq expects input to be piped from stdin");
                process::exit(0);
            }
            Box::new(stdin.lock())
        }
    };
    decode_or_convert(StreamConsumer::for_byte(string_to_stream_type(matches
                                                                         .value_of("STREAM")
                                                                         .unwrap_or("single")),
                                               &mut input),
                      matches);
}

fn decode_or_convert(consumer: StreamConsumer, matches: &ArgMatches) {
    let count = value_t!(matches, "COUNT", i32).unwrap_or(-1);

    let stdout = io::stdout();
    let out_is_tty = unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 };

    if let Some(ref convert_type) = matches.value_of("CONVERT") {
        let converter = StreamConverter::new(consumer, string_to_stream_type(convert_type));
        let stdout_ = &mut stdout.lock();
        for (ctr, item) in converter.enumerate() {
            if count >= 0 {
                if ctr >= count as usize {
                    process::exit(0);
                }
            }
            stdout_.write_all(&item).unwrap();
        }
    } else {
        let mut decoder = match PqrsDecoder::new(matches.value_of("MSGTYPE").unwrap_or_else(|| errexit!("Must specify --msgtype") )) {
            Ok(x) => x,
            Err(e) => errexit!(e),
        };

        for (ctr, item) in consumer.enumerate() {
            if count >= 0 {
                if ctr >= count as usize {
                    process::exit(0);
                }
            }
            match decoder.decode_message(&item, &mut stdout.lock(), out_is_tty) {
                Ok(_) => (),
                Err(e) => errexit!(e),
            }
        }
    }
}
