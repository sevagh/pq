use std::env;
use std::fs::File;
use std::io::Read;
use serde::de::Deserialize;
use serde_protobuf::descriptor::Descriptors;
use serde_protobuf::de::Deserializer;
use serde_value::Value;
use protobuf::{CodedInputStream, parse_from_reader};

pub fn process_single(read: &mut Read) {
    let proto = parse_from_reader(&mut open_combined_fdset()).unwrap();
    let descriptors = Descriptors::from_proto(&proto);
    let byte_is = CodedInputStream::new(read);

    let mut deserializer = Deserializer::for_named_message(&descriptors, ".com.example.dog.Dog", byte_is).unwrap();
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

fn open_combined_fdset() -> File {
    let mut home = match env::home_dir() {
        Some(home) => home,
        None => panic!("Could not find $HOME"),
    };

    home.push(".pq/combined.fdset");

    match File::open(home.as_path()) {
        Ok(x) => return x,
        Err(e) => panic!(e),
    }
}
