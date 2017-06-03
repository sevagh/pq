use std::fmt;
use protobuf::ProtobufError;
use std::error::Error;
use serde_protobuf;
use serde_json;

#[derive(Debug)]
pub enum DiscoveryError {
    NoHome,
    NoFdsetPath(String),
    NoFiles(String),
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DiscoveryError::NoHome => write!(f, "$HOME not defined"),
            DiscoveryError::NoFdsetPath(ref path) => write!(f, "Path {} doesn't exist", path),
            DiscoveryError::NoFiles(ref path) => write!(f, "No valid fdset files in path {}", path),
        }
    }
}

impl Error for DiscoveryError {
    fn description(&self) -> &str {
        match *self {
            DiscoveryError::NoHome => "$HOME not defined",
            DiscoveryError::NoFdsetPath(_) => "fdset_path doesn't exist",
            DiscoveryError::NoFiles(_) => "no files in fdset_path",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DiscoveryError::NoHome |
            DiscoveryError::NoFdsetPath(_) |
            DiscoveryError::NoFiles(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum PqrsError {
    FdsetDiscoveryError(DiscoveryError),
    DecodeError(DecodeError),
    ArgumentError,
}

#[derive(Debug)]
pub enum DecodeError {
    ProtobufError(ProtobufError),
    SerdeProtobufError(serde_protobuf::error::Error),
    SerializeError(serde_json::Error),
}

impl fmt::Display for PqrsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PqrsError::ArgumentError => writeln!(f, "Invalid arguments"),
            PqrsError::FdsetDiscoveryError(ref err) => err.fmt(f),
            PqrsError::DecodeError(ref err) => err.fmt(f),
        }
    }
}

impl Error for PqrsError {
    fn description(&self) -> &str {
        match *self {
            PqrsError::ArgumentError => "Invalid arguments",
            PqrsError::FdsetDiscoveryError(ref err) => err.description(),
            PqrsError::DecodeError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            PqrsError::ArgumentError => None,
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
            DecodeError::ProtobufError(ref err) => err.fmt(f),
            DecodeError::SerdeProtobufError(ref err) => err.fmt(f),
            DecodeError::SerializeError(ref err) => err.fmt(f),
        }
    }
}

impl Error for DecodeError {
    fn description(&self) -> &str {
        match *self {
            DecodeError::ProtobufError(ref err) => err.description(),
            DecodeError::SerdeProtobufError(ref err) => err.description(),
            DecodeError::SerializeError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DecodeError::ProtobufError(ref err) => Some(err),
            DecodeError::SerdeProtobufError(ref err) => Some(err),
            DecodeError::SerializeError(ref err) => Some(err),
        }
    }
}
