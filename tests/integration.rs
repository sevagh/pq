extern crate protobuf;

mod schemata;

use protobuf::Message;
use protobuf::CodedInputStream;
use protobuf::CodedOutputStream;

use schemata::addressbook::Person;

#[cfg(test)]
#[test]
fn decode_basic() {
    let mut person = Person::new();
    person.set_name("sevag".to_string());

    let mut outvec = Vec::new();
    
    let mut outstream = CodedOutputStream::new(&mut outvec);
    person.write_to(&mut outstream).unwrap();

    println!("{:?}", outvec);

    let mut instream = CodedInputStream::from_bytes(&outvec);

    match Person::new().merge_from(&mut instream) {
        Ok(x) => println!("Decode result: {:?}", x),
        Err(e) => panic!(e),
    }
}
