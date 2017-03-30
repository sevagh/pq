extern crate serde;
extern crate serde_protobuf;
extern crate serde_value;

use std::fs;
use std::io::Read;
use self::serde::de::Deserialize;
use self::serde_protobuf::descriptor::Descriptors;
use self::serde_protobuf::de::Deserializer;
use self::serde_value::Value;
use protobuf::{CodedInputStream, parse_from_reader};
use unknown::Unknown;

pub fn process_single(read: &mut Read) {
    let mut file = fs::File::open("testdata/address.fdset").unwrap();
    let proto = parse_from_reader(&mut file).unwrap();
    let descriptors = Descriptors::from_proto(&proto);

    let mut byte_is = CodedInputStream::new(read);

    let mut unknown = Unknown::new();
    let mut deserializer = Deserializer::new(&descriptors, &unknown, byte_is).unwrap();
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
