use protobuf::CodedInputStream;
use error::PqrsError;

pub enum LengthDelimiter {
    U32(),
}

pub trait Parse {
    fn parse(&mut self, buf: &[u8], size: &mut usize) -> Result<(), PqrsError>;
}

impl Parse for LengthDelimiter {
    fn parse(&mut self, buf: &[u8], size: &mut usize) -> Result<(), PqrsError> {
        let mut instream = CodedInputStream::from_bytes(buf);
        match *self {
            LengthDelimiter::U32() => {
                *size = match instream.read_raw_varint32() {
                    Ok(y) => y as usize,
                    _ => return Err(PqrsError::NoLeadingVarintError()),
                };
            }
        }
        Ok(())
    }
}
