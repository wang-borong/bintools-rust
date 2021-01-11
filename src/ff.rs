use std::process::exit;
use crate::shell;

pub fn run(args: &Vec<String>) {
    if args.len() < 1 {
        println!("Usage: ff <file name pattern>");
        exit(1);
    }

    let args_str = args.join(" ");
    let find_file_cmd = format!("fd --color=always {} | fzf", args_str);
    let output = shell::run_with_out(&find_file_cmd);
    if output.stdout != "" {
        let edit_filt_cmd = format!("</dev/tty nvim {}", &output.stdout);
        shell::run(&edit_filt_cmd);
    }
}
