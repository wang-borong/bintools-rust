use std::process::exit;
use term_size;

use crate::shell;

pub fn run(args: &Vec<String>) {
    if args.len() < 1 {
        println!("Usage: fs <search patterns>");
        exit(1);
    }

    let (_, h) = term_size::dimensions().unwrap();
    let args_str = args.join(" ");
    let file_pos_cmd = format!(r#"rg --color=always -n {} | fzf -e --tac -0 -1 --color=bg+:24 --cycle --min-height=20 -d ':' --preview="echo '\033[1;32m  {{1}}\033[0m'; fspreview {{}} {}" --preview-window=right:60% | gawk -F':' '{{printf "%s +%s", $1, $2}}'"#, args_str, h);
    let file_pos_out = shell::run_with_out(&file_pos_cmd);
    if file_pos_out.stdout != "" {
        let edit_file_cmd = "nvim ".to_owned() + &file_pos_out.stdout;
        shell::run(&edit_file_cmd);
    }
}
