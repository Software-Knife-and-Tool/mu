//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::{
        env::Env,
        options::{Opt, Options},
    },
    std::process::Command,
};

pub struct Clean {}

impl Clean {
    pub fn clean(options: &Options) {
        let dirs = ["dist", "tests/regression", "tests/footprint"];

        let mu = match Env::mu_home(options) {
            Some(path) => path,
            None => {
                let cwd = std::env::current_dir().unwrap();

                eprintln!(
                    "error: could not find `.mu` in {:?} or any parent directory",
                    cwd.to_str().unwrap()
                );
                return;
            }
        };

        match options.options.iter().find(|opt| match opt {
            Opt::Verbose => true,
            _ => false,
        }) {
            Some(_) => println!("mux clean: {dirs:?}"),
            None => (),
        };

        let mut home = Command::new("rm")
            .arg("-rf")
            .arg(mu.clone() + "/target")
            .arg(mu.clone() + "/Cargo.lock")
            .arg(mu.clone() + "/TAGS")
            .spawn()
            .unwrap();

        home.wait().unwrap();

        for dir in dirs {
            let mut clean = Command::new("make")
                .args(["-C", &(mu.clone() + "/" + dir)])
                .arg("clean")
                .arg("--no-print-directory")
                .spawn()
                .unwrap();

            clean.wait().unwrap();
        }
    }
}
