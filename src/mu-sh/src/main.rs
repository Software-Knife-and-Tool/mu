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
    mu::{Condition, Env, Result, Tag},
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
                        print!("{} ", Env::VERSION);
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
    println!("mu-sh: {}: [-h?vcelq] [file...]", Env::VERSION);
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
    let eof_value = env.eval_str("(mu:make-symbol \"eof\")").unwrap();

    loop {
        print!("mu> ");
        let _ = std::io::Stdout::flush(&mut std::io::stdout());

        match env.read(env.std_in(), false, eof_value) {
            Ok(expr) => {
                if env.eq(expr, eof_value) {
                    break;
                }

                #[allow(clippy::single_match)]
                match env.compile(expr) {
                    Ok(form) => match env.eval(form) {
                        Ok(eval) => {
                            env.write(eval, true, env.std_out()).unwrap();
                            println!()
                        }
                        Err(e) => {
                            eprint!(
                                "eval exception raised by {}, {:?} condition on ",
                                env.write_to_string(e.source, true),
                                e.condition
                            );
                            env.write(e.object, true, env.err_out()).unwrap();
                            eprintln!()
                        }
                    },
                    Err(e) => {
                        eprint!(
                            "compile exception raised by {}, {:?} condition on ",
                            env.write_to_string(e.source, true),
                            e.condition
                        );
                        env.write(e.object, true, env.err_out()).unwrap();
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
                        env.write_to_string(e.source, true),
                        e.condition
                    );
                    env.write(e.object, true, env.err_out()).unwrap();
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

    listener(&env)
}
