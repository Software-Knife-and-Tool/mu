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
    pub fn commit(argv: &Vec<String>) {
        match Options::parse_options(argv, &[], &["verbose"]) {
            None => (),
            Some(options) => {
                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("mux commit: --verbose"),
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
    }
}
