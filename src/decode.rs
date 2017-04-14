use protob::PqrsDecoder;
use std::io::{Read, Write};
use protobuf::{CodedInputStream, parse_from_reader, ProtobufResult};
use error::PqrsError;
use std::result::Result;
use serde::Deserialize;
use serde_protobuf::de::Deserializer;
use serde_protobuf::descriptor::Descriptors;
use serde_value::Value;

pub fn decode_single(pqrs_decoder: &PqrsDecoder,
                     buf: &[u8],
                     mut out: &mut Write,
                     force: bool)
                     -> Result<(), PqrsError> {
    if !force {
        return pqrs_decoder.decode_message(buf, &mut out);
    }
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

pub fn decode_size(lead: &[u8], size: &mut u32) -> Result<(), PqrsError> {
    let mut is = CodedInputStream::from_bytes(lead);
    *size = match is.read_raw_varint32() {
        Ok(x) => x,
        _ => return Err(PqrsError::NoLeadingVarintError()),
    };
    Ok(())
}
