//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Mode, Opt, Options},
    std::{
        io::{self, Write},
        process::Command,
    },
};

pub struct Bench;

impl Bench {
    pub fn bench(argv: &Vec<String>, home: &str) {
        match Options::parse_options(
            argv,
            &["base", "current", "footprint"],
            &["ntests", "verbose"],
        ) {
            None => (),
            Some(options) => {
                if options.modes.len() != 1 {
                    panic!()
                }

                let mode = &options.modes[0];

                let ntests = match Options::opt_value(&options, &Opt::Ntests("".to_string())) {
                    Some(n) => n.parse().unwrap(),
                    None => 20usize,
                };

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("lade bench: {:?} --ntests {ntests} --verbose", mode),
                    None => (),
                };

                match mode {
                    Mode::Base => Self::base(ntests, home),
                    Mode::Current => Self::current(ntests, home),
                    Mode::Footprint => Self::footprint(ntests, home),
                    _ => panic!(),
                }
            }
        }
    }

    pub fn base(ntests: usize, home: &str) {
        for test_dir in ["tests/footprint", "tests/performance"] {
            let output = Command::new("make")
                .current_dir(home)
                .env("NTESTS", &ntests.to_string())
                .args(["-C", test_dir])
                .arg("base")
                .arg("--no-print-directory")
                .output()
                .expect("command failed to execute");

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
    }

    pub fn current(ntests: usize, home: &str) {
        let output = Command::new("make")
            .current_dir(home)
            .env("NTESTS", &ntests.to_string())
            .args(["-C", "tests/performance"])
            .arg("current")
            .arg("--no-print-directory")
            .output()
            .expect("command failed to execute");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        let output = Command::new("make")
            .current_dir(home)
            .args(["-C", "tests/performance"])
            .arg("report")
            .arg("--no-print-directory")
            .output()
            .expect("command failed to execute");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    pub fn footprint(_ntests: usize, home: &str) {
        let output = Command::new("make")
            .current_dir(home)
            .args(["-C", "tests/footprint"])
            .arg("current")
            .arg("--no-print-directory")
            .output()
            .expect("command failed to execute");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        let output = Command::new("make")
            .current_dir(home)
            .args(["-C", "tests/footprint"])
            .arg("report")
            .arg("--no-print-directory")
            .output()
            .expect("command failed to execute");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
