use std::io::Read;
use serde::de::Deserialize;
use serde_protobuf::descriptor::Descriptors;
use serde_protobuf::de::Deserializer;
use serde_protobuf::value::Message;
use serde_value::Value;
use protobuf::{CodedInputStream, parse_from_reader};

static UNKNOWN_FDSET: &'static [u8] = b"
*
\runknown.protoxyz.sevag.pqrs\"	
Unknown";

pub fn process_single(read: &mut Read) {
    let proto = parse_from_reader(&mut UNKNOWN_FDSET.clone()).unwrap();
    let descriptors = Descriptors::from_proto(&proto);

    let byte_is = CodedInputStream::new(read);

    let mut deserializer = Deserializer::for_named_message(&descriptors, ".xyz.sevag.pqrs.Unknown", byte_is).unwrap();
    let value = Value::deserialize(&mut deserializer).unwrap();
    println!("{:?}", value);
}

pub fn process_stream(read: &mut Read) {
    let mut stream = CodedInputStream::new(read);

    loop {
        match stream.eof() {
            Err(e) => panic!(e),
            Ok(true) => break,
            Ok(false) => break, 
            //todo: actually do deserialization here
        }
    }
}
