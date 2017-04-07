/* inspired by: https://github.com/BurntSushi/xsv/blob/master/tests/workdir.rs */

use std::env;
use std::fmt;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

pub struct Workdir {
    pub cmd: process::Command,
    pub tests_path: PathBuf,
    root: PathBuf,
}

impl Workdir {
    pub fn new() -> Workdir {
        let mut root = env::current_exe().unwrap().parent().expect("executable's directory").to_path_buf();
        if root.ends_with("deps") {
            root.pop();
        }
        let mut tests_path = root.parent().unwrap().parent().unwrap().to_path_buf();
        tests_path.push("tests");
        let cmd = process::Command::new(root.join("pq"));
        Workdir { cmd: cmd, root: root, tests_path: tests_path }
    }

    pub fn read_stdout(&mut self) -> String {
        let stdout: String = self.stdout();
        stdout
    }

    pub fn output(&mut self) -> process::Output {
        let o = self.cmd.output().unwrap();
        if !o.status.success() {
            panic!("\n\n===== {:?} =====\n\
                    command failed but expected success!\
                    \n\nstatus: {}\
                    \n\nstdout: {}\n\nstderr: {}\
                    \n\n=====\n",
                   self.cmd, o.status,
                   String::from_utf8_lossy(&o.stdout),
                   String::from_utf8_lossy(&o.stderr))
        }
        o
    }

    pub fn stdout<T: FromStr>(&mut self) -> T {
        let o = self.output();
        let stdout = String::from_utf8_lossy(&o.stdout);
        stdout.trim_matches(&['\r', '\n'][..]).parse().ok().expect(
            &format!("Could not convert from string: '{}'", stdout))
    }

    pub fn assert_err(&self, cmd: &mut process::Command) {
        let o = cmd.output().unwrap();
        if o.status.success() {
            panic!("\n\n===== {:?} =====\n\
                    command succeeded but expected failure!\
                    \n\nstatus: {}\
                    \n\nstdout: {}\n\nstderr: {}\
                    \n\n=====\n",
                   cmd, o.status,
                   String::from_utf8_lossy(&o.stdout),
                   String::from_utf8_lossy(&o.stderr));
        }
    }
}

impl fmt::Debug for Workdir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path={}", self.root.display())
    }
}
