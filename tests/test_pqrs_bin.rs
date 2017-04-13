extern crate protobuf;

mod runner;

use std::process::{Stdio, Output};
use std::io::Write;
use runner::Runner;

fn for_nonexistent_file(work: &mut Runner) {
    work.cmd.arg("file-doesnt-exist");
    work.spawn();
}

fn for_dog(work: &mut Runner) {
    work.cmd.arg(&work.tests_path.join("samples/dog"));
    work.spawn();
}

fn for_person(work: &mut Runner) {
    work.cmd.arg(&work.tests_path.join("samples/person"));
    work.spawn();
}

fn with_stdin(work: &mut Runner) {
    work.cmd.stdin(Stdio::piped());
    work.spawn();
    work.chld
        .take()
        .unwrap()
        .stdin
        .unwrap()
        .write_all(b"")
        .unwrap();
}

fn empty_stdin(work: &mut Runner) {
    work.spawn();
}

fn run_pqrs<Farg, Fstdin>(modify_arg: Farg, modify_stdin: Fstdin) -> Output
    where Farg: FnOnce(&mut Runner),
          Fstdin: FnOnce(&mut Runner)
{
    let mut work = Runner::new();

    work.cmd
        .arg("--fdsets")
        .arg(&work.tests_path.join("fdsets"));

    modify_arg(&mut work);
    modify_stdin(&mut work);

    work.output()
}

#[test]
fn test_dog_decode() {
    let out = run_pqrs(for_dog, empty_stdin);

    println!("DEBUG: {:?}", out);

    //check if success
    assert!(out.status.success());

    //check output
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"age\":3,\"breed\":\"gsd\",\"temperament\":\"excited\"}");
}

#[test]
fn test_person_decode() {
    let out = run_pqrs(for_person, empty_stdin);

    //check if success
    assert!(out.status.success());

    //check output
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"id\":0,\"name\":\"khosrov\"}");
}

#[test]
fn test_nonexistent_file() {
    let out = run_pqrs(for_nonexistent_file, empty_stdin);

    //check if success
    assert_eq!(out.status.code().unwrap(), 255);

    //check output
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");

    //check stderr
    assert_eq!(String::from_utf8_lossy(&out.stderr),
               "Could not open file: file-doesnt-exist\n");
}
