use std::fmt;
use std::error::Error;

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
            DiscoveryError::NoHome => None,
            DiscoveryError::NoFdsetPath(_) => None,
            DiscoveryError::NoFiles(_) => None,
        }
    }
}
