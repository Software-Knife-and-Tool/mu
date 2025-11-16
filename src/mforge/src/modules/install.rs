//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::{
        io::{self, Write},
        process::Command,
    },
};

pub struct Install {}

impl Install {
    pub fn install(argv: &Vec<String>, home: &str) {
        match Options::parse_options(argv, &[], &["verbose"]) {
            None => (),
            Some(options) => {
                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("mforge install {home}: --verbose"),
                    None => (),
                };

                let dist = &format!("{home}/dist");

                let output = Command::new("make")
                    .args(["-C", dist])
                    .args(["-f", "install.mk"])
                    .arg("install")
                    .arg("--no-print-directory")
                    .output()
                    .expect("command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
    }
}
