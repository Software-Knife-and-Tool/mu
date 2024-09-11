//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#[derive(Debug)]
pub struct Options {
    pub options: Vec<Opt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Opt {
    Namespace(String),
    Load(String),
    Eval(String),
    Verbose,
}

impl Options {
    pub fn version() {
        println!("mux: 0.0.3");
        std::process::exit(0)
    }

    pub fn usage() {
        println!("Usage: mux command [option...]");
        println!("  command:    [help repl symbol-counts reference crossref test version]");
        println!("    crossref                      ; core symbols cross reference");
        println!("    reference [--namespace ns]    ; namespace symbol reference");
        println!("    help                          ; this message");
        println!("    repl      [--namespace ns]    ; repl");
        println!("    test      [--namespace ns]    ; run tests");
        println!("    symbol-counts                 ; core symbol counts");
        println!("    version                       ; mux version");
        println!();
        println!("  general options:");
        println!("    --verbose                     ; verbose operation");

        std::process::exit(0);
    }

    pub fn option_name(opt: Opt) -> String {
        match opt {
            Opt::Verbose => "verbose",
            Opt::Namespace(_) => "namespace",
            Opt::Load(_) => "load",
            Opt::Eval(_) => "eval",
        }
        .to_string()
    }

    pub fn parse(argv: &[String]) -> Option<Options> {
        let mut opts = getopts::Options::new();
        let mut options = Vec::new();

        opts.optflag("", "verbose", "");
        opts.optopt("", "namespace", "", "VALUE");
        opts.optopt("", "load", "", "VALUE");
        opts.optopt("", "eval", "", "VALUE");

        let opt_names = vec!["namespace", "verbose", "load", "eval"];

        let opts = match opts.parse(&argv[2..]) {
            Ok(opts) => opts,
            Err(error) => {
                eprintln!("mux options: {error:?}");
                std::process::exit(-1);
            }
        };

        for name in opt_names {
            if opts.opt_present(name) {
                match opts.opt_get::<String>(name) {
                    Ok(_) => match name {
                        "verbose" => options.push(Opt::Verbose),
                        "namespace" => options.push(Opt::Namespace(opts.opt_str(name).unwrap())),
                        "load" => options.push(Opt::Load(opts.opt_str(name).unwrap())),
                        "eval" => options.push(Opt::Eval(opts.opt_str(name).unwrap())),
                        _ => panic!(),
                    },
                    Err(_) => panic!(),
                }
            }
        }

        Some(Options { options })
    }
}
