extern crate protobuf;

use protobuf::CodedInputStream;

#[derive(Debug)]
pub enum StreamDelimitError {
    NoLeadingVarintError(),
}

pub enum StreamDelimiter {
    U32(),
}

pub trait Parse {
    fn parse(&mut self, buf: &[u8], size: &mut usize) -> Result<(), StreamDelimitError>;
}

impl Parse for StreamDelimiter {
    fn parse(&mut self, buf: &[u8], size: &mut usize) -> Result<(), StreamDelimitError> {
        let mut instream = CodedInputStream::from_bytes(buf);
        match *self {
            StreamDelimiter::U32() => {
                *size = match instream.read_raw_varint32() {
                    Ok(y) => y as usize,
                    _ => return Err(StreamDelimitError::NoLeadingVarintError()),
                };
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
