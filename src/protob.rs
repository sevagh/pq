extern crate serde;
extern crate protobuf;
extern crate serde_protobuf;
extern crate serde_value;

use std::fs;
use std::io::Read;
use self::serde::de::Deserialize;
use self::serde_protobuf::descriptor::Descriptors;
use self::serde_protobuf::de::Deserializer;
use self::serde_value::Value;

pub fn process_single(read: &mut Read) {
    let mut buffer = Vec::new();
    read.read(&mut buffer).unwrap();

    let mut byte_is = protobuf::CodedInputStream::from_bytes(&buffer);

    let mut file = fs::File::open("testdata/address.fdset").unwrap();

    let proto = protobuf::parse_from_reader(&mut file).unwrap();

    let descriptors = Descriptors::from_proto(&proto);
    let mut deserializer = Deserializer::for_named_message(&descriptors, "pq-rs", byte_is).unwrap();
    let value = Value::deserialize(&mut deserializer).unwrap();
    println!("{:?}", value);
}

pub fn process_stream(read: &mut Read) {
    let mut stream = protobuf::stream::CodedInputStream::new(read);

    loop {
        match stream.eof() {
            Err(e) => panic!(e),
            Ok(true) => break,
            Ok(false) => break, 
            //todo: actually do deserialization here
        }
    }
}
