use std::env;
use std::fs;
use std::process::exit;
use std::path::{Path, PathBuf};

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
        eprintln!("[wraped ag|rg] Usage: ag|rg <search pattern>");
        exit(1);
    }

    let cwd = env::current_dir().unwrap();
    let cwd_str = String::from(cwd.to_str().unwrap());
    let ignfpath = get_ignore_path(&cwd_str);

    let mut opts: Vec<&str> = Vec::new();
    let mut sstr: Vec<&str> = Vec::new();
    let mut sdir: &str = "";
    for a in args {
        if a.chars().nth(0).unwrap() == '-' {
            opts.push(&a);
        } else {
            sstr.push(&a);
        }
    }

    if Path::new(sstr.last().unwrap()).is_dir() {
        sdir = sstr.pop().unwrap();
    }

    let args_opts = opts.join(" ");
    let args_str = sstr.join(" ");
    let mut arg_path_env: [&str; 3] = ["/usr/bin", "/usr/local/bin", ""];
    let home = env::var("HOME").unwrap();
    let home_path = home + "/.local/bin";
    arg_path_env[2] = &home_path;

    let mut arg_cmd = String::new();
    for p in arg_path_env.iter() {
        let tmp_cmd = Path::new(arg0)
            .file_name().unwrap()
            .to_str().unwrap();
        arg_cmd = format!("{}/{}", p, tmp_cmd);
        if Path::new(&arg_cmd).exists() {
            break;
        }
    }
    if !Path::new(&arg_cmd).exists() {
        eprintln!("no {} in {}, {} or {}",
            arg0, arg_path_env[0], arg_path_env[1], arg_path_env[2]);
        exit(1);
    }

    if ignfpath.as_path().exists() {
        if arg0 == "ag" {
            arg_cmd += &format!(" -p {} ",
                ignfpath.to_str().unwrap());
        } else if arg0 == "rg" {
            arg_cmd += &format!(" --ignore-file {} ",
                ignfpath.to_str().unwrap());
        } else {
            eprintln!("Error: not ag or rg command!");
            exit(2);
        }
    }
    arg_cmd = format!(r#"{} {} "{}" {}"#,
        arg_cmd, args_opts, args_str, sdir);

    shell::run(&arg_cmd);
}
