mod workdir;

use std::process;
use workdir::Workdir;

fn no_headers(cmd: &mut process::Command) {
    cmd.arg("--no-headers");
}

fn pad(cmd: &mut process::Command) {
    cmd.arg("--pad");
}

fn run_pqrs<F>(test_name: &str, modify_cmd: F) -> String
          where F: FnOnce(&mut process::Command) {
    let wrk = Workdir::new();

    let mut cmd = wrk.command();
    modify_cmd(cmd.arg("in1.csv").arg("in2.csv"));
    wrk.read_stdout(&mut cmd)
}


#[test]
fn test_whatever() {
    assert!(true);
}
