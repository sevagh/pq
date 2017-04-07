#[derive(Debug)]
pub enum PqrsError {
    NoContenderError(String),
    EmptyFdsetError(String),
    InitError(String),
    EofError(String),
    SerdeError(String),
    ProtobufError(String),
}
