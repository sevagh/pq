use varint::encode_varint;
use consumer::GenericConsumer;

pub struct StreamConverter<'a> {
    stream_src: &'a mut GenericConsumer,
    stream_dest: &'a str,
}

impl<'a> StreamConverter<'a> {
    pub fn new(stream_src: &'a mut GenericConsumer, stream_dest: &'a str) -> StreamConverter<'a> {
        StreamConverter {
            stream_src,
            stream_dest,
        }
    }
}

impl<'a> Iterator for StreamConverter<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        match self.stream_dest {
            "varint" => {
                match self.stream_src.get_single_message() {
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
