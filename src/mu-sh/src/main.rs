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
    mu_runtime::{Condition, Env, Mu, Result, Tag},
    std::{fs, io::Write},
};

#[derive(Debug, PartialEq)]
enum ShellOpt {
    Config(String),
    Eval(String),
    Load(String),
    Quiet(String),
}

fn options(mut argv: Vec<String>) -> Option<Vec<ShellOpt>> {
    let mut opts = getopt::Parser::new(&argv, "h?vc:e:l:q: ...");
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
                        print!("{} ", Mu::VERSION);
                        std::process::exit(0);
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
    println!("mu-sh: {}: [-h?vcelq] [file...]", Mu::VERSION);
    println!("?: usage message");
    println!("h: usage message");
    println!("c: [name:value, ...]");
    println!("e: eval [form] and print result");
    println!("l: load [path]");
    println!("q: eval [form] quietly");
    println!("v: print version and exit");

    std::process::exit(0);
}

fn listener(env: &Env) {
    let eof_value = Mu::eval_str(env, "(mu:make-symbol \"eof\")").unwrap();

    loop {
        print!("mu> ");
        let _ = std::io::Stdout::flush(&mut std::io::stdout());

        match Mu::read(env, Mu::std_in(), false, eof_value) {
            Ok(expr) => {
                if Mu::eq(expr, eof_value) {
                    break;
                }

                #[allow(clippy::single_match)]
                match Mu::compile(env, expr) {
                    Ok(form) => match Mu::eval(env, form) {
                        Ok(eval) => {
                            Mu::write(env, eval, true, Mu::std_out()).unwrap();
                            println!()
                        }
                        Err(e) => {
                            eprint!(
                                "eval exception raised by {}, {:?} condition on ",
                                Mu::write_to_string(env, e.source, true),
                                e.condition
                            );
                            Mu::write(env, e.object, true, Mu::err_out()).unwrap();
                            eprintln!()
                        }
                    },
                    Err(e) => {
                        eprint!(
                            "compile exception raised by {}, {:?} condition on ",
                            Mu::write_to_string(env, e.source, true),
                            e.condition
                        );
                        Mu::write(env, e.object, true, Mu::err_out()).unwrap();
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
                        Mu::write_to_string(env, e.source, true),
                        e.condition
                    );
                    Mu::write(env, e.object, true, Mu::err_out()).unwrap();
                    eprintln!()
                }
            }
        }
    }
}

pub fn main() {
    let mut _config: Option<String> = None;
    let mut _debug = false;

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                // maybe a find here?
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

    let env = match Mu::config(_config) {
        Some(config) => Mu::make_env(config),
        None => {
            eprintln!("option: configuration error");
            std::process::exit(-1)
        }
    };

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                match opt {
                    ShellOpt::Eval(expr) => match Mu::eval_str(&env, &expr) {
                        Ok(eval) => println!("{}", Mu::write_to_string(&env, eval, true)),
                        Err(e) => {
                            eprintln!("runtime: error {}, {}", expr, Mu::exception_string(&env, e));
                            std::process::exit(-1);
                        }
                    },
                    ShellOpt::Load(path) => match Mu::load(&env, &path) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!(
                                "runtime: failed to load {}, {}",
                                &path,
                                Mu::exception_string(&env, e)
                            );
                            std::process::exit(-1);
                        }
                    },
                    ShellOpt::Quiet(expr) => match Mu::eval_str(&env, &expr) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("runtime: error {}, {}", expr, Mu::exception_string(&env, e));
                            std::process::exit(-1);
                        }
                    },
                    ShellOpt::Config(_) => (),
                }
            }
        }
        None => std::process::exit(0),
    };

    listener(&env)
}
