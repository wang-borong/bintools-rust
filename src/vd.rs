use std::path::{Path, PathBuf};
use std::process::exit;
use std::env;

use crate::shell;
use crate::utils;

pub fn run(args: &Vec<String>) {
    if args.len() != 2 {
        eprintln!("Usage: vd <diff path1> <diff path2>");
        eprintln!("Note: users can use ~/.cache/vd.dontdiff to exclude diff files");
        exit(1);
    }

    if !Path::new(&args[0]).exists() || !Path::new(&args[1]).exists() {
        eprintln!("path is error!");
        exit(2);
    }

    if !utils::cmd_exist("nvim") {
        eprintln!("no nvim in your path");
        exit(1);
    }

    let args_str = &args.join(" ");
    let mut diff_cmd = String::from("diff -Nr -q ");

    let home = env::var("HOME").unwrap();
    let mut ignore_paths: Vec<PathBuf> = Vec::new();
    ignore_paths.push(Path::new(&home).join(".cache/vd.dontdiff"));
    for igp in ignore_paths {
        if igp.exists() {
            diff_cmd.push_str(&format!("-X {} ", igp.display()));
        }
    }
    diff_cmd.push_str(args_str);

    let output = shell::run_with_out(&diff_cmd);
    //let lines = output.stdout.lines();
    let mut lines: Vec<&str> = output.stdout.split("\n").collect();
    lines.pop(); // pop the last blank line
    let lines_len = lines.len();

    if lines_len == 0 {
        println!("no diff with {} and {}", &args[0], &args[1]);
        exit(0);
    }

    println!("there are {} diffs:", lines_len);

    let mut i: usize = 0;
    for line in &lines {
        let diffs = line
            .replace("Files ", "")
            .replace(" differ", "")
            .replace(" and ", " ");
        println!("[{:02}]: {}", i, diffs);
        i += 1;
    }

    i = 0;
    loop {
        let sel = utils::get_user_input("input number to open: ");
        let selno = sel.parse::<usize>();
        match selno {
            Ok(no) => {
                if i == lines_len {
                    break;
                }
                let line = String::from(lines[no]);
                let diffs = line
                    .replace("Files ", "")
                    .replace(" differ", "")
                    .replace(" and ", " ");
                let edit_diffs_cmd = format!("nvim -d {}", diffs);
                shell::run(&edit_diffs_cmd);
                i += 1;
            }
            Err(_) => {
                exit(3);
            }
        }
    }
}
