use std::process::exit;
use crate::shell;

pub fn run(args: &Vec<String>) {
    if args.len() < 1 {
        eprintln!("Usage: ff <file name pattern>");
        exit(1);
    }

    let mut opts: Vec<&str> = Vec::new();
    let mut sstr: Vec<&str> = Vec::new();
    for arg in args {
        if arg.chars().nth(0).unwrap() == '-' {
            opts.push(&arg);
        } else {
            sstr.push(&arg);
        }
    }

    let file_str = sstr.join(" ");
    let opts_str = opts.join(" ");
    let find_file_cmd = format!(r#"fd --color=always {} "{}" | fzf"#, opts_str, file_str);
    let output = shell::run_with_out(&find_file_cmd);
    if output.stdout != "" {
        let edit_file_cmd = format!("</dev/tty nvim {}", &output.stdout);
        shell::run(&edit_file_cmd);
    }
}
