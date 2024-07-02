//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use lazy_static::lazy_static;

pub struct Options {
    pub options: Vec<Opt>,
}

#[derive(Debug, PartialEq)]
pub enum Opt {
    Crate(String),
    Verbose,
}

lazy_static! {
    pub static ref OPTIONMAP: Vec::<(&'static str, Opt)> = vec![
        ("verbose", Opt::Verbose),
        ("crate", Opt::Crate("".to_string())),
    ];
}

impl Options {
    pub fn version() {
        println!("sysgen: 0.0.4");
        std::process::exit(0)
    }

    pub fn usage() {
        println!("Usage: sysgen command [option]... [crate]...");
        println!("  command:      [init generate image symbols toc]");
        println!("    init        [option]... [crate]...   ; populate workspace");
        println!("    bind        [option]...              ; generate bindings");
        println!("    build       [option]...              ; build image");
        println!("  --verbose     verbose operation");
        println!("  -?, --help    print usage message and exit");
        println!("  --version     print version and exit");

        std::process::exit(0);
    }

    pub fn is_opt(&self, name: &str) -> bool {
        let bindopt = OPTIONMAP.iter().find(|(tag, _)| *tag == name);

        match bindopt {
            Some((_, bindopt)) => self
                .options
                .iter()
                .any(|opt| std::mem::discriminant(opt) == std::mem::discriminant(bindopt)),
            None => panic!(),
        }
    }

    #[allow(dead_code)]
    pub fn opt_value(&self, name: &str) -> Option<String> {
        let bindopt = OPTIONMAP.iter().find(|(tag, _)| *tag == name);

        match bindopt {
            Some((_, bindopt)) => match self
                .options
                .iter()
                .find(|opt| std::mem::discriminant(*opt) == std::mem::discriminant(bindopt))
            {
                Some(opt) => match opt {
                    Opt::Crate(str) => Some(str.to_string()),
                    Opt::Verbose => Some("".to_string()),
                },
                None => None,
            },
            None => panic!(),
        }
    }

    pub fn parse(argv: &[String]) -> Option<Options> {
        let mut opts = getopts::Options::new();
        let mut options = Vec::new();

        opts.optflag("", "help", "print usage");
        opts.optflag("", "verbose", "verbose");
        opts.optflag("", "version", "print version");
        opts.optflag("?", "", "print usage");

        let opt_names = vec!["?", "help", "verbose", "version"];

        let opt_list = match opts.parse(&argv[2..]) {
            Ok(opts) => opts,
            Err(error) => {
                eprintln!("sysgen: {error:?}");
                std::process::exit(-1);
            }
        };

        for name in opt_names {
            if opt_list.opt_present(name) {
                match opt_list.opt_get::<String>(name) {
                    Ok(_) => match name {
                        "help" | "?" => Self::usage(),
                        "verbose" => options.push(Opt::Verbose),
                        _ => panic!(),
                    },
                    Err(_) => panic!(),
                }
            }
        }

        if opt_list.free.len() == 1 {
            options.push(Opt::Crate(opt_list.free[0].clone()))
        }

        Some(Options { options })
    }
}
