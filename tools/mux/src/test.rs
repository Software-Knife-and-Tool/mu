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
    pub fn test(options: &Options, home: &str) {
        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux test:"),
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
