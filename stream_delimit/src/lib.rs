use std::io::Read;

#[derive(Debug)]
pub enum StreamDelimitError {
    NoLeadingVarintError(),
}

pub enum StreamDelimiter {
    Varint(usize),
}

pub trait Parse {
    fn parse(&mut self, buf: &mut Read, size: &mut usize) -> Result<(), StreamDelimitError>;
}

impl Parse for StreamDelimiter {
    fn parse(&mut self, read: &mut Read, size: &mut usize) -> Result<(), StreamDelimitError> {
        match *self {
            StreamDelimiter::Varint(max_attempts) => {
                let mut varint_buf: Vec<u8> = Vec::with_capacity(max_attempts);
                for i in 0..max_attempts {
                    varint_buf.push(0u8);
                    read.read_exact(&mut varint_buf[i..]).unwrap();
                    if (varint_buf[i] & 0b10000000) >> 7 != 0x1 {
                        let mut concat: u64 = 0;
                        for i in (0..varint_buf.len()).rev() {
                            let i_ = i as u32;
                            concat += ((varint_buf[i] & 0b01111111) as u64) << i_*(8u32.pow(i_) - 1);
                        }
                        *size = concat as usize;
                        return Ok(());
                    }
                }
                return Err(StreamDelimitError::NoLeadingVarintError())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{StreamDelimiter, Parse};

    #[test]
    fn test_simple() {
        let mut buf: &[u8] = &[0x01];
        let mut size: usize = 0;
        let mut delimiter = StreamDelimiter::Varint(10);
        delimiter.parse(&mut buf, &mut size).unwrap();
        assert_eq!(1, size);
    }

    #[test]
    fn test_varint_delimiter_longer() {
        let mut buf: &[u8] = &[0xACu8, 0x02u8];
        let mut size: usize = 0;
        let mut delimiter = StreamDelimiter::Varint(10);
        delimiter.parse(&mut buf, &mut size).unwrap();
        assert_eq!(300, size);
    }
}
