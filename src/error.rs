use std::fmt;
use protobuf::ProtobufError;
use std::error::Error;
use serde_protobuf;

#[derive(Debug)]
pub enum PqrsError {
    FdsetDiscoverError(DiscoveryError),
    FdsetLoadError(),
    DecodeError(DecodeError),
}

#[derive(Debug)]
pub enum DiscoveryError {
    NoHome,
    NoFdsetPath(&'static str),
    NoFiles(&'static str),
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
            PqrsError::FdsetDiscoverError(ref err) => err.fmt(f),
            PqrsError::DecodeError(ref err) => err.fmt(f),
            PqrsError::FdsetLoadError() => write!(f, "No loadable fdset files found"),
        }
    }
}

impl Error for PqrsError {
    fn description(&self) -> &str {
        match *self {
            PqrsError::FdsetDiscoverError(ref err) => err.description(),
            PqrsError::DecodeError(ref err) => err.description(),
            PqrsError::FdsetLoadError() => "no loadable fdsets",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            PqrsError::FdsetDiscoverError(ref err) => Some(err),
            PqrsError::DecodeError(ref err) => Some(err),
            PqrsError::FdsetLoadError() => None,
        }
    }
}

impl From<DiscoveryError> for PqrsError {
    fn from(err: DiscoveryError) -> PqrsError {
        PqrsError::FdsetDiscoverError(err)
    }
}

impl From<DecodeError> for PqrsError {
    fn from(err: DecodeError) -> PqrsError {
        PqrsError::DecodeError(err)
    }
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DiscoveryError::NoHome => write!(f, "$HOME not defined"),
            DiscoveryError::NoFdsetPath(ref path) => write!(f, "Path {} doesn't exist", path),
            DiscoveryError::NoFiles(ref path) => write!(f, "No files in path {}", path),
        }
    }
}

impl Error for DiscoveryError {
    fn description(&self) -> &str {
        match *self {
            DiscoveryError::NoHome => "$HOME not defined",
            DiscoveryError::NoFdsetPath(ref path) => format!("Path {} doesn't exist", path).as_str(),
            DiscoveryError::NoFiles(ref path) => format!("No files in path {}", path).as_str(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DiscoveryError::NoHome => None,
            DiscoveryError::NoFdsetPath(_) => None,
            DiscoveryError::NoFiles(_) => None,
        }
    }
}

/*
#[derive(Debug)]
pub enum DecodeError {
    NoSuccessfulAttempts,
    ProtobufError(ProtobufError),
    SerdeProtobufError(serde_protobuf::error::Error),
}

impl Error for DiscoveryError {
    fn description(&self) -> &str {
        match *self {
            DiscoveryError::NoHome => "$HOME not defined",
            DiscoveryError::NoFdsetPath(ref path) => format!("Path {} doesn't exist", path).as_str(),
            DiscoveryError::NoFiles(ref path) => format!("No files in path {}", path).as_str(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DiscoveryError::NoHome => None,
            DiscoveryError::NoFdsetPath(_) => None,
            DiscoveryError::NoFiles(_) => None,
        }
    }
}


*/

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
