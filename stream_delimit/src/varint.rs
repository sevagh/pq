use error::StreamDelimitError;
use std::io::Read;

const VARINT_MAX_BYTES: usize = 10;

pub fn decode_varint(read: &mut Read) -> Result<u64, StreamDelimitError> {
    let mut varint_buf: Vec<u8> = Vec::new();
    for i in 0..VARINT_MAX_BYTES {
        varint_buf.push(0u8);
        match read.read_exact(&mut varint_buf[i..]) {
            Ok(_) => (),
            Err(e) => return Err(StreamDelimitError::VarintDecodeError(e)),
        }
        if (varint_buf[i] & 0b10000000) >> 7 != 0x1 {
            let mut concat: u64 = 0;
            for i in (0..varint_buf.len()).rev() {
                let i_ = i as u32;
                concat += ((varint_buf[i] & 0b01111111) as u64) << (i_ * (8u32.pow(i_) - 1));
            }
            return Ok(concat);
        }
    }
    Err(StreamDelimitError::VarintDecodeMaxBytesError)
}

pub fn encode_varint(mut value: u64) -> Vec<u8> {
    let mut ret = vec![0u8; VARINT_MAX_BYTES];
    let mut n = 0;
    while value > 127 {
        ret[n] = 0x80 | (value & 0x7F) as u8;
        value >>= 7;
        n += 1
    }
    ret[n] = value as u8;
    n += 1;
    ret[0..n].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut buf: &[u8] = &[0x01];
        assert_eq!(1, decode_varint(&mut buf).unwrap());
    }

    #[test]
    fn test_varint_delimiter_longer() {
        let mut buf: &[u8] = &[0xACu8, 0x02u8];
        assert_eq!(300, decode_varint(&mut buf).unwrap());
    }

    #[test]
    fn test_encode_simple() {
        assert_eq!(vec![0x01], encode_varint(1))
    }

    #[test]
    fn test_encode_longer() {
        assert_eq!(vec![0xACu8, 0x02u8], encode_varint(300))
    }
}
