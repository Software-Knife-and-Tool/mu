//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod repl;

use mu::{Config, Mu};

pub fn main() {
    let env = Mu::env(&Config::new(None));

    repl::repl(&env, "mu".to_string()).expect("repl: listener error");
}
