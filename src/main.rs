use std::env;
use std::path::Path;

mod shell;
mod sha;
mod utils;
mod fs;
mod fspreview;
mod ff;
mod arg;
mod rgignore;
mod vd;
mod c;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let arg0 = String::from(&args[0]);
    let cmd_name = Path::new(&arg0).file_name().unwrap().to_str().unwrap();
    let mut cmd_path = env::current_exe().unwrap();
    let oth_args = args.drain(1..).collect();

    // get dirname
    cmd_path.pop();

    match cmd_name {
        "bintools" => {
            eprintln!("bintools {}, whose usage just like busybox!\n", VERSION);
            eprintln!("symlink tools:");
            eprintln!(" * fs - wrapper of rg to search patterns with fzf");
            eprintln!(" * fspreview - fs previewer");
            eprintln!(" * ff - find file with fzf");
            eprintln!(" * rg|ag - wrapper of rg or ag");
            eprintln!(" * rgignore - tool to add ignore patterns for rg or ag");
            eprintln!(" * vd - tool to view differents between two directories");
            eprintln!(" * c - compile and run c code file");
            eprintln!("\nusage: ln -s bintools [fs|...]")
        },
        "fs" => fs::run(&oth_args),
        "fspreview" => fspreview::run(&oth_args),
        "ff" => ff::run(&oth_args),
        "ag" | "rg" => arg::run(&cmd_name, &cmd_path.to_str().unwrap(), &oth_args),
        "rgignore" => rgignore::run(&oth_args),
        "vd" => vd::run(&oth_args),
        "c" => c::run(&oth_args),
        _ => {
            eprintln!("unimplemented command {}", cmd_name);
        }
    }
}
