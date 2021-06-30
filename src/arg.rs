use std::env;
use std::process::exit;
use std::path::Path;

use crate::shell;
use crate::rgignore::get_ignore_path;
use crate::utils;

pub fn run(cmd_name: &str, args: &Vec<String>) {
    if args.len() < 1 {
        eprintln!("[wraped ag|rg] Usage: ag|rg <search pattern>");
        exit(1);
    }

    // We will wrap the command which will be put in a user specified path,
    // and the wrapped command path is always the first entry in all obtained
    // paths. So that we can just get the unwrapped command path from the
    // second entry of paths.
    let arg_paths = utils::cmd_path(cmd_name);

    // We can not get a unwrapped command path
    if arg_paths.len() < 2 {
        eprintln!("no unwrapped {} in your path", cmd_name);
        exit(1);
    }

    let mut arg_cmd = arg_paths[1].clone();

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

    if sstr.len() > 1 && Path::new(sstr.last().unwrap()).is_dir() {
        sdir = sstr.pop().unwrap();
    }

    let args_opts = opts.join(" ");
    let args_str = sstr.join(" ");

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
