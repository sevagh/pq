extern crate byteorder;
#[cfg(feature = "with_kafka")]
extern crate rdkafka;

mod varint;

pub mod error;

/// Utilities to consume from a byte stream
pub mod byte_consumer;

/// Utilities to convert between stream types
pub mod converter;

/// Define stream types
pub mod stream;

#[cfg(feature = "with_kafka")]
/// Utilities to consume from Kafka
pub mod kafka_consumer;
