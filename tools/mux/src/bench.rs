//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::process::Command,
};

pub struct Bench {}

impl Bench {
    pub fn bench(options: &Options) {
        let report_opt = options.options.iter().find(|opt| match opt {
            Opt::Base => true,
            Opt::Current => true,
            Opt::Footprint => true,
            _ => false,
        });

        match report_opt {
            Some(opt) => match opt {
                Opt::Base => Self::base(options),
                Opt::Current => Self::current(options),
                Opt::Footprint => Self::footprint(options),
                _ => panic!(),
            },
            None => Self::current(options),
        }
    }

    pub fn base(options: &Options) {
        let test_opt = options.options.iter().find(|opt| match opt {
            Opt::Ntests(_) => true,
            _ => false,
        });

        let ntests = match test_opt {
            Some(opt) => match opt {
                Opt::Ntests(n) => *n,
                _ => panic!(),
            },
            None => 20u32,
        };

        for test_dir in ["tests/footprint", "tests/performance"] {
            let mut test = Command::new("make")
                .env("NTESTS", &ntests.to_string())
                .args(["-C", test_dir])
                .arg("base")
                .arg("--no-print-directory")
                .spawn()
                .unwrap();

            test.wait().unwrap();
        }
    }

    pub fn current(options: &Options) {
        let test_opt = options.options.iter().find(|opt| match opt {
            Opt::Ntests(_) => true,
            _ => false,
        });

        let ntests = match test_opt {
            Some(opt) => match opt {
                Opt::Ntests(n) => *n,
                _ => panic!(),
            },
            None => 20u32,
        };

        let mut test = Command::new("make")
            .env("NTESTS", &ntests.to_string())
            .args(["-C", "tests/performance"])
            .arg("current")
            .arg("--no-print-directory")
            .spawn()
            .unwrap();

        test.wait().unwrap();

        let mut report = Command::new("make")
            .args(["-C", "tests/performance"])
            .arg("report")
            .arg("--no-print-directory")
            .spawn()
            .unwrap();

        report.wait().unwrap();
    }

    pub fn footprint(_options: &Options) {
        let mut test = Command::new("make")
            .args(["-C", "tests/footprint"])
            .arg("current")
            .arg("--no-print-directory")
            .spawn()
            .unwrap();

        test.wait().unwrap();

        let mut report = Command::new("make")
            .args(["-C", "tests/footprint"])
            .arg("report")
            .arg("--no-print-directory")
            .spawn()
            .unwrap();

        report.wait().unwrap();
    }
}
