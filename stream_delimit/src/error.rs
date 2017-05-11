use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum StreamDelimitError {
    KafkaInitializeError,
}

impl fmt::Display for StreamDelimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StreamDelimitError::KafkaInitializeError => {
                write!(f, "Couldn't initialize kafka consumer")
            }
        }
    }
}

impl Error for StreamDelimitError {
    fn description(&self) -> &str {
        match *self {
            StreamDelimitError::KafkaInitializeError => "couldn't initialize kafka consumer",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            StreamDelimitError::KafkaInitializeError => None,
        }
    }
}
