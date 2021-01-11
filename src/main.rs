use std::env;
use std::path::Path;

mod shell;
mod sha;
mod fs;
mod fspreview;
mod ff;
mod arg;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let arg0 = String::from(&args[0]);
    let cmd_name = Path::new(&arg0).file_name().unwrap().to_str().unwrap();
    let oth_args = args.drain(1..).collect();

    match cmd_name {
        "fs" => {
            fs::run(&oth_args);
        },
        "fspreview" => {
            fspreview::run(&oth_args);
        },
        "ff" => {
            ff::run(&oth_args);
        },
        "ag" | "rg" => {
            arg::run(&arg0, &oth_args);
        },
        _ => {
            eprintln!("uncovered command");
        }
    }
}
