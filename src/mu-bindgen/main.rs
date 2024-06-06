#![allow(dead_code)]
#[allow(unused_imports)]
use {
    crate::bindgen::{BindGen, Config},
    getopts::Options,
    std::{cell::RefCell, env, error::Error, fs, io::Write},
};

mod bindgen;
mod display;
mod symbol_table;
mod syntax;

#[derive(Debug, PartialEq)]
enum BindOpt {
    Parse(String),
    Map(String),
    Namespace(String),
    Output(String),
    Verbose,
}

fn usage() {
    println!("mu-bindgen: 0.0.1: [-h?m:n:o:v][--namespace verbose help map] [file...]");
    println!("?: usage message");
    println!("h: usage message");
    println!("m: bindmap [path]");
    println!("n: namespace [namespace]");
    println!("o: generated code [path]");
    println!("v: print version and exit");
    println!();
    println!("help:      usage message");
    println!("map:       bindmap [path]");
    println!("namespace: namespace [namespace]");
    println!("verbose:   verbose operation");

    std::process::exit(0);
}

fn options(argv: Vec<String>) -> Option<Vec<BindOpt>> {
    let mut opts = Options::new();
    let mut optv = Vec::new();

    opts.optflag("h", "help", "print usage");
    opts.optflag("?", "", "print usage");
    opts.optflag("v", "version", "print version");
    opts.optflag("", "verbose", "verbose");
    opts.optopt("m", "map", "bindmap path", "");
    opts.optopt("n", "namespace", "namespace", "");
    opts.optopt("o", "output", "output path", "");

    let opt_list = match opts.parse(&argv[1..]) {
        Ok(opts) => opts,
        Err(error) => {
            eprintln!("runtime: option {error:?}");
            std::process::exit(-1);
        }
    };

    if opt_list.opt_present("h") || opt_list.opt_present("?") || opt_list.opt_present("help") {
        usage()
    }

    if opt_list.opt_present("m") {
        optv.push(BindOpt::Map(opt_list.opt_str("m").unwrap()))
    }

    if opt_list.opt_present("map") {
        optv.push(BindOpt::Map(opt_list.opt_str("map").unwrap()))
    }

    if opt_list.opt_present("n") {
        optv.push(BindOpt::Namespace(opt_list.opt_str("n").unwrap()))
    }

    if opt_list.opt_present("namespace") {
        optv.push(BindOpt::Namespace(opt_list.opt_str("namespace").unwrap()))
    }

    if opt_list.opt_present("v") || opt_list.opt_present("version") {
        print!("mu-bindgen: 0.0.1");
        std::process::exit(0)
    }

    if opt_list.opt_present("o") {
        optv.push(BindOpt::Output(opt_list.opt_str("o").unwrap()))
    }

    if opt_list.opt_present("output") {
        optv.push(BindOpt::Output(opt_list.opt_str("output").unwrap()))
    }

    if opt_list.opt_present("verbose") {
        optv.push(BindOpt::Verbose)
    }

    for file in opt_list.free {
        optv.push(BindOpt::Parse(file));
    }

    Some(optv)
}

pub fn main() {
    let config = Config::new();

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                match opt {
                    BindOpt::Parse(src_path) => match BindGen::with_config(&config, &src_path) {
                        Ok(bindgen) => {
                            if let Some(ref path) = config.output_path() {
                                bindgen.write(path).unwrap()
                            }
                            if config.verbose() {
                                bindgen.print_items(&src_path)
                            }
                        }
                        Err(e) => {
                            eprintln!("mu-bindgen: failed to parse {}", e);
                            std::process::exit(-1);
                        }
                    },
                    BindOpt::Map(path) => {
                        config.bindmap_path.replace(Some(path));
                    }
                    BindOpt::Output(path) => {
                        config.output_path.replace(Some(path));
                    }
                    BindOpt::Namespace(name) => {
                        config.namespace.replace(Some(name));
                    }
                    BindOpt::Verbose => {
                        config.verbose.replace(true);
                    }
                }
            }
        }
        None => std::process::exit(0),
    };
}
