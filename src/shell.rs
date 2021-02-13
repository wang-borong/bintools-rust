use std::process::{Command, Stdio};

pub struct ShellOut {
    pub exitcode: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl ShellOut {
    pub fn display_out_err(&self) {
        print!("{}", self.stdout);
        eprint!("{}", self.stderr);
    }

    pub fn _display_out(&self) {
        print!("{}", self.stdout);
    }

    pub fn _display_err(&self) {
        eprint!("{}", self.stderr);
    }
}

pub fn run(args_str: &str) {
    Command::new("bash")
        .arg("-c")
        .arg(args_str)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success();
}

pub fn run_with_out(args_str: &str) -> ShellOut {
    let child = Command::new("bash")
        .arg("-c")
        .arg(args_str)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = child.wait_with_output().unwrap();

    ShellOut {
        exitcode: output.status.code(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}
