#[rustfmt::skip]
use {
    crate::{
        bindgen::BindGen,
        config::Config,
        symbol_table::SymbolTable,
        syntax::Syntax
    },
    getopts::Options,
};

mod bindgen;
mod config;
mod display;
mod symbol_table;
mod syntax;

#[derive(Debug, PartialEq)]
enum BindOpt {
    Parse(String),
    Map(String),
    Namespace(String),
    Output(String),
    Symbols(String),
    Verbose,
}

fn usage() {
    println!("mu-bindgen: 0.0.1: [options] file");
    println!("-?                   usage message");
    println!("-h                   usage message");
    println!("-m path              bindmap path");
    println!("-n name              namespace symbol");
    println!("-o path              generated code path");
    println!("-s path              symbols path");
    println!("-v                   print version and exit");
    println!();
    println!("--help               usage message");
    println!("--map path           bindmap path");
    println!("--namespace symbol   namespace [namespace]");
    println!("--output path        output [path]");
    println!("--symbols path       symbols [path]");
    println!("--verbose path       verbose operation");
    println!("--version            print version and exit");

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
    opts.optopt("s", "symbols", "symbols path", "");

    #[rustfmt::skip]
    let opt_names = vec![
        "h", "help", "?",
        "m", "map",
        "n", "namespace",
        "o", "output",
        "s", "symbols",
        "v", "version",
        "verbose",
    ];

    let opt_list = match opts.parse(&argv[1..]) {
        Ok(opts) => opts,
        Err(error) => {
            eprintln!("runtime: option {error:?}");
            std::process::exit(-1);
        }
    };

    for name in opt_names {
        if opt_list.opt_present(name) {
            match opt_list.opt_get::<String>(name) {
                Ok(opt) => match name {
                    "h" | "help" | "?" => usage(),
                    "v" | "version" => {
                        println!("mu-bindgen: 0.0.1");
                        std::process::exit(0)
                    }
                    "m" | "map" => {
                        if let Some(path) = opt {
                            optv.push(BindOpt::Map(path))
                        }
                    }
                    "n" | "namespace" => {
                        if let Some(name) = opt {
                            optv.push(BindOpt::Namespace(name))
                        }
                    }
                    "o" | "output" => {
                        if let Some(path) = opt {
                            optv.push(BindOpt::Output(path))
                        }
                    }
                    "s" | "symbols" => {
                        if let Some(path) = opt {
                            optv.push(BindOpt::Symbols(path))
                        }
                    }
                    "verbose" => optv.push(BindOpt::Verbose),
                    _ => panic!(),
                },
                Err(_) => panic!(),
            }
        }
    }

    if opt_list.free.len() == 1 {
        optv.push(BindOpt::Parse(opt_list.free[0].clone()))
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
                            if let Some(ref path) = config.symbols_path() {
                                let symbol_table = SymbolTable::new(&config, &bindgen);

                                symbol_table.write(path).unwrap()
                            }
                            if config.verbose() {
                                bindgen.print_file(&src_path)
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
                    BindOpt::Symbols(path) => {
                        config.symbols_path.replace(Some(path));
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
