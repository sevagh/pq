extern crate kafka;

mod error;

use kafka::consumer::{Consumer, FetchOffset};
use error::StreamDelimitError;

use std::io::Read;

const MAX_ATTEMPTS: usize = 10;

pub struct StreamDelimiter<'a> {
    delim: String,
    read: Option<&'a mut Read>,
    kafka_consumer: Option<KafkaConsumer>,
}

impl<'a> StreamDelimiter<'a> {
    pub fn new(delim: String, read: &'a mut Read) -> StreamDelimiter<'a> {
        StreamDelimiter {
            delim: delim,
            read: Some(read),
            kafka_consumer: None,
        }
    }

    pub fn for_kafka(brokers: String,
                     topic: String,
                     from_beginning: bool)
                     -> Result<StreamDelimiter<'a>, StreamDelimitError> {
        let kfc = match KafkaConsumer::new(brokers.as_str(), topic.as_str(), from_beginning) {
            Ok(kfc) => kfc,
            Err(e) => return Err(e),
        };
        Ok(StreamDelimiter {
                delim: "kafka".to_owned(),
                read: None,
                kafka_consumer: Some(kfc),
            })
    }
}

impl<'a> Iterator for StreamDelimiter<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let mut ret: Option<Vec<u8>> = None;
        match self.delim.as_str() {
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
            "single" => {
                let mut buf = Vec::new();
                if let Some(ref mut read_) = self.read {
                    match read_.read_to_end(&mut buf) {
                        Ok(x) => {
                            if x > 0 {
                                ret = Some(buf);
                            } else {
                                ret = None
                            }
                        }
                        Err(_) => ret = None,
                    }
                }
            }
            "kafka" => {
                let kafka_consumer = self.kafka_consumer.as_mut().unwrap();
                for mut ms in kafka_consumer {
                    ret = ms.pop();
                    break;
                }
            }
            _ => panic!("Unknown delimiter"),
        }
        ret
    }
}

struct KafkaConsumer {
    consumer: Consumer,
}

impl<'a> KafkaConsumer {
    fn new(brokers: &str,
           topic: &str,
           from_beginning: bool)
           -> Result<KafkaConsumer, StreamDelimitError> {
        let fetch_offset: FetchOffset;
        if from_beginning {
            fetch_offset = FetchOffset::Latest;
        } else {
            fetch_offset = FetchOffset::Earliest;
        }
        match  
        Consumer::from_hosts(brokers.split(",").map(|x| x.to_owned()).collect::<Vec<String>>())
            .with_topic_partitions(topic.to_owned(), &[0, 1])
            .with_fallback_offset(fetch_offset)
            .create() {
                Ok(consumer) => { Ok(KafkaConsumer{ consumer }) }
                Err(_) => Err(StreamDelimitError::KafkaInitializeError),
        }
    }
}

impl Iterator for KafkaConsumer {
    type Item = Vec<Vec<u8>>;

    fn next(&mut self) -> Option<Vec<Vec<u8>>> {
        let ref mut kafka_consumer = self.consumer;
        let mut ret: Option<Vec<Vec<u8>>> = None;
        for ms in kafka_consumer.poll().unwrap().iter().take(1) {
            ret = Some(ms.messages().iter().map(|x| x.value.to_vec()).collect::<Vec<_>>());
            kafka_consumer.consume_messageset(ms).unwrap();
        }
        kafka_consumer.commit_consumed().unwrap();
        ret
    }
}
