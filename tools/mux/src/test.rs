//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::process::Command,
};

pub struct Test {}

impl Test {
    pub fn test(options: &Options) {
        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux test:"),
            None => (),
        };

        let mut test = Command::new("make")
            .args(["-C", "tests/regression"])
            .arg("summary")
            .arg("--no-print-directory")
            .spawn()
            .unwrap();

        test.wait().unwrap();
    }
}
