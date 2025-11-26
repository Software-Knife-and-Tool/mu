//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
use {
    crate::config::{Config, Value},
    mu::{Env, Mu},
};

pub struct Env_ {
    pub env: Env,
    pub config: Config,
    pub ns: String,
}

impl Env_ {
    pub fn new(config: Config) -> Self {
        let env = match config.map("config") {
            Some(value) => match value {
                Value::String(str) => Mu::env(&Mu::config(Some(str))),
                _ => {
                    eprintln!("mrepl: illegal config type: {value:?}",);
                    std::process::exit(-1)
                }
            },
            None => Mu::env(&Mu::config(None)),
        };

        let ns = match config.map("namespace") {
            Some(ns) => match ns {
                Value::String(ns) => match ns.as_str() {
                    "mu" => "mu".into(),
                    "core" => {
                        Self::load_sys(env, "core.sys");
                        "core"
                    }
                    "common" => {
                        Self::load_sys(env, "core.sys");
                        Self::load_sys(env, "common.fasl");
                        "common"
                    }
                    "prelude" => {
                        Self::load_sys(env, "core.sys");
                        Self::load_sys(env, "prelude.fasl");
                        "prelude"
                    }
                    _ => {
                        eprintln!("repl: unrecognized namespace: {ns}",);
                        std::process::exit(-1)
                    }
                },
                _ => {
                    eprintln!("mrepl: illegal namespace type: {ns:?}",);
                    std::process::exit(-1)
                }
            },
            None => "mu",
        };

        match config.map("system") {
            Some(rc) => match rc {
                Value::Array(vec) => {
                    for source in vec {
                        match source {
                            Value::String(path) => match Mu::load(env, path.as_str()) {
                                Ok(_) => (),
                                Err(e) => {
                                    eprintln!(
                                        "mrepl: can't load system file {path}: {}",
                                        Mu::exception_string(env, &e)
                                    );
                                    std::process::exit(-1)
                                }
                            },
                            _ => {
                                eprintln!("mrepl: illegal system path type: {source:?}",);
                                std::process::exit(-1)
                            }
                        }
                    }
                }
                _ => {
                    eprintln!("mrepl: illegal system type: {rc:?}",);
                    std::process::exit(-1)
                }
            },
            None => (),
        };

        match config.map("rc") {
            Some(rc) => match rc {
                Value::String(rc) => match Mu::load(env, rc.as_str()) {
                    Ok(bool_) => bool_,
                    Err(e) => {
                        eprintln!(
                            "mrepl: can't load rc file {rc}: {}",
                            Mu::exception_string(env, &e)
                        );
                        std::process::exit(-1)
                    }
                },
                _ => {
                    eprintln!("mrepl: illegal rc type: {rc:?}",);
                    std::process::exit(-1)
                }
            },
            None => false,
        };

        Self {
            env,
            config,
            ns: ns.into(),
        }
    }

    pub fn load_sys(env: Env, name: &str) {
        let sys = format!("/opt/mu/lib/{name}");

        match Mu::load(env, sys.as_str()) {
            Ok(bool_) => {
                if !bool_ {
                    eprintln!("repl: can't load {name}");
                    std::process::exit(-1)
                }
            }
            Err(e) => {
                eprintln!(
                    "repl: exception while loading {name}: {}",
                    Mu::exception_string(env, &e)
                );
                std::process::exit(-1)
            }
        }
    }
}
