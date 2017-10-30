#![deny(missing_docs)]

use error::*;

/// An enum type for byte streams
pub enum StreamType {
    /// Protobuf messages with leading length encoded in leb128
    Leb128,
    /// Protobuf messages with leading length encoded in varint
    Varint,
    /// Single protobuf messages with no separators/delimiters
    Single,
}

/// Convert &str to associated StreamType
pub fn str_to_streamtype(input: &str) -> Result<StreamType> {
    match input {
        "single" => Ok(StreamType::Single),
        "varint" => Ok(StreamType::Varint),
        "leb128" => Ok(StreamType::Leb128),
        _ => Err(ErrorKind::InvalidStreamTypeError(input.to_string()))?,
    }
}
