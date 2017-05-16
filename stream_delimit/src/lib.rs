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
                return kafka_consumer.get_single_message();
            }
            _ => panic!("Unknown delimiter"),
        }
        ret
    }
}

struct KafkaConsumer {
    consumer: Consumer,
    messages: Vec<Vec<u8>>,
}

impl KafkaConsumer {
    fn new(brokers: &str,
           topic: &str,
           from_beginning: bool)
           -> Result<KafkaConsumer, StreamDelimitError> {
        let fetch_offset = if from_beginning {
            FetchOffset::Latest
        } else {
            FetchOffset::Earliest
        };
        match Consumer::from_hosts(brokers
                                       .split(',')
                                       .map(|x| x.to_owned())
                                       .collect::<Vec<String>>())
                      .with_topic_partitions(topic.to_owned(), &[0, 1])
                      .with_fallback_offset(fetch_offset)
                      .create() {
            Ok(consumer) => {
                Ok(KafkaConsumer {
                       consumer: consumer,
                       messages: vec![],
                   })
            }
            Err(_) => Err(StreamDelimitError::KafkaInitializeError),
        }
    }

    fn get_single_message(&mut self) -> Option<Vec<u8>> {
        if self.messages.is_empty() {
            let kafka_consumer = &mut self.consumer;
            match kafka_consumer.poll() {
                Ok(x) => {
                    let x = x.iter().take(1).next().unwrap();
                    self.messages
                        .append(&mut x.messages()
                                         .iter()
                                         .map(|x| x.value.to_vec())
                                         .collect::<Vec<_>>());
                    kafka_consumer.consume_messageset(x).unwrap();
                }
                Err(_) => return None,
            }
            kafka_consumer.commit_consumed().unwrap();
        }
        self.messages.pop()
    }
}
