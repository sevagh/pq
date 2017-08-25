#[cfg(feature = "with_kafka")]
extern crate kafka;

#[macro_use]
extern crate error_chain;

mod varint;
mod leb128;

pub mod error;
pub mod byte_consumer;
pub mod converter;
pub mod stream;

#[cfg(feature = "with_kafka")]
pub mod kafka_consumer;
