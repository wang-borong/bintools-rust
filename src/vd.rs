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

    print!("there are {} diffs, use nvim to show them?", lines_len);
    let ans = utils::get_user_input(" ");
    for line in lines {
        let diffs = line
            .replace("Files ", "")
            .replace(" differ", "")
            .replace(" and ", " ");
        if ans == "y" || ans == "n" {
            let edit_diffs_cmd = format!("nvim -d {}", diffs);
            shell::run(&edit_diffs_cmd);
        } else {
            println!("{}", diffs);
        }
    }
}
