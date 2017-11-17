#![deny(missing_docs)]

use varint::encode_varint;
use stream::*;

/// A Converter struct to convert from a stream iterator to another `StreamType`
/// Useful for example to dump Kafka messages to a varint-delimited text file
pub struct Converter<'a> {
    stream_src: Box<&'a mut Iterator<Item = Vec<u8>>>,
    stream_dest: StreamType,
}

impl<'a> Converter<'a> {
    /// Return a converter from a stream iterator
    pub fn new<T: Iterator<Item = Vec<u8>>>(
        stream_src: &'a mut T,
        stream_dest: StreamType,
    ) -> Converter<'a> {
        Converter {
            stream_src: Box::new(stream_src),
            stream_dest,
        }
    }
}

impl<'a> Iterator for Converter<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        match self.stream_dest {
            StreamType::Varint |
            StreamType::Leb128 => {
                match self.stream_src.next() {
                    Some(ref mut x) => {
                        let mut lead_varint = encode_varint(x.len() as u64);
                        lead_varint.append(x);
                        Some(lead_varint)
                    }
                    None => None,
                }
            }
            _ => unimplemented!(),
        }
    }
}
