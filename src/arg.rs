use std::env;
use std::fs;
use std::process::exit;
use std::path::PathBuf;

use crate::sha;
use crate::shell;

fn get_ignore_path(p: &str) -> PathBuf {
    let home = env::var("HOME").unwrap();
    let ignpath = PathBuf::from(home).join(".cache/arg-ignore");
    if !ignpath.as_path().exists() {
        fs::create_dir(&ignpath).unwrap();
    }
    let cwdh = &sha::sha256(p.as_bytes())[..8];
    ignpath.join(cwdh)
}

pub fn run(arg0: &str, args: &Vec<String>) {
    if args.len() < 1 {
        println!("[wraped ag|rg] Usage: ag|rg <search pattern>");
        exit(1);
    }

    let cwd = env::current_dir().unwrap();
    let cwd_str = String::from(cwd.to_str().unwrap());
    let ignfpath = get_ignore_path(&cwd_str);
    let args_str = args.join(" ");
    let mut arg_cmd = String::from("/usr/bin/") + arg0 + " ";
    if ignfpath.as_path().exists() {
        if arg0 == "ag" {
            arg_cmd += &format!("-p {} ", ignfpath.to_str().unwrap());
        } else if arg0 == "rg" {
            arg_cmd += &format!("--ignore-file {} ", ignfpath.to_str().unwrap());
        } else {
            eprintln!("Error: not ag or rg command!");
            exit(2);
        }
    }
    arg_cmd += &args_str;

    shell::run(&arg_cmd);
}
