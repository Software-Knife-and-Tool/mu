//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::{fs::File, process::Command},
};

pub struct Test {}

impl Test {
    pub fn test(options: &Options) {
        let ns_opt = options.options.iter().find(|opt| match opt {
            Opt::Namespace(_) => true,
            _ => false,
        });

        match ns_opt {
            None => {
                let mut child = Command::new("make").arg("tests/summary").spawn().unwrap();

                let _ = child.wait();
            }
            Some(opt) => match opt {
                Opt::Namespace(ns) => match (*ns).as_str() {
                    "mu" => {
                        let tmpfn = temp_file::empty();
                        let tmpfile = File::create(tmpfn.path()).unwrap();

                        let mut child = Command::new("make")
                            .args(["-C", "tests"])
                            .arg("--no-print-directory")
                            .arg("mu")
                            .stdout(tmpfile)
                            .spawn()
                            .unwrap();

                        child.wait().unwrap();

                        let mut child = Command::new("python3")
                            .arg("tests/summarize-ns.py")
                            .arg(tmpfn.path())
                            .spawn()
                            .unwrap();

                        child.wait().unwrap();
                    }
                    "prelude" => {
                        let tmpfn = temp_file::empty();
                        let tmpfile = File::create(tmpfn.path()).unwrap();

                        let mut child = Command::new("make")
                            .args(["-C", "tests"])
                            .arg("--no-print-directory")
                            .arg("prelude")
                            .stdout(tmpfile)
                            .spawn()
                            .unwrap();

                        child.wait().unwrap();

                        let mut child = Command::new("python3")
                            .arg("tests/summarize-ns.py")
                            .arg(tmpfn.path())
                            .spawn()
                            .unwrap();

                        child.wait().unwrap();
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
        }
    }
}
