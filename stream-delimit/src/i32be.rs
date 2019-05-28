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
