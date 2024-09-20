//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::process::Command,
};

pub struct Install {}

impl Install {
    pub fn install(options: &Options) {
        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux install:"),
            None => (),
        };

        let mut install = Command::new("make")
            .args(["-C", "./dist"])
            .args(["-f", "install.mk"])
            .arg("install")
            .arg("--no-print-directory")
            .spawn()
            .unwrap();

        let _ = install.wait();
    }
}
