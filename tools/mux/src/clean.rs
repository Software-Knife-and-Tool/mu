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
    pub fn clean(options: &Options, home: &str) {
        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux clean:"),
            None => (),
        };

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
