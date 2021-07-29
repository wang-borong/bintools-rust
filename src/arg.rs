use std::env;
use std::process::exit;
use std::path::Path;

use crate::shell;
use crate::rgignore::get_ignore_path;
use crate::utils;

pub fn run(cmd_name: &str, cmd_path: &str, args: &Vec<String>) {
    if args.len() < 1 {
        eprintln!("[wraped ag|rg] Usage: ag|rg <search pattern>");
        exit(1);
    }

    let arg_paths = utils::cmd_path(cmd_name);

    // Exit if we can not get an unwrapped command path.
    if arg_paths.len() < 2 {
        eprintln!("no unwrapped {} in your path", cmd_name);
        exit(1);
    }

    // get the first unwrapped command (full path)
    let mut arg_cmd = String::new();
    for ap in arg_paths {
        if !ap.contains(cmd_path) {
            arg_cmd = ap;
            break;
        }
    }

    let cwd = env::current_dir().unwrap();
    let cwd_str = cwd.to_str().unwrap();
    let ignpath = get_ignore_path(&cwd_str);

    let mut opts: Vec<&str> = Vec::new();
    let mut sstr: Vec<&str> = Vec::new();
    let mut spath: &str = "";
    for a in args {
        if a.chars().nth(0).unwrap() == '-' {
            opts.push(&a);
        } else {
            sstr.push(&a);
        }
    }

    if sstr.len() > 1 && Path::new(sstr.last().unwrap()).exists() {
        spath = sstr.pop().unwrap();
    }

    let args_opts = opts.join(" ");
    let args_str = sstr.join(" ");

    if ignpath.as_path().exists() {
        if cmd_name == "ag" {
            arg_cmd += &format!(" -p {} ",
                ignpath.to_str().unwrap());
        } else if cmd_name == "rg" {
            arg_cmd += &format!(" --ignore-file {} ",
                ignpath.to_str().unwrap());
        } else {
            eprintln!("Error: not ag or rg command!");
            exit(2);
        }
    }
    arg_cmd = format!(r#"{} {} "{}" {}"#,
        arg_cmd, args_opts, args_str, spath);

    shell::run(&arg_cmd);
}
