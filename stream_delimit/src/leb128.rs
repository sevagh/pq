use error::*;
use std::io::Read;

const LEB128_MAX_BYTES: usize = 10;

pub fn consume_single_leb128(read: &mut Read) -> Option<Vec<u8>> {
    let ret: Option<Vec<u8>>;
    match decode_leb128(read) {
        Ok(x) => {
            let mut msg_buf = vec![0; x as usize];
            match read.read_exact(&mut msg_buf) {
                Ok(_) => (),
                Err(_) => return None,
            }
            ret = Some(msg_buf);
        }
        Err(_) => ret = None,
    }
    ret
}

pub fn decode_leb128(read: &mut Read) -> Result<u64> {
    let mut varint_buf: Vec<u8> = Vec::new();
    let mut shift = 0;
    let mut acc = 0;

    for i in 0..LEB128_MAX_BYTES {
        varint_buf.push(0u8);
        match read.read_exact(&mut varint_buf[i..]) {
            Ok(_) => (),
            Err(e) => return Err(ErrorKind::Leb128DecodeError(e))?,
        }
        let b = varint_buf[i];
        acc |= ((b & 0x7f) as u64) << shift;
        shift += 7;
        if 0 == b & 0x80 {
            return Ok(acc);
        }
    }
    Err(ErrorKind::Leb128DecodeMaxBytesError)?
}

pub fn encode_leb128(value: u64) -> Vec<u8> {
    let mut ret = vec![0u8; LEB128_MAX_BYTES];
    let mut n = 0;
    let mut v: u64 = value.into();
    loop {
        ret[n] = ((v & 0x7f) | if v > 127 { 0x80 } else { 0 }) as u8;
        v >>= 7;
        if 0 == v {
            break;
        }
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
        assert_eq!(1, decode_leb128(&mut buf).unwrap());
    }

    #[test]
    fn test_leb128_delimiter_longer() {
        let mut buf: &[u8] = &[0xACu8, 0x02u8];
        assert_eq!(300, decode_leb128(&mut buf).unwrap());
    }

    #[test]
    fn test_encode_simple() {
        assert_eq!(vec![0x01], encode_leb128(1))
    }

    #[test]
    fn test_encode_longer() {
        assert_eq!(vec![0xACu8, 0x02u8], encode_leb128(300))
    }
}
