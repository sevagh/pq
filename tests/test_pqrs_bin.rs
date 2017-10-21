extern crate protobuf;
extern crate assert_cli;

use std::env;

fn get_fdset_dir(final_piece: &str) -> String {
    let mut cwd = env::current_dir().unwrap();
    cwd.push("tests");
    cwd.push(final_piece);
    String::from(cwd.to_str().unwrap())
}

#[test]
fn test_dog_decode() {

    assert_cli::Assert::main_binary()
        .with_env(assert_cli::Environment::inherit().insert(
            "FDSET_PATH",
            get_fdset_dir("fdsets"),
        ))
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
    assert_cli::Assert::main_binary()
        .with_env(assert_cli::Environment::inherit().insert(
            "FDSET_PATH",
            get_fdset_dir("fdsets"),
        ))
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
    assert_cli::Assert::main_binary()
        .with_env(assert_cli::Environment::inherit().insert(
            "FDSET_PATH",
            get_fdset_dir(
                "fdsets-doesnt-exist",
            ),
        ))
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
    assert_cli::Assert::main_binary()
        .with_env(assert_cli::Environment::inherit().insert(
            "FDSET_PATH",
            get_fdset_dir(
                "fdsets-invalid",
            ),
        ))
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
    assert_cli::Assert::main_binary()
        .with_env(assert_cli::Environment::inherit().insert(
            "FDSET_PATH",
            get_fdset_dir("fdsets"),
        ))
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
    assert_cli::Assert::main_binary()
        .with_env(assert_cli::Environment::inherit().insert(
            "FDSET_PATH",
            get_fdset_dir("fdsets"),
        ))
        .with_args(&["--msgtype=com.example.dog.Dog"])
        .stdin(include_str!("samples/bad"))
        .fails()
        .and()
        .stderr()
        .contains("WireError")
        .unwrap();
}
