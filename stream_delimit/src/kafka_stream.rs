use stream_type::StreamType;
use stream_consumer::StreamConsumer;
use kafka::consumer::{Consumer, FetchOffset};
use error::StreamDelimitError;

pub struct KafkaConsumer {
    consumer: Consumer,
    messages: Vec<Vec<u8>>,
}

impl<'a> StreamConsumer <'a> {
    pub fn for_kafka(brokers: String,
                     topic: String,
                     from_beginning: bool)
                     -> Result<StreamConsumer<'a>, StreamDelimitError> {
        let kfc = match KafkaConsumer::new(brokers.as_str(), topic.as_str(), from_beginning) {
            Ok(kfc) => kfc,
            Err(e) => return Err(e),
        };
        Ok(StreamConsumer {
               stream_type: StreamType::Kafka,
               read: None,
               kafka_consumer: Some(kfc),
           })
    }
}

impl KafkaConsumer {
    pub fn new(brokers: &str,
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

    pub fn get_single_message(&mut self) -> Option<Vec<u8>> {
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
