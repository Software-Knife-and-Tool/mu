//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use lazy_static::lazy_static;

pub struct Options {
    pub options: Vec<Opt>,
}

#[derive(Debug, PartialEq)]
pub enum Opt {
    Namespace(String),
    Verbose,
}

lazy_static! {
    pub static ref OPTIONMAP: Vec::<(&'static str, Opt)> = vec![
        ("verbose", Opt::Verbose),
        ("namespace", Opt::Namespace("".to_string())),
    ];
}

impl Options {
    pub fn version() {
        println!("devop: 0.0.1");
        std::process::exit(0)
    }

    pub fn usage() {
        println!("Usage: devop command [option]...");
        println!("  command:      [repl symbols crossref test]");
        println!("    crossref    [option]...   ; symbol cross reference");
        println!("    help        [option]...   ; this message");
        println!("    repl        [option]...   ; repl");
        println!("    test        [option]...   ; run tests");
        println!("    symbols     [option]...   ; symbol table");
        println!("    version     [option]...   ; devop version");
        println!();
        println!("  options:");
        println!("    --verbose                 ; verbose operation");
        println!("    --namespace               ; specify namespace");

        std::process::exit(0);
    }

    #[allow(dead_code)]
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
                    Opt::Namespace(str) => Some(str.to_string()),
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

        opts.optflag("", "verbose", "verbose");
        opts.optflag("", "namespace", "namespacen");

        let opt_names = vec!["namespace", "verbose"];

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

        Some(Options { options })
    }
}
