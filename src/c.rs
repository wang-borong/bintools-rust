use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::env;

use crate::shell;
use crate::utils;


mod cconf {
    use serde::Deserialize;
    use std::fs;

    #[derive(Debug, Deserialize)]
    pub struct Bare {
        pub commands: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Make {
        pub commands: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Autotool {
        pub commands: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Cmake {
        pub commands: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub bare: Bare,
        pub make: Make,
        pub autotool: Autotool,
        pub cmake: Cmake,
    }

    pub fn get_config(confname: &str) -> Config {
        let conf_u8v = fs::read(confname).expect("Failed to read c.config.toml");
        let conf_string = String::from_utf8_lossy(&conf_u8v);
        let conf: Config = toml::from_str(&conf_string).expect("Failed to parse conf string");

        conf
    }
}

#[derive(Debug, Copy, Clone)]
enum Ctype {
    Bare,
    Make,
    Autotool,
    Cmake,
}

impl Ctype {
    pub fn get_type(projfile: &str) -> Option<Self> {
        let pf_attr = fs::metadata(projfile).expect(&format!("get {} attr failed!", projfile));
        if pf_attr.is_file() {
            Some(Ctype::Bare)
        } else if pf_attr.is_dir() {
            let proj = PathBuf::from(projfile);
            if proj.join("Makefile").exists() || proj.join("makefile").exists() {
                Some(Ctype::Make)
            } else if proj.join("configure.ac").exists() {
                Some(Ctype::Autotool)

            } else if proj.join("CMakefile.txt").exists() {
                Some(Ctype::Cmake)
            } else {
                eprintln!("Not supported type!");
                None
            }
        } else {
            eprintln!("Unknown type!");
            None
        }
    }
}

#[derive(Debug)]
struct Crun {
    conf: cconf::Config,
    outdir: String,
    ctype: Option<Ctype>,
    projfile: String, // project or file name
    outelf: String,
}

impl Crun {
    pub fn new(projfile: &str, confname: &str) -> Self {
        let out = &utils::get_file_sha256(projfile)[..10];
        //let elf = String::from(PathBuf::from("/tmp/c").join(out).to_str().unwrap());

        Self {
            conf: cconf::get_config(confname),
            outdir: String::from("/tmp/c"),
            ctype: Ctype::get_type(projfile),
            projfile: String::from(projfile),
            outelf: String::from(out),
        }
    }

    fn compile(&self) {
        if self.ctype.is_none() {
            eprintln!("Can not get c project type!");
            process::exit(1);
        } else {
            let ctype = self.ctype.unwrap();
            match ctype {
                Ctype::Bare => {
                    if !Path::new(&self.outdir).exists() {
                        fs::create_dir(&self.outdir).expect("Create /tmp/c failed!");
                    }

                    let elf = Path::new(&self.outdir).join(&self.outelf);
                    if !elf.exists() {
                        // compile file with gcc command
                        let compcmd = String::from(&self.conf.bare.commands[0]);
                        let compcmd = compcmd + " " + &self.projfile + " -o " + &elf.to_str().unwrap();
                        shell::run(&compcmd);
                    }
                },
                Ctype::Make => {
                    println!("make compiling...");
                    todo!();
                },
                Ctype::Autotool => {
                    println!("autotool compiling...");
                    if PathBuf::from(&self.projfile).join("autogen.sh").exists() {
                        // run ./autogen.sh, ./configure and make
                        // usually, autogen.sh just generate a configure, so users should
                        // ./configure the project and make compile it.
                    } else {
                    }
                    todo!();
                },
                Ctype::Cmake => {
                    println!("cmake compiling...");
                    todo!();
                },
            }
        }
    }

    pub fn exec(&self) {
        match self.ctype.unwrap() {
            Ctype::Bare => {
                let elf = PathBuf::from(&self.outdir).join(&self.outelf);
                if Path::new(&elf).exists() {
                    let shout = shell::run_with_out(&elf.to_str().unwrap());
                    shout.display_out_err();
                    match shout.exitcode {
                        Some(ec) => process::exit(ec),
                        None => eprintln!("Terminated by signal"),
                    }
                } else {
                    eprintln!("No compiled elf to execute!");
                }
            },
            _ => {
                // Do nothing
            }
        }
    }
}

pub fn run(args: &Vec<String>) {
    let projfile: &str;
    if args.len() == 0 {
        projfile = ".";
    } else {
        projfile = &args[0];
    }

    let confname = PathBuf::from(env::var("HOME").unwrap()).join(".config/c.config.toml");
    let crun = Crun::new(projfile, &confname.as_path().to_str().unwrap());
    crun.compile();
    crun.exec();
}
