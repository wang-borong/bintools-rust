use std::env;
use std::process::exit;
use std::path::Path;

use crate::shell;
use crate::rgignore::get_ignore_path;

pub fn run(cmd_name: &str, args: &Vec<String>) {
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
        arg_cmd = format!("{}/{}", p, cmd_name);
        if Path::new(&arg_cmd).exists() {
            break;
        }
    }
    if !Path::new(&arg_cmd).exists() {
        eprintln!("no {} in {}, {} or {}",
            cmd_name, arg_path_env[0], arg_path_env[1], arg_path_env[2]);
        exit(1);
    }

    if ignfpath.as_path().exists() {
        if cmd_name == "ag" {
            arg_cmd += &format!(" -p {} ",
                ignfpath.to_str().unwrap());
        } else if cmd_name == "rg" {
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
