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
    pub fn clean(argv: &Vec<String>, home: &str) {
        match Options::parse_options(argv, &[], &["verbose"]) {
            None => (),
            Some(options) => {
                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("mux clean: --verbose"),
                    None => (),
                };

                let _dist = &format!("{home}/dist");

                let dirs = ["dist", "tests/regression", "tests/footprint"];

                let mu = match Env::mu_home() {
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

                Command::new("rm")
                    .current_dir(home)
                    .arg("-rf")
                    .arg(mu.clone() + "/target")
                    .arg(mu.clone() + "/Cargo.lock")
                    .arg(mu.clone() + "/TAGS")
                    .spawn()
                    .expect("command failed to execute");

                for dir in dirs {
                    Command::new("make")
                        .current_dir(home)
                        .args(["-C", &(mu.clone() + "/" + dir)])
                        .arg("clean")
                        .arg("--no-print-directory")
                        .spawn()
                        .expect("command faled to execute");
                }
            }
        }
    }
}
