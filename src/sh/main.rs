//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader/repl
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

extern crate mu;

#[allow(unused_imports)]
use {
    getopt::Opt,
    mu::{Condition, Mu, Result, Tag},
    std::{fs, io::Write},
};

#[derive(Debug, PartialEq)]
enum ShellOpt {
    Config(String),
    Eval(String),
    Load(String),
    Pipe,
    Quiet(String),
}

fn options(mut argv: Vec<String>) -> Option<Vec<ShellOpt>> {
    let mut opts = getopt::Parser::new(&argv, "h?pvc:e:l:q:");
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
                        print!("mu-sh: {} ", Mu::VERSION);
                        return None;
                    }
                    Opt('p', None) => {
                        optv.push(ShellOpt::Pipe);
                    }
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
                    _ => panic!(),
                },
                None => panic!(),
            },
        }
    }

    for file in argv.split_off(opts.index()) {
        optv.push(ShellOpt::Load(file));
    }

    Some(optv)
}

fn usage() {
    println!("mu-sh: {}: [-h?pvcelq] [file...]", Mu::VERSION);
    println!("?: usage message");
    println!("h: usage message");
    println!("c: [name:value,...]");
    println!("e: eval [form] and print result");
    println!("l: load [path]");
    println!("p: pipe mode");
    println!("q: eval [form] quietly");
    println!("v: print version and exit");

    std::process::exit(0);
}

fn listener(mu: &Mu) {
    let eof_value = mu.eval_str("(mu:symbol \"eof\")").unwrap();

    loop {
        match mu.read(mu.std_in(), false, eof_value) {
            Ok(expr) => {
                if mu.eq(expr, eof_value) {
                    break;
                }

                #[allow(clippy::single_match)]
                match mu.compile(expr) {
                    Ok(form) => match mu.eval(form) {
                        Ok(eval) => {
                            mu.write(eval, true, mu.std_out()).unwrap();
                            println!();
                        }
                        Err(e) => {
                            eprint!(
                                "eval exception raised by {}, {:?} condition on ",
                                mu.write_to_string(e.source, true),
                                e.condition
                            );
                            mu.write(e.object, true, mu.err_out()).unwrap();
                            eprintln!()
                        }
                    },
                    Err(e) => {
                        eprint!(
                            "compile exception raised by {}, {:?} condition on ",
                            mu.write_to_string(e.source, true),
                            e.condition
                        );
                        mu.write(e.object, true, mu.err_out()).unwrap();
                        eprintln!()
                    }
                }
            }
            Err(e) => {
                if let Condition::Eof = e.condition {
                    std::process::exit(0);
                } else {
                    eprint!(
                        "reader exception raised by {}, {:?} condition on ",
                        mu.write_to_string(e.source, true),
                        e.condition
                    );
                    mu.write(e.object, true, mu.err_out()).unwrap();
                    eprintln!()
                }
            }
        }
    }
}

pub fn main() {
    let mut _config = String::new();
    let mut _debug = false;
    let mut pipe = false;

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                // maybe a filter here?
                if let ShellOpt::Config(string) = opt {
                    _config = string
                }
            }
        }
        None => {
            eprintln!("option: error");
            std::process::exit(-1)
        }
    }

    let mu = match Mu::config(&_config) {
        Some(config) => Mu::new(&config),
        None => {
            eprintln!("option: configuration error");
            std::process::exit(-1)
        }
    };

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                match opt {
                    ShellOpt::Eval(expr) => match mu.eval_str(&expr) {
                        Ok(eval) => println!("{}", mu.write_to_string(eval, true)),
                        Err(e) => {
                            eprintln!("runtime: error {}, {}", expr, mu.exception_string(e));
                            std::process::exit(-1);
                        }
                    },
                    ShellOpt::Pipe => {
                        pipe = true;
                    }
                    ShellOpt::Load(path) => match mu.load(&path) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!(
                                "runtime: failed to load {}, {}",
                                &path,
                                mu.exception_string(e)
                            );
                            std::process::exit(-1);
                        }
                    },
                    ShellOpt::Quiet(expr) => match mu.eval_str(&expr) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("runtime: error {}, {}", expr, mu.exception_string(e));
                            std::process::exit(-1);
                        }
                    },
                    ShellOpt::Config(_) => (),
                }
            }
        }
        None => std::process::exit(0),
    };

    if !pipe {
        listener(&mu)
    }
}
