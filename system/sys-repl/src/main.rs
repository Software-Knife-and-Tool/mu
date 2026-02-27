//  SPDX-FileCopyrightText: Copyright 2026 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod config;
mod repl;

use {config::Config, mu::Mu, std::fs};

pub fn main() {
    let rc_path: Option<String> = if fs::metadata("./.sys-repl").is_ok() {
        Some("./.sys-repl".to_string())
    } else if fs::metadata("~/.sys-repl").is_ok() {
        Some("~/.sys-repl".to_string())
    } else {
        None
    };

    let config_json: Option<String> = match rc_path {
        Some(path) => Some(fs::read_to_string(path).expect("mu-load: failed to read .sys-repl")),
        None => None,
    };

    let config = Config::new(config_json);
    let env = Mu::env(&config.config);

    match config.load {
        Some(vec) => {
            for path in vec {
                Mu::load(&env, &path).expect("sys-repl: failed to load file");
            }
        }
        None => (),
    };

    let ns = match config.ns {
        Some(ns) => match ns.as_str() {
            "core" => {
                Mu::load(&env, "/opt/mu/lib/core.sys").expect("sys-repl: failed to load core.sys");
                ns
            }
            _ => ns,
        },
        None => "mu".to_string(),
    };

    repl::repl(&env, ns).expect("sys-repl: listener error");
}
