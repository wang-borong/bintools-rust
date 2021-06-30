use std::io::{stdin,stdout,Write};
use std::fs;
use sha2::{Sha256, Digest};
use hex;

use crate::shell;

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);

    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

pub fn get_file_sha256(filename: &str) -> String {
    let cont = fs::read(filename).expect(&format!("Read {} failed!", filename));

    let mut hasher = Sha256::new();
    hasher.update(cont);
    let digest = hasher.finalize();
    hex::encode(&digest)
}

pub fn cmd_exist(cmd: &str) -> bool {
    let which_cmd = format!("which {}", cmd);
    let out = shell::run_with_out(&which_cmd);
    if out.exitcode.unwrap() != 0 {
        false
    } else {
        true
    }
}

pub fn cmd_path(cmd: &str) -> Vec<String> {
    let whereis_cmd = format!("whereis -b {}", cmd);

    let out = shell::run_with_out(&whereis_cmd);

    let mut vec: Vec<String> = Vec::new();
    for s in out.stdout.trim_end().split(" ") {
        if s.chars().nth(0).unwrap() == '/' {
            vec.push(String::from(s));
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use crate::utils;

    #[test]
    fn test_cmd_path() {
        let expect_ls_paths = vec![String::from("/bin/ls")];
        let ls_paths = utils::cmd_path("ls");

        assert_eq!(expect_ls_paths, ls_paths);
    }
}
