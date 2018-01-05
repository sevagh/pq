use std::fmt;
use std::error::Error;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, StreamDelimitError>;

#[derive(Debug)]
pub enum StreamDelimitError {
    #[cfg(feature="with_kafka")]
    KafkaInitializeError(::kafka::error::Error),
    VarintDecodeError(io::Error),
    InvalidStreamTypeError(String),
    VarintDecodeMaxBytesError,
}

impl fmt::Display for StreamDelimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            #[cfg(feature="with_kafka")]
            StreamDelimitError::KafkaInitializeError(ref e) => {
                write!(f, "Couldn't initialize kafka consumer: {}", e)
            }
            StreamDelimitError::VarintDecodeError(ref e) => {
                write!(f, "Couldn't decode leading varint: {}", e)
            }
            StreamDelimitError::InvalidStreamTypeError(ref t) => {
                write!(f, "Invalid stream type: {} (only support single,leb128,varint)", t)
            }
            StreamDelimitError::VarintDecodeMaxBytesError => {
                write!(f, "Exceeded max attempts to decode leading varint")
            }
        }
    }
}

impl Error for StreamDelimitError {
    fn description(&self) -> &str {
        match *self {
            #[cfg(feature="with_kafka")]
            StreamDelimitError::KafkaInitializeError => "couldn't initialize kafka consumer",
            StreamDelimitError::VarintDecodeError(_) => "couldn't decode leading varint",
            StreamDelimitError::InvalidStreamTypeError(_) => "invalid stream type",
            StreamDelimitError::VarintDecodeMaxBytesError => "couldn't decode leading varint",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            #[cfg(feature="with_kafka")]
            StreamDelimitError::KafkaInitializeError(ref e) => Some(e),
            StreamDelimitError::VarintDecodeError(ref e) => Some(e),
            StreamDelimitError::InvalidStreamTypeError(_) => None,
            StreamDelimitError::VarintDecodeMaxBytesError => None,
        }
    }
}
