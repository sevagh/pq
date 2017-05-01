use std::io::Read;

const MAX_ATTEMPTS: usize = 10;

pub struct StreamDelimiter<'a> {
    delim: &'a str,
    read: &'a mut Read,
    trail: Option<String>,
}

impl<'a> StreamDelimiter<'a> {
    pub fn new(delim: &'a str, read: &'a mut Read, trail: Option<String>) -> StreamDelimiter<'a> {
        StreamDelimiter {
            delim: delim,
            read: read,
            trail: trail,
        }
    }
}

impl<'a> Iterator for StreamDelimiter<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let trail = self.trail.clone();
        let ret = match self.delim {
            "varint" => {
                let mut varint_buf: Vec<u8> = Vec::new();
                for i in 0..MAX_ATTEMPTS {
                    varint_buf.push(0u8);
                    match self.read.read_exact(&mut varint_buf[i..]) {
                        Ok(_) => (),
                        Err(_) => return None,
                    }
                    if (varint_buf[i] & 0b10000000) >> 7 != 0x1 {
                        let mut concat: u64 = 0;
                        for i in (0..varint_buf.len()).rev() {
                            let i_ = i as u32;
                            concat += ((varint_buf[i] & 0b01111111) as u64) <<
                                      (i_ * (8u32.pow(i_) - 1));
                        }
                        let mut msg_buf = vec![0; concat as usize];
                        self.read.read_exact(&mut msg_buf).unwrap();
         
                        return Some(msg_buf);
                    }
                }
                None
            }
            "leb128" => unimplemented!(),
            _ => panic!("Unknown delimiter"),
        };
        if let Some(x) = trail {
            let mut trail_buf: Vec<u8> = vec![0u8; x.len()];
            match self.read.read_exact(&mut trail_buf) {
                Ok(_) => (),
                Err(_) => return None,
            }
        }
        ret
    }
}

/*
#[cfg(test)]
mod tests {
    use super::{StreamDelimiter, Parse};

    #[test]
    fn test_simple() {
        let mut buf: &[u8] = &[0x01];
        let mut size: usize = 0;
        let mut delimiter = StreamDelimiter::Varint();
        delimiter.parse(&mut buf, &mut size).unwrap();
        assert_eq!(1, size);
    }

    #[test]
    fn test_varint_delimiter_longer() {
        let mut buf: &[u8] = &[0xACu8, 0x02u8];
        let mut size: usize = 0;
        let mut delimiter = StreamDelimiter::new("varint", &mut buf, None);
        delimiter.parse(&mut buf, &mut size).unwrap();
        assert_eq!(300, size);
    }
}
*/
