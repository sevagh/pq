/* inspired by: https://github.com/BurntSushi/xsv/blob/master/tests/workdir.rs */

use std::env;
use std::path::PathBuf;
use std::io::{Read, Write};
use std::process::{Command, Output, Child, Stdio};
use std::fs::File;

pub struct Runner {
    pub cmd: Command,
    pub tests_path: PathBuf,
    chld: Option<Child>,
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

    fn with_stdin(&mut self, contents: &[u8]) {
        self.cmd.stdin(Stdio::piped());
        let mut chld = self._spawn();
        chld.stdin
            .as_mut()
            .unwrap()
            .write_all(contents)
            .unwrap();
        self.chld = Some(chld);
    }

    pub fn stdin_from_file(&mut self, path: &str) {
        let mut file = File::open(&self.tests_path.join(path)).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        self.with_stdin(&buf);
    }

    pub fn _spawn(&mut self) -> Child {
        self.cmd.spawn().unwrap()
    }

    pub fn spawn(&mut self) {
        match self.chld {
            Some(_) => (),
            None => self.chld = Some(self._spawn()),
        }
    }

    pub fn output(&mut self) -> Output {
        self.chld.take().unwrap().wait_with_output().unwrap()
    }
}
