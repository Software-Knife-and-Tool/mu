//  SPDX-FileCopyrightText: Copyright 2026 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod rc;
mod repl;

use {
    mu::{Env, Mu},
    rc::Rc,
    std::fs,
};

fn rc_(env: &Env, rc: &Rc) -> (String, Option<Vec<String>>) {
    let options = rc.options.clone();

    let reader = match &rc.reader {
        Some(reader) => &reader,
        None => "mu",
    };

    match &rc.lib {
        Some(vec) => {
            for sys in vec {
                match Mu::load(&env, &("/opt/system-lisp/lib/".to_owned() + &sys)) {
                    Ok(_) => (),
                    Err(ex) => {
                        eprintln!(
                            "sys-repl: failed to load /opt/system-lisp/lib/{sys}, {}",
                            Mu::exception_string(&env, &ex)
                        );
                        std::process::exit(-1)
                    }
                }
            }
        }
        None => (),
    };

    match &rc.require {
        Some(vec) => {
            for module in vec {
                match Mu::eval_str(&env, &format!("(core:require \"{module}\")")) {
                    Ok(_) => (),
                    Err(ex) => {
                        eprintln!(
                            "sys-repl: failed to load module {module}, {}",
                            Mu::exception_string(&env, &ex)
                        );
                        std::process::exit(-1)
                    }
                }
            }
        }
        None => (),
    };

    let loader = match &rc.loader {
        Some(loader) => &loader,
        None => "mu",
    };

    match &rc.load {
        Some(vec) => {
            for path in vec {
                match loader {
                    "mu" => match Mu::load(&env, &path) {
                        Ok(_) => (),
                        Err(ex) => {
                            eprintln!(
                                "sys-repl: failed to load {path}, {}",
                                Mu::exception_string(&env, &ex)
                            );
                            std::process::exit(-1)
                        }
                    },
                    _ => {
                        if rc.option("verbose") {
                            println!("sys-repl: loading: {path}")
                        }
                        match Mu::eval_str(&env, &format!("(core:load \"{path}\")")) {
                            Ok(_) => (),
                            Err(ex) => {
                                eprintln!(
                                    "sys-repl: failed to load {path}, {}",
                                    Mu::exception_string(&env, &ex)
                                );
                                std::process::exit(-1)
                            }
                        }
                    }
                }
            }
        }
        None => (),
    };

    (reader.to_string(), options)
}

pub fn main() {
    let mut config_json = None;
    for path in vec!["./.sys-replrc", "~/.sys-replrc"] {
        if fs::metadata(path).is_ok() {
            config_json =
                Some(fs::read_to_string(path).expect("mu-load: failed to read .sys-repl"));
        }
    }

    let rc = Rc::new(config_json);
    let env = Mu::env(&rc.config);

    let (reader, _) = rc_(&env, &rc);
    repl::repl(&env, reader).expect("sys-repl: listener error");
}
