use std::fmt;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum StreamDelimitError {
    KafkaInitializeError,
    VarintDecodeError(io::Error),
    VarintDecodeMaxBytesError,
    InvalidStreamTypeError,
}

impl fmt::Display for StreamDelimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StreamDelimitError::KafkaInitializeError => {
                write!(f, "Couldn't initialize kafka consumer")
            }
            StreamDelimitError::VarintDecodeError(ref e) => {
                write!(f, "Couldn't decode leading varint: {}", e)
            }
            StreamDelimitError::VarintDecodeMaxBytesError => {
                write!(f, "Exceeded max attempts to decode leading varint")
            }
            StreamDelimitError::InvalidStreamTypeError => {
                write!(f, "Only supports stream types single, varint, leb128")
            }
        }
    }
}

impl Error for StreamDelimitError {
    fn description(&self) -> &str {
        match *self {
            StreamDelimitError::KafkaInitializeError => "couldn't initialize kafka consumer",
            StreamDelimitError::VarintDecodeError(_) |
            StreamDelimitError::VarintDecodeMaxBytesError => "couldn't decode leading varint",
            StreamDelimitError::InvalidStreamTypeError => {
                "only supports stream types single, varint, leb128"
            }
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            StreamDelimitError::KafkaInitializeError |
            StreamDelimitError::InvalidStreamTypeError |
            StreamDelimitError::VarintDecodeMaxBytesError => None,
            StreamDelimitError::VarintDecodeError(ref e) => Some(e),
        }
    }
}
