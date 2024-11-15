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
    pub fn init(options: &Options) {
        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux init: {:?}", env::current_dir().unwrap()),
            None => (),
        };

        let output = Command::new("touch")
            .arg(".mux")
            .output()
            .expect("command failed to execute");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
