use std::io::{stdin,stdout,Write};
use std::fs;
use sha2::{Sha256, Digest};
use hex;


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
