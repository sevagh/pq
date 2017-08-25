use error::*;

pub enum StreamType {
    Leb128,
    Varint,
    Single,
}

pub fn str_to_streamtype(input: &str) -> Result<StreamType> {
    match input {
        "single" => Ok(StreamType::Single),
        "varint" => Ok(StreamType::Varint),
        "leb128" => Ok(StreamType::Leb128),
        _ => Err(ErrorKind::InvalidStreamTypeError(input.to_string()))?,
    }
}
