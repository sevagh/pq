extern crate protobuf;

mod schemata;

use protobuf::Message;
use protobuf::CodedInputStream;
use protobuf::CodedOutputStream;

use schemata::addressbook::Person;

use std::fs::File;
use std::io::{self, Read, Write};

#[cfg(test)]
#[test]
fn decode_basic() {
    let mut _in_handle = io::stdin();
    let mut in_handle = _in_handle.lock();

    let mut _out_handle = std::io::stdout();
    let mut out_handle = _out_handle.lock();

    let mut person = Person::new();
    person.set_name("sevag".to_string());

    let mut outstream = CodedOutputStream::new(&mut out_handle);
    match person.write_to(&mut outstream) {
        Ok(x) => println!("Succesfully wrote to stdout: {:?}", x),
        Err(e) => panic!(e),
    }

    let mut buffer = Vec::new();

    match in_handle.read_to_end(&mut buffer) {
        Ok(_) => {
            let mut instream = CodedInputStream::from_bytes(&buffer);
            match Person::new().merge_from(&mut instream) {
                Ok(x) => println!("Decode result: {:?}", x),
                Err(e) => panic!(e),
            }
        },
        Err(e) => panic!(e),
    }
}
