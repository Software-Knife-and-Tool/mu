//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::process::Command,
};

pub struct Commit {}

impl Commit {
    pub fn commit(options: &Options) {
        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux commit: fmt clippy"),
            None => (),
        };

        let mut fmt = Command::new("cargo").arg("fmt").spawn().unwrap();

        fmt.wait().unwrap();

        let mut clippy = Command::new("cargo").arg("clippy").spawn().unwrap();

        clippy.wait().unwrap();
    }
}
