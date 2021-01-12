use std::process::{Command, Stdio};

pub struct ShellOut {
    pub stdout: String,
    pub stderr: String,
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
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}
