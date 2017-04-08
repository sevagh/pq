use std::io::Write;
use error::PqrsError;
use protob::PqrsDecoder;

pub fn forcefully_decode(pqrs_decoder: &PqrsDecoder,
                         buf: &[u8],
                         mut out: &mut Write)
                         -> Result<(), PqrsError> {
    let mut offset = 0;
    let buflen = buf.len();
    while offset < buflen {
        for n in 0..offset + 1 {
            match pqrs_decoder.decode_message(&buf[n..(buflen - offset + n)], &mut out) {
                Ok(_) => return Ok(()),
                Err(_) => (),
            }
        }
        offset += 1;
    }
    Err(PqrsError::CouldNotDecodeError(String::from("Could not decode")))
}
