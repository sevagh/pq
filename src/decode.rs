use protob::PqrsDecoder;
use std::io::Write;
use protobuf::{CodedInputStream, parse_from_reader};
use error::PqrsError;
use std::result::Result;
use serde::Deserialize;
use serde_protobuf::de::Deserializer;
use serde_protobuf::descriptor::Descriptors;
use serde_value::Value;

const LEADING_VARINT: &'static [u8] = b"
K
leading_varint.protoxyz.sevag.pqrs\"#
\rLeadingVarint
size (Rsize";

pub fn decode_single(pqrs_decoder: &PqrsDecoder,
                     buf: &[u8],
                     mut out: &mut Write,
                     force: bool)
                     -> Result<(), PqrsError> {
    match force {
        false => return pqrs_decoder.decode_message(buf, &mut out),
        true => (),
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

pub fn decode_leading_varint(lead: &[u8], resulting_size: &mut u64) -> Result<(), PqrsError> {
    println!("Contents of lead:\n\t{:?}", lead);
    let mut leading_varint = LEADING_VARINT.clone();

    let proto = parse_from_reader(&mut leading_varint).unwrap();
    let descriptors = Descriptors::from_proto(&proto);
    let byte_is = CodedInputStream::from_bytes(lead);

    let mut deserializer = Deserializer::for_named_message(&descriptors, ".xyz.sevag.pqrs.LeadingVarint", byte_is).unwrap();
    *resulting_size = match Value::deserialize(&mut deserializer) {
        Ok(Value::Map(x)) => {
            let val = match *x.values().nth(0).unwrap() {
                Value::U8(ref y) => *y as u64,
                Value::U16(ref y) => *y as u64,
                Value::U32(ref y) => *y as u64,
                Value::U64(ref y) => *y as u64,
                _ => {
                    return Err(PqrsError::NoLeadingVarintError());
                }
            };
            val
        }
        Ok(_) | Err(_) => return Err(PqrsError::CouldNotDecodeError()),
    };
    Ok(())
}
