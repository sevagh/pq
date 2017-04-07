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

    pub fn run(&mut self) -> process::Output {
        self.cmd.output().unwrap()
    }
}

impl fmt::Debug for Workdir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path={}", self.root.display())
    }
}
