#![crate_type = "bin"]

#[macro_use]
extern crate clap;
extern crate libc;
extern crate protobuf;
extern crate serde;
extern crate serde_protobuf;
extern crate serde_value;
extern crate serde_json;
extern crate stream_delimit;
#[macro_use]
extern crate error_chain;

mod discovery;
mod newline_pretty_formatter;
mod decode;
mod errors;

use decode::PqrsDecoder;
use stream_delimit::consumer::*;
use stream_delimit::converter::StreamConverter;
use std::io::{self, Write};
use std::process;
use std::fmt::Display;
use clap::ArgMatches;

fn main() {
    include_str!("../Cargo.toml");
    let matches = clap_app!(
        @app (app_from_crate!())
        (@arg MSGTYPE: --msgtype +takes_value +global conflicts_with[CONVERT]
            "Sets protobuf message type")
        (@arg STREAM: --stream +takes_value "Enables stream + sets stream type")
        (@arg COUNT: --count +takes_value +global "Stop after count messages")
        (@arg CONVERT: --convert +takes_value +global "Convert to different stream type")
        (@subcommand kafka =>
            (@arg TOPIC: +required "Sets the kafka topic")
            (@arg BROKERS: +required --brokers +takes_value "Comma-separated kafka brokers")
            (@arg FROMBEG: --beginning "Consume topic from beginning")
        )
    ).get_matches();

    match matches.subcommand() {
        ("kafka", Some(m)) => run_kafka(m),
        _ => run_byte(&matches),
    }
}

fn run_kafka(matches: &ArgMatches) {
    if let (Some(brokers), Some(topic)) = (matches.value_of("BROKERS"), matches.value_of("TOPIC")) {
        match KafkaConsumer::new(brokers, topic, matches.is_present("FROMBEG")) {
            Ok(mut x) => decode_or_convert(StreamConsumer::new(&mut x), matches),
            Err(e) => errexit(&e, 255),
        }
    } else {
        errexit(&String::from("Kafka needs a broker and topic"), 255);
    }
}

fn run_byte(matches: &ArgMatches) {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    if unsafe { libc::isatty(libc::STDIN_FILENO) != 0 } {
        println!("pq expects input to be piped from stdin");
        process::exit(0);
    }
    let mut byte_consumer: Box<GenericConsumer> =
        match matches.value_of("STREAM").unwrap_or("single") {
            "single" => Box::new(SingleConsumer::new(&mut stdin)),
            "varint" => Box::new(VarintConsumer::new(&mut stdin)),
            _ => {
                errexit(
                    &String::from("Only supports stream types single and varint"),
                    255,
                )
            }
        };
    decode_or_convert(StreamConsumer::new(byte_consumer.as_mut()), matches);
}

fn decode_or_convert(mut consumer: StreamConsumer, matches: &ArgMatches) {
    let count = value_t!(matches, "COUNT", i32).unwrap_or(-1);

    let stdout = io::stdout();
    let out_is_tty = unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 };

    if let Some(convert_type) = matches.value_of("CONVERT") {
        let converter = StreamConverter::new(&mut consumer, convert_type);
        let stdout_ = &mut stdout.lock();
        for (ctr, item) in converter.enumerate() {
            if count >= 0 && ctr >= count as usize {
                process::exit(0);
            }
            stdout_.write_all(&item).expect("Couldn't write to stdout");
        }
    } else {
        let decoder = match PqrsDecoder::new(matches.value_of("MSGTYPE").expect(
            "Must supply --msgtype or --convert",
        )) {
            Ok(x) => x,
            Err(e) => errexit(&e, 255),
        };

        for (ctr, item) in consumer.enumerate() {
            if count >= 0 && ctr >= count as usize {
                process::exit(0);
            }
            match decoder.decode_message(&item, &mut stdout.lock(), out_is_tty) {
                Ok(_) => (),
                Err(e) => errexit(&e, 255),
            }
        }
    }
}

fn errexit<T: Display>(msg: &T, exit_code: i32) -> ! {
    eprintln!("{}", msg);
    process::exit(exit_code);
}
