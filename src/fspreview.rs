use std::process::exit;
use crate::shell;
use crate::utils;

pub fn run(args: &Vec<String>) {
    if args.len() < 1 {
        eprintln!("Usage: fspreview <rgout> <termnal hight>");
        exit(1);
    }

    let rgout = &args[0];
    let termh = args[1].parse::<i32>().unwrap();
    let rgarr: Vec<&str> = rgout.splitn(3, ":").collect();
    let filname = rgarr[0];
    let linum = rgarr[1].parse::<i32>().unwrap();
    let half_termh = termh * 3 / 4;
    let mut startline = 0;
    if linum > half_termh {
        startline = linum - half_termh;
    }

    let view_cmd: String;
    if utils::cmd_exist("bat") {
        view_cmd = format!("bat -n --color=always -H {} -r {}: {}", linum, startline, filname);
    } else {
        view_cmd = format!("cat {}", filname);
    }

    shell::run(&view_cmd);
}
