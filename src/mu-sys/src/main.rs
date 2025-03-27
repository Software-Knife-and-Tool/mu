//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader/repl
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[allow(unused_imports)]
use {
    getopt::Opt,
    mu_runtime::{Condition, Env, Result, Tag},
    std::{fs, io::Write},
};

#[derive(Debug, PartialEq)]
enum ShellOpt {
    Config(String),
    Eval(String),
    Load(String),
    Quiet(String),
}

fn options(argv: Vec<String>) -> Option<Vec<ShellOpt>> {
    let mut opts = getopt::Parser::new(&argv, "c:e:l:q:");
    let mut optv = Vec::new();

    loop {
        let opt = opts.next().transpose();
        match opt {
            Err(_) => {
                if let Err(error) = opt {
                    eprintln!("runtime: option {error:?}")
                };
                std::process::exit(-1);
            }
            Ok(None) => {
                break;
            }
            Ok(clause) => match clause {
                Some(opt) => match opt {
                    Opt('e', Some(expr)) => {
                        optv.push(ShellOpt::Eval(expr));
                    }
                    Opt('q', Some(expr)) => {
                        optv.push(ShellOpt::Quiet(expr));
                    }
                    Opt('l', Some(path)) => {
                        optv.push(ShellOpt::Load(path));
                    }
                    Opt('c', Some(config)) => {
                        optv.push(ShellOpt::Config(config));
                    }
                    _ => panic!("{opt:?}"),
                },
                None => panic!(),
            },
        }
    }

    Some(optv)
}

pub fn main() {
    let mut _config: Option<String> = None;
    let mut _debug = false;

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                // maybe a filter here?
                if let ShellOpt::Config(string) = opt {
                    _config = Some(string)
                }
            }
        }
        None => {
            eprintln!("option: error");
            std::process::exit(-1)
        }
    }

    let env = match Env::config(_config) {
        Some(config) => Env::new(config, None),
        None => {
            eprintln!("option: configuration error");
            std::process::exit(-1)
        }
    };

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                match opt {
                    ShellOpt::Eval(expr) => match env.eval_str(&expr) {
                        Ok(eval) => println!("{}", env.write_to_string(eval, true)),
                        Err(e) => {
                            eprintln!("runtime: error {}, {}", expr, env.exception_string(e));
                            std::process::exit(-1);
                        }
                    },
                    ShellOpt::Load(path) => match env.load(&path) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!(
                                "runtime: failed to load {}, {}",
                                &path,
                                env.exception_string(e)
                            );
                            std::process::exit(-1);
                        }
                    },
                    ShellOpt::Quiet(expr) => match env.eval_str(&expr) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("runtime: error {}, {}", expr, env.exception_string(e));
                            std::process::exit(-1);
                        }
                    },
                    ShellOpt::Config(_) => (),
                }
            }
        }
        None => std::process::exit(0),
    };
}
