//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::{
        env,
        io::{self, Write},
        process::Command,
    },
};

pub struct Init {}

impl Init {
    pub fn init(argv: &Vec<String>) {
        match Options::parse_options(argv, &[], &["verbose"]) {
            None => (),
            Some(options) => {
                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => {
                        println!("manifest init {:?}: --verbose", env::current_dir().unwrap())
                    }
                    None => (),
                };

                let output = Command::new("touch")
                    .arg(".manifest")
                    .output()
                    .expect("command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
    }
}
