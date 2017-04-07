#[derive(Debug)]
pub enum PqrsError {
    InitError(String),
    EofError(String),
    SerdeError(String),
    ProtobufError(String),
}
