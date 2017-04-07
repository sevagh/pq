extern crate protobuf;

mod workdir;

use workdir::Workdir;

fn for_dog(work: &mut Workdir) {
    work.cmd.arg(&work.tests_path.join("samples/dog"));
}

fn for_person(work: &mut Workdir) {
    work.cmd.arg(&work.tests_path.join("samples/person"));
}
 
fn run_pqrs<F>(modify_arg: F) -> String
          where F: FnOnce(&mut Workdir) {
    let mut work = Workdir::new();

    work.cmd.arg("--fdsets").arg(&work.tests_path.join("fdsets"));
    modify_arg(&mut work);

    work.read_stdout()
}

#[test]
fn test_dog_decode() {
    assert_eq!(run_pqrs(for_dog),
                "{\"age\":3,\"breed\":\"gsd\",\"temperament\":\"excited\"}");
}

#[test]
fn test_person_decode() {
    assert_eq!(run_pqrs(for_person),
                "{\"id\":0,\"name\":\"khosrov\"}");
}
