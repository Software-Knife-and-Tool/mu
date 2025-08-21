//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod config;
mod env_;
mod repl;

use crate::{config::Config, env_::Env_};

pub fn main() {
    let env = Env_::new(Config::new());

    repl::listener(&env).expect("core-sys: listener error");
}
