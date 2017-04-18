#[derive(Debug)]
pub enum PqrsError {
    NoContenderError(),
    EmptyFdsetError(),
    CouldNotDecodeError(),
    EofError(),
    InitError(String),
    SerdeError(String),
    ProtobufError(String),
}
