use kafka_stream::*;
use varint::decode_varint;
use stream_type::StreamType;

use std::io::Read;

pub struct StreamConsumer<'a> {
    pub stream_type: StreamType,
    pub read: Option<&'a mut Read>,
    pub kafka_consumer: Option<KafkaConsumer>,
}

impl<'a> Iterator for StreamConsumer<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let mut ret: Option<Vec<u8>> = None;
        match self.stream_type {
            StreamType::ByteVarint => {
                if let Some(ref mut read_) = self.read {
                    match decode_varint(read_) {
                        Ok(x) => {
                            let mut msg_buf = vec![0; x as usize];
                            match read_.read_exact(&mut msg_buf) {
                                Ok(_) => (),
                                Err(_) => return None,
                            }
                            ret = Some(msg_buf);
                        }
                        Err(_) => ret = None,
                    }
                }
            }
            StreamType::ByteLeb128 => unimplemented!(),
            StreamType::Single => {
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
            StreamType::Kafka => {
                let kafka_consumer = self.kafka_consumer.as_mut().unwrap();
                return kafka_consumer.get_single_message();
            }
        }
        ret
    }
}
