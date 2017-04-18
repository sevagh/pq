use std::io::Read;

#[derive(Debug)]
pub enum StreamDelimitError {
    NoLeadingVarintError(),
}

pub enum StreamDelimiter {
    Varint(),
}

pub trait Parse {
    fn parse(&mut self, buf: &mut Read, size: &mut usize) -> Result<(), StreamDelimitError>;
}

impl Parse for StreamDelimiter {
    fn parse(&mut self, read: &mut Read, size: &mut usize) -> Result<(), StreamDelimitError> {
        match *self {
            StreamDelimiter::Varint() => {
                let mut varint_buf = Vec::new();
                let mut tmpbuf = vec![0; 1];
                loop {
                    read.read_exact(&mut tmpbuf).unwrap();
                    let chopped_msb = (tmpbuf[0] & 0b10000000) >> 7;
                    varint_buf.append(&mut tmpbuf.clone());
                    if chopped_msb != 0x1 as u8 {
                        let mut concat: u64 = 0;
                        for i in (0..varint_buf.len()).rev() {
                            let chop = varint_buf[i] & 0b01111111; //chop off msb
                            let shift_amount: u32 = (i as u32)*8u32.pow(i as u32) - 1*(i as u32); 
                            let shift: u64 = (chop as u64) << shift_amount;
                            concat = concat + shift;
                        }
                        *size = concat as usize;
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{StreamDelimiter, Parse};

    #[test]
    #[ignore]
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
        let mut delimiter = StreamDelimiter::Varint();
        delimiter.parse(&mut buf, &mut size).unwrap();
        assert_eq!(300, size);
    }
}
