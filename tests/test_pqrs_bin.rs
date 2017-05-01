extern crate protobuf;

mod runner;

use std::process::Output;
use std::io::Read;
use std::fs::File;
use runner::Runner;

fn for_nonexistent_fdset_dir(work: &mut Runner) {
    work.cmd.env("FDSET_PATH", "fdset-doesnt-exist");
    work.cmd.arg(&work.tests_path.join("samples/dog"));
}

fn for_no_valid_fdsets(work: &mut Runner) {
    work.cmd
        .env("FDSET_PATH", &work.tests_path.join("fdsets-invalid"));
}

fn for_nonexistent_file(work: &mut Runner) {
    work.cmd.arg("file-doesnt-exist");
}

fn for_bad_input(work: &mut Runner) {
    work.cmd.arg(&work.tests_path.join("samples/bad"));
}

fn for_dog_file(work: &mut Runner) {
    work.cmd.arg(&work.tests_path.join("samples/dog"));
}

fn for_dog_stream(work: &mut Runner) {
    work.cmd.arg("--stream=varint");
    work.cmd.arg(&work.tests_path.join("samples/dog_stream"));
}

fn for_person(work: &mut Runner) {
    work.cmd.arg(&work.tests_path.join("samples/person"));
}

fn for_dog_stdin(work: &mut Runner) {
    let mut file = File::open(&work.tests_path.join("samples/dog")).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    work.with_stdin(&buf);
}

fn run_pqrs<F>(modify_in: F) -> Output
    where F: FnOnce(&mut Runner)
{
    let mut work = Runner::new();

    work.cmd
        .env("FDSET_PATH", &work.tests_path.join("fdsets"));

    modify_in(&mut work);

    work.spawn();
    work.output()
}

#[test]
fn test_dog_decode_from_file() {
    let out = run_pqrs(for_dog_file);
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"age\":3,\"breed\":\"gsd\",\"temperament\":\"excited\"}");
}

#[test]
fn test_dog_decode_from_stdin() {
    let out = run_pqrs(for_dog_stdin);
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"age\":3,\"breed\":\"gsd\",\"temperament\":\"excited\"}");
}

#[test]
fn test_dog_decode_stream() {
    let out = run_pqrs(for_dog_stream);
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"age\":2,\"breed\":\"rottweiler\",\"temperament\":\"chill\"}");
}

#[test]
fn test_nonexistent_fdset_dir() {
    let out = run_pqrs(for_nonexistent_fdset_dir);
    assert_eq!(out.status.code().unwrap(), 255);
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert!(String::from_utf8_lossy(&out.stderr)
            .contains("Path fdset-doesnt-exist doesn\'t exist"));
}

#[test]
fn test_no_fdset_files() {
    let out = run_pqrs(for_no_valid_fdsets);
    assert_eq!(out.status.code().unwrap(), 255);
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert!(String::from_utf8_lossy(&out.stderr).contains("No valid fdset files in path"));
}

#[test]
fn test_person_decode() {
    let out = run_pqrs(for_person);
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"id\":0,\"name\":\"khosrov\"}");
}

#[test]
fn test_nonexistent_file() {
    let out = run_pqrs(for_nonexistent_file);
    assert_eq!(out.status.code().unwrap(), 255);
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert!(String::from_utf8_lossy(&out.stderr)
            .contains("No such file or directory (os error 2)"));
}

#[test]
fn test_bad_input() {
    let out = run_pqrs(for_bad_input);
    assert_eq!(out.status.code().unwrap(), 255);
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert!(String::from_utf8_lossy(&out.stderr)
            .contains("Couldn\'t decode with any message descriptor\n"));
}
