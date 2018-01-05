use decode::PqDecoder;
use discovery::get_loaded_descriptors;

use stream_delimit::byte_consumer::ByteConsumer;
use stream_delimit::stream::*;
use stream_delimit::converter::Converter;
use std::io::{self, Write};
use clap::ArgMatches;
use nix::unistd::isatty;
use std::path::PathBuf;
use protobuf::descriptor::FileDescriptorSet;

pub struct CommandRunner {
    descriptors: Vec<FileDescriptorSet>,
}

#[cfg(feature = "default")]
use stream_delimit::kafka_consumer::KafkaConsumer;

impl CommandRunner {
    pub fn new(additional_fdset_dirs: Vec<PathBuf>, additional_fdset_files: Vec<PathBuf>) -> Self {
        let descriptors = get_loaded_descriptors(additional_fdset_dirs, additional_fdset_files);
        CommandRunner { descriptors }
    }

    #[cfg(feature = "default")]
    pub fn run_kafka(self, matches: &ArgMatches) {
        if let (Some(brokers), Some(topic)) =
            (matches.value_of("BROKERS"), matches.value_of("TOPIC"))
        {
            let consumer = match KafkaConsumer::new(brokers, topic, matches.is_present("FROMBEG")) {
                Ok(x) => x,
                Err(e) => panic!("Couldn't initialize kafka consumer: {}", e),
            };
            decode_or_convert(consumer, matches, self.descriptors);
        } else {
            panic!("Kafka needs broker[s] and topic");
        }
    }

    #[cfg(not(feature = "default"))]
    pub fn run_kafka(self, _: &ArgMatches) {
        not_implemented!("This version of pq has been compiled without kafka support");
    }

    pub fn run_byte(self, matches: &ArgMatches) {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        match isatty(0) {
            Ok(stdin_is_tty) => {
                if !stdin_is_tty {
                    panic!("pq expects input piped from stdin")
                }
            }
            Err(e) => panic!("Error checking if stdin is tty: {}", e),
        }
        let stream_type = str_to_streamtype(matches.value_of("STREAM").unwrap_or("single"))
            .expect("Couldn't convert str to streamtype");
        decode_or_convert(
            ByteConsumer::new(&mut stdin, stream_type),
            matches,
            self.descriptors,
        )
    }
}

fn decode_or_convert<T: Iterator<Item = Vec<u8>>>(
    mut consumer: T,
    matches: &ArgMatches,
    descriptors: Vec<FileDescriptorSet>,
) {
    let count = value_t!(matches, "COUNT", i32).unwrap_or(-1);

    let stdout = io::stdout();
    let out_is_tty = match isatty(1) {
        Ok(x) => x,
        Err(e) => panic!("Error checking if stdout is tty: {}", e),
    };

    if let Some(convert_type) = matches.value_of("CONVERT") {
        let converter = Converter::new(
            &mut consumer,
            str_to_streamtype(convert_type).expect("Couldn't convert str to streamtype"),
        );
        let stdout_ = &mut stdout.lock();
        for (ctr, item) in converter.enumerate() {
            if count >= 0 && ctr >= count as usize {
                return;
            }
            stdout_.write_all(&item).expect("Couldn't write to stdout");
        }
    } else {
        let decoder = PqDecoder::new(
            descriptors,
            matches
                .value_of("MSGTYPE")
                .expect("Must supply --msgtype or --convert"),
        );

        for (ctr, item) in consumer.enumerate() {
            if count >= 0 && ctr >= count as usize {
                return;
            }
            decoder.decode_message(&item, &mut stdout.lock(), out_is_tty);
        }
    }
}
