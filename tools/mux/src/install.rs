//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::{
        env::Env,
        options::{Opt, Options},
    },
    std::process::Command,
};

pub struct Install {}

impl Install {
    pub fn install(options: &Options) {
        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux install:"),
            None => (),
        };

        let dist = match Env::mu_home(options) {
            Some(path) => format!("{path}/dist"),
            None => {
                let cwd = std::env::current_dir().unwrap();

                eprintln!(
                    "error: could not find `.mux` in {:?} or any parent directory",
                    cwd.to_str().unwrap()
                );
                return;
            }
        };

        let mut install = Command::new("make")
            .args(["-C", &dist])
            .args(["-f", "install.mk"])
            .arg("install")
            .arg("--no-print-directory")
            .spawn()
            .unwrap();

        let _ = install.wait();
    }
}
