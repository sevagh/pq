#[derive(Debug)]
pub enum DiscoveryError {
    Error(String),
}

#[derive(Debug)]
pub enum LoadFdsetError {
    Error(String),
}

#[derive(Debug)]
pub enum DecodeError {
    Error(String),
}
