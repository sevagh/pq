use std::fmt;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum StreamDelimitError {
    KafkaInitializeError,
    VarintDecodeError(io::Error),
    VarintDecodeMaxBytesError,
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
        }
    }
}

impl Error for StreamDelimitError {
    fn description(&self) -> &str {
        match *self {
            StreamDelimitError::KafkaInitializeError => "couldn't initialize kafka consumer",
            StreamDelimitError::VarintDecodeError(_) |
            StreamDelimitError::VarintDecodeMaxBytesError => "couldn't decode leading varint",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            StreamDelimitError::KafkaInitializeError |
            StreamDelimitError::VarintDecodeMaxBytesError => None,
            StreamDelimitError::VarintDecodeError(ref e) => Some(e),
        }
    }
}
