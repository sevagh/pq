use stream_consumer::StreamConsumer;
use stream_type::StreamType;

use std::io::Read;

impl<'a> StreamConsumer<'a> {
    pub fn for_byte(stream_type: StreamType, read: &'a mut Read) -> StreamConsumer<'a> {
        StreamConsumer {
            stream_type: stream_type,
            read: Some(read),
            kafka_consumer: None,
        }
    }
}
