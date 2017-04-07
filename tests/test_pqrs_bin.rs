extern crate protobuf;

mod workdir;
mod schemata;

use std::process;
use workdir::Workdir;
use schemata::dog::Dog;
use schemata::person::Person;

fn no_arg(_: &mut process::Command) {
    ()
}

fn run_pqrs<F>(modify_cmd: F) -> String
          where F: FnOnce(&mut process::Command) {
    let work = Workdir::new();

    let mut cmd = work.command();

    println!("DEBUG: {:?}", cmd);
    work.read_stdout(&mut cmd)
}

#[test]
fn test_whatever() {
    let out = run_pqrs(no_arg);
    println!("{:?}", out);
    assert!(true);
}
