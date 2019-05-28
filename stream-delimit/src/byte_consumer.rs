#![deny(missing_docs)]

use crate::i32be::consume_single_i32be;
use crate::stream::*;
use crate::varint::consume_single_varint;
use std::io::Read;

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
}

impl<T: Read> Iterator for ByteConsumer<T> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        match self.type_ {
            StreamType::Leb128 | StreamType::Varint => consume_single_varint(&mut self.read),
            StreamType::I32BE => consume_single_i32be(&mut self.read),
            StreamType::Single => {
                let ret: Option<Vec<u8>>;
                let mut buf = Vec::new();
                match self.read.read_to_end(&mut buf) {
                    Ok(x) => {
                        if x > 0 {
                            ret = Some(buf);
                        } else {
                            ret = None
                        }
                    }
                    Err(_) => ret = None,
                }
                ret
            }
        }
    }
}
