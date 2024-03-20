//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

pub mod listener;
pub mod server;
pub mod server_config;

use {
    crate::{
        server::{delay, Server},
        server_config::ServerConfig,
    },
    async_std::task::spawn,
    std::time::Duration,
};

//
// entry point
//
fn main() {
    let server = Server::new();
    let _config = ServerConfig::new();

    server.spawn(async {
        spawn(async {
            delay(Duration::from_millis(100)).await;
            println!("world");
        });

        spawn(async {
            println!("hello");
        });

        delay(Duration::from_millis(200)).await;
        std::process::exit(0);
    });

    server.run();
}
