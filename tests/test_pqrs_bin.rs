extern crate protobuf;

mod workdir;

use std::process;
use workdir::Workdir;

fn for_nonexistent_file(work: &mut Workdir) {
    work.cmd.arg("file-doesnt-exist");
}

fn for_dog(work: &mut Workdir) {
    work.cmd.arg(&work.tests_path.join("samples/dog"));
}

fn for_person(work: &mut Workdir) {
    work.cmd.arg(&work.tests_path.join("samples/person"));
}
 
fn run_pqrs<F>(modify_arg: F) -> process::Output
          where F: FnOnce(&mut Workdir) {
    let mut work = Workdir::new();

    work.cmd.arg("--fdsets").arg(&work.tests_path.join("fdsets"));
    modify_arg(&mut work);

    work.run()
}
 
fn run_pqrs_with_error<F>(modify_arg: F) -> String
          where F: FnOnce(&mut Workdir) {
    let mut work = Workdir::new();

    work.cmd.arg("--fdsets").arg(&work.tests_path.join("fdsets"));
    modify_arg(&mut work);

    work.assert_err()
}


#[test]
fn test_dog_decode() {
    assert_eq!(run_pqrs(for_dog),
                "{\"age\":3,\"breed\":\"gsd\",\"temperament\":\"excited\"}");
}

#[test]
fn test_person_decode() {
    assert_eq!(run_pqrs(for_person),
                "{\"age\":3,\"name\":\"khosrov\"}");
}
