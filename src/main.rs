#![crate_type = "bin"]

#[macro_use]
extern crate clap;
extern crate atty;
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

use std::fs::File;
use decode::PqrsDecoder;
use stream_delimit::stream_consumer::StreamConsumer;
use stream_delimit::stream_type::{string_to_stream_type, StreamType};
use stream_delimit::stream_converter::StreamConverter;
use std::io::{self, Read, BufReader, Write, stderr};
use std::process;
use error::PqrsError;

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
        (@arg INPUT: "Sets the input file to use")
        (@arg MSGTYPE: --msgtype +takes_value "Sets protobuf message type")
        (@arg STREAM: --stream +takes_value "Enabled stream + sets stream type")
        (@arg COUNT: --count +takes_value "Stop after count messages")
        (@arg CONVERT: --convert +takes_value "Convert to different stream type")
        (@subcommand kafka =>
            (@arg TOPIC: +required "Sets the kafka topic")
            (@arg BROKERS: +required --brokers +takes_value "Comma-separated kafka brokers")
            (@arg FROMBEG: --from_beginning "Consume topic from beginning")
        )
    )
            .get_matches();

    let pqrs_decoder = match PqrsDecoder::new(matches.value_of("MSGTYPE")) {
        Ok(x) => x,
        Err(e) => errexit!(e),
    };

    let count = value_t!(matches, "COUNT", i32).unwrap_or(-1);

    match matches.subcommand() {
        ("kafka", _) => {
            run_kafka(pqrs_decoder,
                      matches.value_of("BROKERS"),
                      matches.value_of("TOPIC"),
                      matches.is_present("FROMBEG"),
                      matches.value_of("CONVERT"),
                      count)
        }
        _ => {
            run_byte(pqrs_decoder,
                     matches.value_of("INPUT"),
                     string_to_stream_type(matches.value_of("STREAM").unwrap_or("single")),
                     matches.value_of("CONVERT"),
                     count)
        }
    }
}

fn run_kafka(decoder: PqrsDecoder,
             brokers: Option<&str>,
             topic: Option<&str>,
             from_beginning: bool,
             convert: Option<&str>,
             count: i32) {
    if let (Some(brokers), Some(topic)) = (brokers, topic) {
        match StreamConsumer::for_kafka(String::from(brokers),
                                        String::from(topic),
                                        from_beginning) {
            Ok(x) => decode_or_convert(decoder, x, convert, count),
            Err(e) => errexit!(e),
        }
    } else {
        errexit!(PqrsError::ArgumentError);
    }
}

fn run_byte(decoder: PqrsDecoder,
            input: Option<&str>,
            stream: StreamType,
            convert: Option<&str>,
            count: i32) {
    let stdin = io::stdin();
    let mut input: Box<Read> = match input {
        Some(x) => {
            let file = match File::open(&x) {
                Ok(x) => x,
                Err(e) => errexit!(e),
            };
            Box::new(BufReader::new(file))
        }
        None => {
            if atty::is(atty::Stream::Stdin) {
                println!("pq expects input to be piped from stdin");
                process::exit(0);
            }
            Box::new(stdin.lock())
        }
    };
    decode_or_convert(decoder,
                      StreamConsumer::for_byte(stream, &mut input),
                      convert,
                      count);
}

fn decode_or_convert(decoder: PqrsDecoder,
                     consumer: StreamConsumer,
                     convert: Option<&str>,
                     count: i32) {
    let stdout = io::stdout();
    let out_is_tty = atty::is(atty::Stream::Stdout);

    if let Some(ref convert_type) = convert {
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
