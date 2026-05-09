#![deny(missing_docs)]

use crate::error::*;
use std::io::Read;

const VARINT_MAX_BYTES: usize = 10;

pub fn decode_varint(read: &mut dyn Read) -> Result<u64> {
    let mut varint_buf: Vec<u8> = Vec::new();
    for i in 0..VARINT_MAX_BYTES {
        varint_buf.push(0u8);
        match read.read_exact(&mut varint_buf[i..]) {
            Ok(_) => (),
            Err(e) => return Err(StreamDelimitError::VarintDecodeError(e)),
        }
        if (varint_buf[i] & 0x80) == 0 {
            let mut concat: u64 = 0;
            for (j, &byte) in varint_buf[..=i].iter().enumerate() {
                concat |= u64::from(byte & 0x7f) << (j * 7);
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
    use std::io::Cursor;

    #[test]
    fn test_simple() {
        assert_eq!(
            1,
            decode_varint(&mut Cursor::new(encode_varint(1))).unwrap()
        );
    }

    #[test]
    fn test_two_byte_varint() {
        assert_eq!(
            300,
            decode_varint(&mut Cursor::new(encode_varint(300))).unwrap()
        );
    }

    #[test]
    fn test_decode_known_varint_bytes() {
        let cases: &[(u64, &[u8])] = &[
            (0, &[0x00]),
            (1, &[0x01]),
            (127, &[0x7f]),
            (128, &[0x80, 0x01]),
            (300, &[0xac, 0x02]),
            (16384, &[0x80, 0x80, 0x01]),
            (2097152, &[0x80, 0x80, 0x80, 0x01]),
            (u32::MAX as u64, &[0xff, 0xff, 0xff, 0xff, 0x0f]),
            (
                u64::MAX,
                &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
            ),
        ];

        for &(expected, bytes) in cases {
            assert_eq!(expected, decode_varint(&mut Cursor::new(bytes)).unwrap());
        }
    }

    #[test]
    fn test_three_byte_varint() {
        // 16384 requires 3 bytes: 0x80 0x80 0x01
        // This would fail with the old buggy shift formula
        assert_eq!(
            16384,
            decode_varint(&mut Cursor::new(encode_varint(16384))).unwrap()
        );
        assert_eq!(
            100000,
            decode_varint(&mut Cursor::new(encode_varint(100000))).unwrap()
        );
    }

    #[test]
    fn test_four_byte_varint() {
        // 2097152 requires 4 bytes
        assert_eq!(
            2097152,
            decode_varint(&mut Cursor::new(encode_varint(2097152))).unwrap()
        );
        assert_eq!(
            10000000,
            decode_varint(&mut Cursor::new(encode_varint(10000000))).unwrap()
        );
    }

    #[test]
    fn test_large_varints() {
        assert_eq!(
            u32::MAX as u64,
            decode_varint(&mut Cursor::new(encode_varint(u32::MAX as u64))).unwrap()
        );
        assert_eq!(
            u64::MAX,
            decode_varint(&mut Cursor::new(encode_varint(u64::MAX))).unwrap()
        );
    }
}
