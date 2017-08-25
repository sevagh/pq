use error::StreamDelimitError;

pub enum StreamType {
    Leb128,
    Varint,
    Single,
}

pub fn str_to_streamtype(input: &str) -> Result<StreamType, StreamDelimitError> {
    match input {
        "single" => Ok(StreamType::Single),
        "varint" => Ok(StreamType::Varint),
        "leb128" => Ok(StreamType::Leb128),
        _ => Err(StreamDelimitError::InvalidStreamTypeError),
    }
}
