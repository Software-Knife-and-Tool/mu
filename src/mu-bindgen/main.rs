#[rustfmt::skip]
use {
    crate::{
        bindings::Bindings,
        config::Config,
        symbol_table::SymbolTable,
    },
    getopts::Options,
};

mod bindings;
mod config;
mod debug;
mod display;
mod symbol_table;
mod syntax;

#[derive(Debug, PartialEq)]
enum BindOpt {
    Parse(String),
    Map(String),
    Namespace(String),
    Bindings(String),
    Symbols(String),
    Verbose,
}

fn usage() {
    println!("mu-bindgen: 0.0.2: [options] file");
    println!("-?                   usage message");
    println!();
    println!("--help               usage message");
    println!("--map path           bindmap path");
    println!("--namespace symbol   namespace [namespace]");
    println!("--bindings path      generated bindings [path]");
    println!("--symbols path       symbols [path]");
    println!("--verbose path       verbose operation");
    println!("--version            print version and exit");

    std::process::exit(0);
}

fn options(argv: Vec<String>) -> Option<Vec<BindOpt>> {
    let mut opts = Options::new();
    let mut optv = Vec::new();

    opts.optflag("", "help", "print usage");
    opts.optflag("?", "", "print usage");
    opts.optflag("", "version", "print version");
    opts.optflag("", "verbose", "verbose");
    opts.optopt("", "map", "bindmap path", "");
    opts.optopt("", "namespace", "namespace", "");
    opts.optopt("", "bindings", "output path", "");
    opts.optopt("", "symbols", "symbols path", "");

    let opt_names = vec![
        "help",
        "?",
        "map",
        "namespace",
        "bindings",
        "symbols",
        "version",
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
                    "help" | "?" => usage(),
                    "version" => {
                        println!("mu-bindgen: 0.0.2");
                        std::process::exit(0)
                    }
                    "map" => {
                        if let Some(path) = opt {
                            optv.push(BindOpt::Map(path))
                        }
                    }
                    "namespace" => {
                        if let Some(name) = opt {
                            optv.push(BindOpt::Namespace(name))
                        }
                    }
                    "bindings" => {
                        if let Some(path) = opt {
                            optv.push(BindOpt::Bindings(path))
                        }
                    }
                    "symbols" => {
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
                    BindOpt::Parse(src_path) => match Bindings::with_config(&config, &src_path) {
                        Ok(bindings) => {
                            if let Some(ref path) = config.bindings_path() {
                                bindings.gencode(path).unwrap()
                            }
                            if let Some(ref path) = config.symbols_path() {
                                let symbol_table = SymbolTable::new(&config, &bindings);

                                symbol_table.write(path).unwrap()
                            }
                            if config.verbose() {}
                        }
                        Err(e) => {
                            eprintln!("mu-bindgen: failed to parse {}", e);
                            std::process::exit(-1);
                        }
                    },
                    BindOpt::Map(path) => {
                        config.bindmap_path.replace(Some(path));
                    }
                    BindOpt::Bindings(path) => {
                        config.bindings_path.replace(Some(path));
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
