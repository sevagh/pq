extern crate futures;
extern crate rdkafka;
extern crate rdkafka_sys;

use futures::stream::Stream;

use rdkafka::client::{Context};
use rdkafka::consumer::{Consumer, ConsumerContext, CommitMode, Rebalance};
use rdkafka::consumer::stream_consumer::{StreamConsumer, MessageStream};
use rdkafka::config::{ClientConfig, TopicConfig, RDKafkaLogLevel};

use std::io::Read;

const MAX_ATTEMPTS: usize = 10;

pub struct StreamDelimiter<'a> {
    delim: &'a str,
    read: Option<&'a mut Read>,
    kafka_consumer: Option<&'a KafkaConsumer<'a>>,
}

impl<'a> StreamDelimiter<'a> {
    pub fn new(delim: &'a str, read: &'a mut Read) -> StreamDelimiter<'a> {
        StreamDelimiter {
            delim: delim,
            read: Some(read),
            kafka_consumer: None,
        }
    }

    pub fn for_kafka(brokers: Option<String>, topic: Option<String>, from_beginning: bool) -> StreamDelimiter<'a> {
        StreamDelimiter {
            delim: "kafka",
            read: None,
            kafka_consumer: Some(&KafkaConsumer::new(topic, brokers, from_beginning)),
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
                    return self.kafka_consumer.unwrap().consume_single();
            }
            _ => panic!("Unknown delimiter"),
        }
        ret
    }
}

struct ConsumerContextExample;

impl Context for ConsumerContextExample {}

impl ConsumerContext for ConsumerContextExample {
    fn pre_rebalance(&self, _: &Rebalance) {}

    fn post_rebalance(&self, _: &Rebalance) {}
}

type LoggingConsumer = StreamConsumer<ConsumerContextExample>;

struct KafkaConsumer<'a> {
    consumer: &'a LoggingConsumer,
    message_stream: &'a MessageStream,
}

impl <'a>KafkaConsumer<'a> {
    fn new(brokers: Option<String>, topic: Option<String>, from_beginning: bool) -> KafkaConsumer<'a> {
        let context = ConsumerContextExample;

        let auto_offset_reset: &str;
        if from_beginning {
            auto_offset_reset = "earliest";
        } else {
            auto_offset_reset = "smallest";
        }

        let Some(ref brokers_) = brokers;
        let Some(ref topic_) = topic;

        let mut consumer = ClientConfig::new()
            .set("group.id", "pq-consumer-group-id")
            .set("bootstrap.servers", brokers_)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set("statistics.interval.ms", "5000")
            .set_default_topic_config(TopicConfig::new()
                .set("auto.offset.reset", auto_offset_reset)
                .finalize())
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context::<_, LoggingConsumer>(context)
            .expect("Consumer creation failed");

        consumer.subscribe(&vec![topic_]).expect("Can't subscribe to specified topics");
        let message_stream = consumer.start();

        KafkaConsumer{
            consumer: &consumer,
            message_stream: &message_stream,
        }
    }

    fn consume_single(&mut self) -> Option<Vec<u8>> {
        let ret: Option<Vec<u8>>;
        for message in self.message_stream.take(1).wait() {
            match message {
                Err(_) => ret = None,
                Ok(Ok(m)) => {
                    let _ = match m.payload_view::<[u8]>() {
                        None => ret = None,
                        Some(Ok(s)) => ret = Some(s.to_vec()),
                        Some(Err(e)) => ret = None,
                    };
                    self.consumer.commit_message(&m, CommitMode::Async).unwrap();
                },
                Ok(Err(e)) => ret = None,
            };
        }
        return ret
    }
}
