//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

use {
    crate::config::Config,
    mu::{Env, Mu},
};

pub struct Env_ {
    pub env: Env,
    pub config: Config,
}

impl Env_ {
    pub fn new(config: Config) -> Self {
        let env = match config.map("config") {
            Some(config) => {
                Mu::make_env(&Mu::config(Some(config)).expect("mcore: unable to allocate env"))
            }
            None => Mu::make_env(&Mu::config(None).expect("mcore: unable to allocate env")),
        };

        match Mu::load(env, "/opt/mu/lib/core.fasl") {
            Ok(bool_) => {
                if !bool_ {
                    eprintln!("mcore: can't load core.fasl");
                    std::process::exit(-1)
                }
            }
            Err(e) => {
                eprintln!(
                    "mcore: can't load core.fasl: {}",
                    Mu::exception_string(env, e)
                );
                std::process::exit(-1)
            }
        }

        match config.map("rc") {
            Some(rc) => match Mu::load(env, rc.as_str()) {
                Ok(bool_) => bool_,
                Err(e) => {
                    eprintln!(
                        "mcore: can't load rc file {rc}: {}",
                        Mu::exception_string(env, e)
                    );
                    std::process::exit(-1)
                }
            },
            None => false,
        };

        Self { env, config }
    }
}
