/* inspired by: https://github.com/BurntSushi/xsv/blob/master/tests/workdir.rs */

use std::env;
use std::path::PathBuf;
use std::process::{Command, Output, Child, Stdio};

pub struct Runner {
    pub cmd: Command,
    pub tests_path: PathBuf,
    pub chld: Option<Child>,
}

impl Runner {
    pub fn new() -> Runner {
        let mut root = env::current_exe()
            .unwrap()
            .parent()
            .expect("executable's directory")
            .to_path_buf();
        if root.ends_with("deps") {
            root.pop();
        }
        let mut tests_path = root.parent().unwrap().parent().unwrap().to_path_buf();
        tests_path.push("tests");
        let mut cmd = Command::new(root.join("pq"));
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        Runner {
            cmd: cmd,
            tests_path: tests_path,
            chld: None,
        }
    }

    pub fn spawn(&mut self) {
        let chld = self.cmd.spawn().unwrap();
        self.chld = Some(chld);
    }

    pub fn output(&mut self) -> Output {
        self.chld.take().unwrap().wait_with_output().unwrap()
    }
}
