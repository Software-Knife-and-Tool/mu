//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::{
        io::{self, Write},
        process::Command,
    },
};

pub struct Bench {}

impl Bench {
    pub fn bench(options: &Options, home: &str) {
        let report_opt = options.options.iter().find(|opt| match opt {
            Opt::Base => true,
            Opt::Current => true,
            Opt::Footprint => true,
            _ => false,
        });

        match report_opt {
            Some(opt) => match opt {
                Opt::Base => Self::base(options, home),
                Opt::Current => Self::current(options, home),
                Opt::Footprint => Self::footprint(options, home),
                _ => panic!(),
            },
            None => Self::current(options, home),
        }
    }

    pub fn base(options: &Options, home: &str) {
        let ntests = match options.opt_value(&Opt::Ntests("".to_string())) {
            Some(n) => n.parse().unwrap(),
            None => 20u32,
        };

        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux bench: base ntests {ntests}"),
            None => (),
        };

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

    pub fn current(options: &Options, home: &str) {
        let ntests = match options.opt_value(&Opt::Ntests("".to_string())) {
            Some(n) => n.parse().unwrap(),
            None => 20u32,
        };

        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux bench: current ntests {ntests}"),
            None => (),
        };

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

    pub fn footprint(_options: &Options, home: &str) {
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
