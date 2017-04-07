extern crate protobuf;

mod runner;

use std::process;
use runner::Runner;

fn for_nonexistent_file(work: &mut Runner) {
    work.cmd.arg("file-doesnt-exist");
}

fn for_dog(work: &mut Runner) {
    work.cmd.arg(&work.tests_path.join("samples/dog"));
}

fn for_person(work: &mut Runner) {
    work.cmd.arg(&work.tests_path.join("samples/person"));
}

fn run_pqrs<F>(modify_arg: F) -> process::Output
    where F: FnOnce(&mut Runner)
{
    let mut work = Runner::new();

    work.cmd
        .arg("--fdsets")
        .arg(&work.tests_path.join("fdsets"));
    modify_arg(&mut work);

    work.run()
}

#[test]
fn test_dog_decode() {
    let out = run_pqrs(for_dog);

    //check if success
    assert!(out.status.success());

    //check output
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"age\":3,\"breed\":\"gsd\",\"temperament\":\"excited\"}");
}

#[test]
fn test_person_decode() {
    let out = run_pqrs(for_person);

    //check if success
    assert!(out.status.success());

    //check output
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"id\":0,\"name\":\"khosrov\"}");
}

#[test]
fn test_nonexistent_file() {
    let out = run_pqrs(for_nonexistent_file);

    //check if success
    assert_eq!(out.status.code().unwrap(), 255);

    //check output
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");

    //check stderr
    assert_eq!(String::from_utf8_lossy(&out.stderr),
               "Could not open file: file-doesnt-exist\n");
}
