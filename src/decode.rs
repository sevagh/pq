use discovery::discover_fdsets;
use docopt::Docopt;
use error::PqrsError;
use protob::PqrsDecoder;
use stream::decode_leading_varint;
use std::fs::File;
use std::io::{self, Write, Read, BufReader};
use std::process;

pub fn decode_single(pqrs_decoder: &PqrsDecoder,
                     buf: &[u8],
                     mut out: &mut Write)
                     -> Result<(), PqrsError> {
    let mut offset = 0;
    let buflen = buf.len();
    while offset < buflen {
        for n in 0..offset + 1 {
            if pqrs_decoder
                   .decode_message(&buf[n..(buflen - offset + n)], &mut out)
                   .is_ok() {
                return Ok(());
            }
        }
        offset += 1;
    }
    Err(PqrsError::CouldNotDecodeError())
}
