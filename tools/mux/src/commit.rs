//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::{
        io::{self, Write},
        process::Command,
    },
};

pub struct Commit {}

impl Commit {
    pub fn commit(options: &Options, _: &str) {
        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux commit: fmt clippy"),
            None => (),
        };

        for cmd in vec!["fmt", "clippy", "test"] {
            let output = Command::new("cargo")
                .arg(cmd)
                .output()
                .expect("command failed to execute");

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
    }
}
