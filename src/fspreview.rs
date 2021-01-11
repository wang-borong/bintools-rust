use std::process::exit;
use crate::shell;

pub fn run(args: &Vec<String>) {
    if args.len() < 1 {
        println!("Usage: fspreview <rgout> <termnal hight>");
        exit(1);
    }

    let rgout = &args[0];
    let termh = args[1].parse::<i32>().unwrap();
    let rgarr: Vec<&str> = rgout.splitn(3, ":").collect();
    let filname = rgarr[0];
    let linum = rgarr[1].parse::<i32>().unwrap();
    let half_termh = termh / 2;
    let mut startline = 0;
    if linum > half_termh {
        startline = linum - half_termh;
    }
    let batcmd = format!("bat -n --color=always -H {} -r {}: {}", linum, startline, filname);

    shell::run(&batcmd);
}
