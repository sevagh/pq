use varint::consume_single_varint;
use stream::*;
use std::io::Read;

pub struct ByteConsumer<'a> {
    read: &'a mut Read,
    type_: StreamType,
}

impl<'a> ByteConsumer<'a> {
    pub fn new(read: &'a mut Read, type_: StreamType) -> ByteConsumer {
        ByteConsumer { read, type_ }
    }
}

impl<'a> Iterator for ByteConsumer<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        match self.type_ {
            StreamType::Leb128 => consume_single_varint(self.read),
            StreamType::Varint => consume_single_varint(self.read),
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
