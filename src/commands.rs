use decode::PqrsDecoder;

use stream_delimit::consumer::Consumer;
use stream_delimit::stream::*;
use stream_delimit::converter::Converter;
use std::io::{self, Write};
use clap::ArgMatches;
use errors::*;
use libc;

#[cfg(feature = "default")]
use stream_delimit::kafka_consumer::KafkaConsumer;

#[cfg(feature = "default")]
pub fn run_kafka(matches: &ArgMatches) -> Result<()> {
    if let (Some(brokers), Some(topic)) = (matches.value_of("BROKERS"), matches.value_of("TOPIC")) {
        let consumer = KafkaConsumer::new(brokers, topic, matches.is_present("FROMBEG"))?;
        decode_or_convert(consumer, matches)?;
    } else {
        bail!("Kafka needs a broker and topic");
    }
    Ok(())
}

#[cfg(not(feature = "default"))]
pub fn run_kafka(_: &ArgMatches) -> Result<()> {
    bail!("This version of pq has been compiled without kafka support");
}

pub fn run_byte(matches: &ArgMatches) -> Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    if unsafe { libc::isatty(libc::STDIN_FILENO) != 0 } {
        bail!("pq expects input to be piped from stdin");
    }
    let stream_type = str_to_streamtype(matches.value_of("STREAM").unwrap_or("single"))?;
    Ok(decode_or_convert(
        Consumer::new(&mut stdin, stream_type),
        matches,
    )?)
}

fn decode_or_convert<T: Iterator<Item = Vec<u8>>>(
    mut consumer: T,
    matches: &ArgMatches,
) -> Result<()> {
    let count = value_t!(matches, "COUNT", i32).unwrap_or(-1);

    let stdout = io::stdout();
    let out_is_tty = unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 };

    if let Some(convert_type) = matches.value_of("CONVERT") {
        let converter = Converter::new(&mut consumer, str_to_streamtype(convert_type)?);
        let stdout_ = &mut stdout.lock();
        for (ctr, item) in converter.enumerate() {
            if count >= 0 && ctr >= count as usize {
                return Ok(());
            }
            stdout_.write_all(&item).expect("Couldn't write to stdout");
        }
    } else {
        let decoder = match PqrsDecoder::new(matches.value_of("MSGTYPE").expect(
            "Must supply --msgtype or --convert",
        )) {
            Ok(x) => x,
            Err(e) => bail!(e),
        };

        for (ctr, item) in consumer.enumerate() {
            if count >= 0 && ctr >= count as usize {
                return Ok(());
            }
            match decoder.decode_message(&item, &mut stdout.lock(), out_is_tty) {
                Ok(_) => (),
                Err(e) => bail!(e),
            }
        }
    }
    Ok(())
}
