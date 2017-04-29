use std::fmt;
use protobuf::ProtobufError;
use std::error::Error;
use serde_protobuf;
use fdset_discovery::error::DiscoveryError;

#[derive(Debug)]
pub enum PqrsError {
    FdsetDiscoveryError(DiscoveryError),
    DecodeError(DecodeError),
}

#[derive(Debug)]
pub enum DecodeError {
    NoSuccessfulAttempts,
    ProtobufError(ProtobufError),
    SerdeProtobufError(serde_protobuf::error::Error),
}

impl fmt::Display for PqrsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PqrsError::FdsetDiscoveryError(ref err) => err.fmt(f),
            PqrsError::DecodeError(ref err) => err.fmt(f),
        }
    }
}

impl Error for PqrsError {
    fn description(&self) -> &str {
        match *self {
            PqrsError::FdsetDiscoveryError(ref err) => err.description(),
            PqrsError::DecodeError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            PqrsError::FdsetDiscoveryError(ref err) => Some(err),
            PqrsError::DecodeError(ref err) => Some(err),
        }
    }
}

impl From<DiscoveryError> for PqrsError {
    fn from(err: DiscoveryError) -> PqrsError {
        PqrsError::FdsetDiscoveryError(err)
    }
}

impl From<DecodeError> for PqrsError {
    fn from(err: DecodeError) -> PqrsError {
        PqrsError::DecodeError(err)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::NoSuccessfulAttempts => write!(f, "Couldn't decode with any message descriptor"),
            DecodeError::ProtobufError(ref err) => err.fmt(f),
            DecodeError::SerdeProtobufError(ref err) => err.fmt(f),
        }
    }
}

impl Error for DecodeError {
    fn description(&self) -> &str {
        match *self {
            DecodeError::NoSuccessfulAttempts => "no suitable message descriptor",
            DecodeError::ProtobufError(ref err) => err.description(),
            DecodeError::SerdeProtobufError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DecodeError::NoSuccessfulAttempts => None,
            DecodeError::ProtobufError(ref err) => Some(err),
            DecodeError::SerdeProtobufError(ref err) => Some(err),
        }
    }
}
