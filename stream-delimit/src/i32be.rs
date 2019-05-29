#![deny(missing_docs)]

use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

pub fn consume_single_i32be(read: &mut Read) -> Option<Vec<u8>> {
    match read.read_i32::<BigEndian>() {
        Ok(length) => {
            let mut msg_buf = vec![0; length as usize];
            match read.read_exact(&mut msg_buf) {
                Ok(_) => Some(msg_buf),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::WriteBytesExt;
    use std::io::Cursor;

    fn reads_back(x: i32) {
        let mut buf = vec![];
        buf.write_i32::<BigEndian>(x).unwrap();
        assert_eq!(x, Cursor::new(buf).read_i32::<BigEndian>().unwrap());
    }

    #[test]
    fn test_simple() {
        reads_back(1);
    }

    #[test]
    fn test_delimiter_longer() {
        reads_back(300);
    }
}
