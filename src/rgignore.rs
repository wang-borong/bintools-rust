use std::env;
use std::fs;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

use crate::sha;

pub fn get_ignore_path(p: &str) -> PathBuf {
    let home = env::var("HOME").unwrap();
    let ignpath = PathBuf::from(home).join(".cache/arg-ignore");
    if !ignpath.as_path().exists() {
        fs::create_dir(&ignpath).unwrap();
    }
    let cwdh = &sha::sha256(p.as_bytes())[..8];
    ignpath.join(cwdh)
}

fn add_ignore(p: &PathBuf, args: &Vec<String>) {
    let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(p)
                .unwrap();
    let ignores = args.join("\n");

    file.write_all(ignores.as_bytes())
                .expect("write ignore patterns failed!");
}

pub fn run(args: &Vec<String>) {
    let cwd = env::current_dir().unwrap();
    let cwd_str = String::from(cwd.to_str().unwrap());
    let ignfpath = get_ignore_path(&cwd_str);
    add_ignore(&ignfpath, args);
}
