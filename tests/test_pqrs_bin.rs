extern crate protobuf;

extern crate assert_cli;

use std::env;

#[test]
fn test_dog_decode() {
    env::remove_var("FDSET_PATH");
    env::set_var("FDSET_PATH", "/home/sevagh/repos/pq/tests/fdsets/");
    assert_cli::Assert::main_binary()
        .with_args(&["--msgtype=com.example.dog.Dog"])
        .stdin(include_str!("samples/dog"))
        .succeeds()
        .and()
        .stdout()
        .contains("{\"age\":3,\"breed\":\"gsd\",\"temperament\":\"excited\"}")
        .unwrap();
}

#[test]
fn test_dog_decode_stream() {
    env::remove_var("FDSET_PATH");
    env::set_var("FDSET_PATH", "/home/sevagh/repos/pq/tests/fdsets/");
    assert_cli::Assert::main_binary()
        .with_args(&["--msgtype=com.example.dog.Dog", "--stream=varint"])
        .stdin(include_str!("samples/dog_stream"))
        .succeeds()
        .and()
        .stdout()
        .contains(
            "{\"age\":2,\"breed\":\"rottweiler\",\"temperament\":\"chill\"}",
        )
        .unwrap();
}

#[test]
fn test_nonexistent_fdset_dir() {
    env::remove_var("FDSET_PATH");
    env::set_var("FDSET_PATH", "fdsets-doesnt-exist");
    assert_cli::Assert::main_binary()
        .with_args(&["--msgtype=com.example.dog.Dog"])
        .stdin(include_str!("samples/dog"))
        .fails()
        .and()
        .stderr()
        .contains(
            "No valid fdset files found in dirs: $FDSET_PATH, $HOME/.pq, /etc/pq",
        )
        .unwrap();
}

#[test]
fn test_no_fdset_files() {
    env::remove_var("FDSET_PATH");
    env::set_var("FDSET_PATH", "/home/sevagh/repos/pq/tests/fdsets-invalid");
    assert_cli::Assert::main_binary()
        .with_args(&["--msgtype=com.example.dog.Dog"])
        .stdin(include_str!("samples/dog"))
        .fails()
        .and()
        .stderr()
        .contains(
            "No valid fdset files found in dirs: $FDSET_PATH, $HOME/.pq, /etc/pq",
        )
        .unwrap();
}


#[test]
fn test_person_decode() {
    env::remove_var("FDSET_PATH");
    env::set_var("FDSET_PATH", "/home/sevagh/repos/pq/tests/fdsets/");
    assert_cli::Assert::main_binary()
        .with_args(&["--msgtype=com.example.person.Person"])
        .stdin(include_str!("samples/person"))
        .succeeds()
        .and()
        .stdout()
        .contains("{\"id\":0,\"name\":\"khosrov\"}")
        .unwrap();
}

#[test]
fn test_bad_input() {
    env::remove_var("FDSET_PATH");
    env::set_var("FDSET_PATH", "/home/sevagh/repos/pq/tests/fdsets/");
    assert_cli::Assert::main_binary()
        .with_args(&["--msgtype=com.example.dog.Dog"])
        .stdin(include_str!("samples/bad"))
        .fails()
        .and()
        .stderr()
        .contains("WireError")
        .unwrap();
}
