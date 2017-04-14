use protobuf::{CodedInputStream, parse_from_reader};
use std::io::Read;
use error::PqrsError;
use discovery::LoadedDescriptors;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;
use serde::{Deserialize, Serialize};
use serde_json::ser::Serializer;
use serde_protobuf::de::Deserializer;
use serde_protobuf::error::{Error, ErrorKind};
use serde_protobuf::descriptor::{Descriptors, MessageDescriptor};
use serde_value::Value;
use protobuf::error::ProtobufError;


const LEADING_VARINT: &'static [u8] = b"
K
leading_varint.protoxyz.sevag.pqrs\"#
\rLeadingVarint
size (Rsize";

pub fn decode_leading_varint(lead: &[u8], resulting_size: &mut i32) -> Result<(), PqrsError> {
    let mut leading_varint = LEADING_VARINT.clone();

    let proto = parse_from_reader(&mut leading_varint).unwrap();
    let descriptors = Descriptors::from_proto(&proto);
    let byte_is = CodedInputStream::from_bytes(lead);

    let mut deserializer = Deserializer::for_named_message(&descriptors, ".xyz.sevag.pqrs.LeadingVarint", byte_is).unwrap();
   let value = match Value::deserialize(&mut deserializer) {
       Ok(x) => x,
       Err(_) => return Err(PqrsError::CouldNotDecodeError()),
    };
    println!("{:?}", value);
    *resulting_size = 1337;
    Ok(())
}
