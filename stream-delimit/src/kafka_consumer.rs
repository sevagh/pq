#![deny(missing_docs)]

use kafka::consumer::{Consumer, FetchOffset};
use std::{self, thread, time};
use error::*;

/// A consumer from Kafka
pub struct KafkaConsumer {
    consumer: Consumer,
    messages: Vec<Vec<u8>>,
}

impl Iterator for KafkaConsumer {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        if self.messages.is_empty() {
            let kafka_consumer = &mut self.consumer;
            loop {
                match kafka_consumer.poll() {
                    Ok(x) => match x.iter().take(1).next() {
                        Some(y) => {
                            self.messages.append(&mut y.messages()
                                .iter()
                                .map(|z| z.value.to_vec())
                                .collect::<Vec<_>>());
                            kafka_consumer
                                .consume_messageset(y)
                                .expect("Couldn't mark messageset as consumed");
                            kafka_consumer
                                .commit_consumed()
                                .expect("Couldn't commit consumption");
                            break;
                        }
                        None => {
                            thread::sleep(time::Duration::from_secs(1));
                            continue;
                        }
                    },
                    Err(_) => return None,
                }
            }
        }
        self.messages.pop()
    }
}

impl KafkaConsumer {
    /// Return a KafkaConsumer with some basic kafka connection properties
    pub fn new(brokers: &str, topic: &str, from_beginning: bool) -> Result<KafkaConsumer> {
        let fetch_offset = if from_beginning {
            FetchOffset::Earliest
        } else {
            FetchOffset::Latest
        };
        match Consumer::from_hosts(
            brokers
                .split(',')
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<String>>(),
        ).with_topic(topic.to_owned())
            .with_fallback_offset(fetch_offset)
            .create()
        {
            Ok(consumer) => Ok(KafkaConsumer {
                consumer: consumer,
                messages: vec![],
            }),
            Err(e) => Err(StreamDelimitError::KafkaInitializeError(e))?,
        }
    }
}
