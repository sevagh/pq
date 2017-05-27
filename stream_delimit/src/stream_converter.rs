use varint::encode_varint;
use stream_type::StreamType;
use stream_consumer::StreamConsumer;

pub struct StreamConverter<'a> {
    pub stream_src: StreamConsumer<'a>,
    pub stream_dest: StreamType,
}

impl<'a> StreamConverter<'a> {
    pub fn new(stream_src: StreamConsumer, stream_dest: StreamType) -> StreamConverter {
        StreamConverter {
            stream_src,
            stream_dest,
        }
    }
}

impl<'a> Iterator for StreamConverter<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        match self.stream_src.stream_type {
            StreamType::Varint => {
                match self.stream_dest {
                    StreamType::Varint => panic!("Won't convert bytevarint stream to itself"),
                    StreamType::Leb128 => unimplemented!(),
                    _ => panic!("Unsupported conversion"),
                }
            }
            StreamType::Single => panic!("Won't convert single stream"),
            StreamType::Leb128 => unimplemented!(),
            StreamType::Kafka => {
                match self.stream_dest {
                    StreamType::Varint => {
                        let kafka_consumer = self.stream_src.kafka_consumer.as_mut().unwrap();
                        match kafka_consumer.get_single_message() {
                            Some(ref mut x) => {
                                let mut lead_varint = encode_varint(x.len() as u64);
                                lead_varint.append(x);
                                Some(lead_varint)
                            }
                            None => None,
                        }
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
}
