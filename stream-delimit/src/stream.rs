#![deny(missing_docs)]

use std::io;

use crate::error::*;

/// An enum type for byte streams
#[derive(PartialEq, Eq)]
pub enum StreamType {
    /// Protobuf messages with leading length encoded in leb128
    Leb128,
    /// Protobuf messages with leading length encoded in varint
    Varint,
    /// Protobuf messages with leading length encoded as
    /// binary big endian 32-bit signed integer
    I32BE,
    /// Single protobuf messages with no separators/delimiters
    Single,
}

/// Convert &str to associated `StreamType`
pub fn str_to_streamtype(input: &str) -> Result<StreamType> {
    match input {
        "single" => Ok(StreamType::Single),
        "varint" => Ok(StreamType::Varint),
        "leb128" => Ok(StreamType::Leb128),
        "i32be" => Ok(StreamType::I32BE),
        _ => Err(StreamDelimitError::InvalidStreamTypeError(
            input.to_string(),
        )),
    }
}

/// A trait for a stream that can be read in clearly defined chunks
pub trait FramedRead {
    /// should read the next available frame into the provided buffer.
    /// clear() will be called before the buffer is filled
    fn read_next_frame<'a>(&mut self, buffer: &'a mut Vec<u8>) -> io::Result<Option<&'a [u8]>>;
}
