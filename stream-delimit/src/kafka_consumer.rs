#![deny(missing_docs)]

use crate::error::*;
use crate::stream::FramedRead;
use rdkafka::{
    consumer::{BaseConsumer, Consumer, DefaultConsumerContext},
    ClientConfig, Message,
};

/// A consumer from Kafka
pub struct KafkaConsumer {
    consumer: BaseConsumer<DefaultConsumerContext>,
}

impl FramedRead for KafkaConsumer {
    fn read_next_frame<'a>(
        &mut self,
        buffer: &'a mut Vec<u8>,
    ) -> std::io::Result<Option<&'a [u8]>> {
        self.consumer
            .poll(None)
            .transpose()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))
            .map(move |o| {
                o.and_then(move |m| {
                    m.payload().map(move |p| {
                        buffer.clear();
                        buffer.extend_from_slice(p);
                        &buffer[..]
                    })
                })
            })
    }
}

impl Iterator for KafkaConsumer {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let mut buffer = Vec::new();
        self.read_next_frame(&mut buffer)
            .expect("Failed to read next kafka message")?;
        Some(buffer)
    }
}

impl KafkaConsumer {
    /// Return a KafkaConsumer with some basic kafka connection properties
    pub fn new(brokers: &str, topic: &str, from_beginning: bool) -> Result<KafkaConsumer> {
        let mut config = ClientConfig::new();
        // we create a group id we can be reasonably sure is unique.
        // this is to safeguard against some other process accessing kafka under the same group id.
        // We are not committing or reading any offsets, so this is fine
        let group_id = format!(
            "pq-{}-pid-{}-starttime-{}",
            env!("CARGO_PKG_VERSION"),
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );
        config
            .set("bootstrap.servers", brokers)
            .set("enable.auto.commit", "false")
            .set(
                "auto.offset.reset",
                if from_beginning { "earliest" } else { "latest" },
            )
            .set("group.id", &group_id);
        let consumer: BaseConsumer = config.create()?;
        consumer.subscribe(&[topic])?;
        Ok(KafkaConsumer { consumer })
    }
}
