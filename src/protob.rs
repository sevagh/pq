extern crate serde;
extern crate serde_protobuf;
extern crate serde_value;

use std::io::Read;
use self::serde::de::Deserialize;
use self::serde_protobuf::descriptor::Descriptors;
use self::serde_protobuf::de::Deserializer;
use self::serde_value::Value;
use protobuf::{CodedInputStream, parse_from_reader};

pub fn process_single(read: &mut Read) {
    let mut unknown_fdset: &'static [u8] = b"
*
\runknown.protoxyz.sevag.pqrs\"	
Unknown";

    let proto = parse_from_reader(&mut unknown_fdset).unwrap();
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
