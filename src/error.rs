#[derive(Debug)]
pub enum PqrsError {
    NoLeadingVarintError(),
    NoContenderError(),
    EmptyFdsetError(),
    CouldNotDecodeError(),
    EofError(),
    InitError(String),
    SerdeError(String),
    ProtobufError(String),
}
