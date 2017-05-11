extern crate futures;
extern crate rdkafka;
extern crate rdkafka_sys;

use futures::stream::Stream;

use rdkafka::client::{Context};
use rdkafka::consumer::{Consumer, ConsumerContext, CommitMode, Rebalance};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::config::{ClientConfig, TopicConfig, RDKafkaLogLevel};

use std::io::Read;

const MAX_ATTEMPTS: usize = 10;

pub struct StreamDelimiter<'a> {
    delim: &'a str,
    read: Option<&'a mut Read>,
    topic: Option<String>,
    brokers: Option<String>,
    from_beginning: Option<bool>,
}

impl<'a> StreamDelimiter<'a> {
    pub fn new(delim: &'a str, read: &'a mut Read) -> StreamDelimiter<'a> {
        StreamDelimiter {
            delim: delim,
            read: Some(read),
            topic: None,
            brokers: None,
            from_beginning: None,
        }
    }

    pub fn for_kafka(brokers: Option<String>, topic: Option<String>, from_beginning: bool) -> StreamDelimiter<'a> {
        StreamDelimiter {
            delim: "kafka",
            read: None,
            topic: topic,
            brokers: brokers,
            from_beginning: Some(from_beginning),
        }
    }
}

impl<'a> Iterator for StreamDelimiter<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let mut ret: Option<Vec<u8>> = None;
        match self.delim {
            "varint" => {
                if let Some(ref mut read_) = self.read {
                    let mut varint_buf: Vec<u8> = Vec::new();
                    for i in 0..MAX_ATTEMPTS {
                        varint_buf.push(0u8);
                        match read_.read_exact(&mut varint_buf[i..]) {
                            Ok(_) => (),
                            Err(_) => break,
                        }
                        if (varint_buf[i] & 0b10000000) >> 7 != 0x1 {
                            let mut concat: u64 = 0;
                            for i in (0..varint_buf.len()).rev() {
                                let i_ = i as u32;
                                concat += ((varint_buf[i] & 0b01111111) as u64) <<
                                        (i_ * (8u32.pow(i_) - 1));
                            }
                            let mut msg_buf = vec![0; concat as usize];
                            match read_.read_exact(&mut msg_buf) {
                                Ok(_) => (),
                                Err(_) => break,
                            }
                            ret = Some(msg_buf);
                            break;
                        }
                    }
                }
            }
            "leb128" => unimplemented!(),
            "kafka" => {
                if let Some(ref brokers) = self.brokers {
                    if let Some(ref topic) = self.topic {
                        consume_and_print(brokers, topic);
                    }
                }
            }
            _ => panic!("Unknown delimiter"),
        }
        ret
    }
}

struct ConsumerContextExample;

impl Context for ConsumerContextExample {}

impl ConsumerContext for ConsumerContextExample {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }
}

type LoggingConsumer = StreamConsumer<ConsumerContextExample>;

fn consume_and_print(brokers: &str, topic: &str) {
    let context = ConsumerContextExample;

    let mut consumer = ClientConfig::new()
        .set("group.id", "pq-consumer-group-id")
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("statistics.interval.ms", "5000")
        .set_default_topic_config(TopicConfig::new()
            .set("auto.offset.reset", "smallest")
            .finalize())
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context::<_, LoggingConsumer>(context)
        .expect("Consumer creation failed");

    consumer.subscribe(&vec![topic]).expect("Can't subscribe to specified topics");

    let message_stream = consumer.start();

    for message in message_stream.take(5).wait() {
        match message {
            Err(_) => {
                println!("Error while reading from stream.");
            },
            Ok(Ok(m)) => {
                let key = match m.key_view::<[u8]>() {
                    None => &[],
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message key: {:?}", e);
                        &[]
                    },
                };
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    },
                };
                println!("key: '{:?}', payload: '{}', partition: {}, offset: {}",
                      key, payload, m.partition(), m.offset());
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            },
            Ok(Err(e)) => {
                println!("Kafka error: {}", e);
            },
        };
    }
}
