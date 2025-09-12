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
            Some(config) => Mu::make_env(
                &Mu::config(Some(config))
                    .expect("core-cli: unable to allocate env from config {config:?}"),
            ),
            None => Mu::make_env(&Mu::config(None).expect("core-cli: unable to allocate env")),
        };

        match Mu::load(env, "/opt/mu/lib/core.sys") {
            Ok(bool_) => {
                if !bool_ {
                    eprintln!("core-cli: can't load core.sys");
                    std::process::exit(-1)
                }
            }
            Err(e) => {
                eprintln!(
                    "core-cli: can't load core.sys: {}",
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
                        "core-cli: can't load rc file {rc}: {}",
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
