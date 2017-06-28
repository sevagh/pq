use varint::decode_varint;
use kafka::consumer::{Consumer, FetchOffset};
use error::StreamDelimitError;
use std;
use std::io::Read;

pub struct StreamConsumer<'a> {
    consumer: &'a mut GenericConsumer,
}

impl<'a> StreamConsumer<'a> {
    pub fn new(consumer: &'a mut GenericConsumer) -> StreamConsumer<'a> {
        StreamConsumer { consumer }
    }
}

pub trait GenericConsumer {
    fn get_single_message(&mut self) -> Option<Vec<u8>>;
}

pub struct KafkaConsumer {
    consumer: Consumer,
    messages: Vec<Vec<u8>>,
}

pub struct VarintConsumer<'a> {
    read: &'a mut Read,
}

pub struct SingleConsumer<'a> {
    read: &'a mut Read,
}

impl<'a> Iterator for StreamConsumer<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        self.consumer.get_single_message()
    }
}

impl KafkaConsumer {
    pub fn new(
        brokers: &str,
        topic: &str,
        from_beginning: bool,
    ) -> Result<KafkaConsumer, StreamDelimitError> {
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
        ).with_topic_partitions(topic.to_owned(), &[0, 1])
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
}

impl<'a> VarintConsumer<'a> {
    pub fn new(read: &'a mut Read) -> VarintConsumer {
        VarintConsumer { read }
    }
}

impl<'a> SingleConsumer<'a> {
    pub fn new(read: &'a mut Read) -> SingleConsumer {
        SingleConsumer { read }
    }
}

impl GenericConsumer for KafkaConsumer {
    fn get_single_message(&mut self) -> Option<Vec<u8>> {
        if self.messages.is_empty() {
            let kafka_consumer = &mut self.consumer;
            match kafka_consumer.poll() {
                Ok(x) => {
                    let x = x.iter().take(1).next().expect(
                        "Couldn't take 1 message from kafka stream",
                    );
                    self.messages.append(&mut x.messages()
                        .iter()
                        .map(|x| x.value.to_vec())
                        .collect::<Vec<_>>());
                    kafka_consumer.consume_messageset(x).expect(
                        "Couldn't mark messageset as consumed",
                    );
                }
                Err(_) => return None,
            }
            kafka_consumer.commit_consumed().expect(
                "Couldn't commit consumption",
            );
        }
        self.messages.pop()
    }
}

impl<'a> GenericConsumer for VarintConsumer<'a> {
    fn get_single_message(&mut self) -> Option<Vec<u8>> {
        let ret: Option<Vec<u8>>;
        match decode_varint(self.read) {
            Ok(x) => {
                let mut msg_buf = vec![0; x as usize];
                match self.read.read_exact(&mut msg_buf) {
                    Ok(_) => (),
                    Err(_) => return None,
                }
                ret = Some(msg_buf);
            }
            Err(_) => ret = None,
        }
        ret
    }
}

impl<'a> GenericConsumer for SingleConsumer<'a> {
    fn get_single_message(&mut self) -> Option<Vec<u8>> {
        let ret: Option<Vec<u8>>;
        let mut buf = Vec::new();
        match self.read.read_to_end(&mut buf) {
            Ok(x) => {
                if x > 0 {
                    ret = Some(buf);
                } else {
                    ret = None
                }
            }
            Err(_) => ret = None,
        }
        ret
    }
}

impl<'a> GenericConsumer for StreamConsumer<'a> {
    fn get_single_message(&mut self) -> Option<Vec<u8>> {
        self.consumer.get_single_message()
    }
}
