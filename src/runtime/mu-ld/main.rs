//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader/repl
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

extern crate mu_core;

mod image;
mod reader;
mod writer;

#[allow(unused_imports)]
use {
    crate::image::Image,
    getopt::Opt,
    mu_core::{Condition, Env, Result, Tag},
    std::{error::Error, fs, io::Write},
};

#[derive(Debug, PartialEq)]
enum LoadOpt {
    Config(String),
    Dump(String),
    Eval(String),
    Load(String),
    Output(String),
}

fn usage() {
    println!("env-ld: {}: [-h?vcelqod] [file...]", Env::VERSION);
    println!("?: usage message");
    println!("h: usage message");
    println!("c: [name:value,...]");
    println!("d: dump [path]");
    println!("e: eval form");
    println!("l: load path");
    println!("o: output [path]");
    println!("v: print version and exit");

    std::process::exit(0);
}

fn options(mut argv: Vec<String>) -> Option<Vec<LoadOpt>> {
    let mut opts = getopt::Parser::new(&argv, "h?vc:e:l:o:d:");
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
                    Opt('h', None) | Opt('?', None) => usage(),
                    Opt('v', None) => {
                        print!("env-ld: {} ", Env::VERSION);
                        return None;
                    }
                    Opt('e', Some(expr)) => {
                        optv.push(LoadOpt::Eval(expr));
                    }
                    Opt('o', Some(path)) => {
                        optv.push(LoadOpt::Output(path));
                    }
                    Opt('d', Some(path)) => {
                        optv.push(LoadOpt::Dump(path));
                    }
                    Opt('l', Some(path)) => {
                        optv.push(LoadOpt::Load(path));
                    }
                    Opt('c', Some(config)) => {
                        optv.push(LoadOpt::Config(config));
                    }
                    _ => panic!(),
                },
                None => panic!(),
            },
        }
    }

    for file in argv.split_off(opts.index()) {
        optv.push(LoadOpt::Load(file));
    }

    Some(optv)
}

pub fn main() {
    let mut _config: Option<String> = None;
    let mut _opath = "a.out".to_string();

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                // maybe a filter here?

                if let LoadOpt::Config(string) = opt {
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
        Some(config) => Env::new(config),
        None => {
            eprintln!("option: configuration error");
            std::process::exit(-1)
        }
    };

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                match opt {
                    LoadOpt::Eval(expr) => match env.eval_str(&expr) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("env-ld: error {}, {}", expr, env.exception_string(e));
                            std::process::exit(-1);
                        }
                    },
                    LoadOpt::Load(path) => match env.load(&path) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!(
                                "env-ld: failed to load {}, {}",
                                &path,
                                env.exception_string(e)
                            );
                            std::process::exit(-1);
                        }
                    },
                    LoadOpt::Config(_) => (),
                    LoadOpt::Output(path) => Image::output(&path),
                    LoadOpt::Dump(path) => Image::dump(&path),
                }
            }
        }
        None => std::process::exit(0),
    };
}
