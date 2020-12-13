#![deny(missing_docs)]

use crate::stream::*;
use crate::{error::StreamDelimitError, varint::decode_varint};
use byteorder::{BigEndian, ReadBytesExt};
use std::{
    io::{self, Read},
    num::NonZeroUsize,
};

/// A consumer for a byte stream
pub struct ByteConsumer<T: Read> {
    read: T,
    type_: StreamType,
}

impl<T: Read> ByteConsumer<T> {
    /// Return a ByteConsumer from for single messages, varint or leb128-delimited
    pub fn new(read: T, type_: StreamType) -> ByteConsumer<T> {
        ByteConsumer { read, type_ }
    }

    fn read_next_frame_length(&mut self) -> io::Result<Option<NonZeroUsize>> {
        let r = match self.type_ {
            StreamType::Leb128 | StreamType::Varint => decode_varint(&mut self.read)
                .map_err(|e| {
                    // For unified error handling we force everything into io::Error
                    match e {
                        StreamDelimitError::VarintDecodeError(i) => i,
                        e => io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)),
                    }
                })
                .map(|v| NonZeroUsize::new(v as usize)),
            StreamType::I32BE => self
                .read
                .read_i32::<BigEndian>()
                .map(|v| NonZeroUsize::new(v as usize)),
            StreamType::Single => Ok(None),
        };

        // In the cases where we have hit the end of the stream, read_i32 will return UnexpectedEof
        // we treat this as no more data
        match r {
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
            a => a,
        }
    }
}

impl<T: Read> Iterator for ByteConsumer<T> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = Vec::new();
        self.read_next_frame(&mut buffer).ok()??;
        Some(buffer)
    }
}

impl<T: Read> FramedRead for ByteConsumer<T> {
    fn read_next_frame<'a>(&mut self, buffer: &'a mut Vec<u8>) -> io::Result<Option<&'a [u8]>> {
        let r = match self.read_next_frame_length()? {
            Some(length) => {
                buffer.clear();
                let mut take = (&mut self.read).take(length.get() as u64);
                take.read_to_end(buffer)?;
                Some(&buffer[..])
            }
            // the single stream type does not have a defined length, so read_next_frame_length will return None
            // and we catch that special case here
            None if self.type_ == StreamType::Single => {
                buffer.clear();
                if self.read.read_to_end(buffer)? > 0 {
                    Some(&buffer[..])
                } else {
                    None
                }
            }
            _ => None,
        };
        Ok(r)
    }
}
