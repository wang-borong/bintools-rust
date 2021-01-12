use std::path::Path;
use std::process::exit;

use crate::shell;
use crate::utils;

pub fn run(args: &Vec<String>) {
    if args.len() != 2 {
        eprintln!("Usage: vd <diff path1> <diff path2>");
        exit(1);
    }

    if !Path::new(&args[0]).exists() || !Path::new(&args[1]).exists() {
        eprintln!("path is error!");
        exit(2);
    }

    let args_str = &args.join(" ");
    let mut diff_cmd = String::from("diff -Nr -q -X ~/Workspace/tools/cgminer.dontdiff ");
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
