use varint::encode_varint;
use leb128::encode_leb128;
use stream::*;

pub struct Converter<'a> {
    stream_src: Box<&'a mut Iterator<Item = Vec<u8>>>,
    stream_dest: StreamType,
}

impl<'a> Converter<'a> {
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
            StreamType::Varint => {
                match self.stream_src.next() {
                    Some(ref mut x) => {
                        let mut lead_varint = encode_varint(x.len() as u64);
                        lead_varint.append(x);
                        Some(lead_varint)
                    }
                    None => None,
                }
            }
            StreamType::Leb128 => {
                match self.stream_src.next() {
                    Some(ref mut x) => {
                        let mut lead_leb128 = encode_leb128(x.len() as u64);
                        lead_leb128.append(x);
                        Some(lead_leb128)
                    }
                    None => None,
                }
            }
            _ => unimplemented!(),
        }
    }
}
