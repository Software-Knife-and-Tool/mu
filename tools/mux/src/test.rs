//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::{
        io::{self, Write},
        process::Command,
    },
};

pub struct Test {}

impl Test {
    pub fn test(argv: &Vec<String>, home: &str) {
        match Options::parse_options(argv, &[], &["verbose"]) {
            None => (),
            Some(options) => {
                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("mux repl: --verbose"),
                    None => (),
                };

                let output = Command::new("make")
                    .current_dir(home)
                    .args(["-C", "tests/regression"])
                    .arg("summary")
                    .arg("--no-print-directory")
                    .output()
                    .expect("command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
    }
}
