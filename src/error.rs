#[derive(Debug)]
pub enum PqrsError {
    EmptyFdsetError(String),
    InitError(String),
    EofError(String),
    SerdeError(String),
    ProtobufError(String),
}
