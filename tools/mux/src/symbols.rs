//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::{
        fs::File,
        io::{self, Write},
        process::Command,
    },
};

pub struct Symbols {}

impl Symbols {
    pub fn symbols(options: &Options, home: &str) {
        let report_opt = options.options.iter().find(|opt| match opt {
            Opt::Crossref => true,
            Opt::Counts => true,
            Opt::Reference => true,
            _ => false,
        });

        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux symbols: {report_opt:?}"),
            None => (),
        };

        match report_opt {
            Some(opt) => match opt {
                Opt::Crossref => Self::crossref(options, home),
                Opt::Counts => Self::counts(options, home),
                Opt::Reference => Self::reference(options, home),
                _ => {
                    eprintln!("mux repl: unmapped symbol report {opt:?}");
                    std::process::exit(-1)
                }
            },
            None => {
                eprintln!("mux repl: unspecified symbol report");
                std::process::exit(-1)
            }
        }
    }

    fn counts(_options: &Options, home: &str) {
        let output = Command::new("make")
            .current_dir(home)
            .args(["-C", "tools/symbol-counts"])
            .arg("core")
            .output()
            .expect("command failed to execute");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    fn crossref(_options: &Options, home: &str) {
        let output = Command::new("make")
            .current_dir(home)
            .args(["-C", "tools/crossref"])
            .arg("crossref")
            .output()
            .expect("command failed to execute");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    fn reference(options: &Options, home: &str) {
        let ns_opt = options.options.iter().find(|opt| match opt {
            Opt::Namespace(_) => true,
            _ => false,
        });

        let output_opt = options.options.iter().find(|opt| match opt {
            Opt::Output(_) => true,
            _ => false,
        });

        let ns_str = match ns_opt {
            None => "core",
            Some(opt) => match opt {
                Opt::Namespace(ns) => ns,
                _ => panic!(),
            },
        };

        match output_opt {
            Some(opt) => match opt {
                Opt::Output(path) => {
                    let out_file = File::create(path).expect(&format!("failed to open {path}"));

                    match ns_str {
                        "mu" => {
                            let output = Command::new("make")
                                .current_dir(home)
                                .args(["-C", "tools/reference"])
                                .arg("--no-print-directory")
                                .arg("mu")
                                .stdout(out_file)
                                .output()
                                .expect("command failed to execute");

                            io::stdout().write_all(&output.stdout).unwrap();
                            io::stderr().write_all(&output.stderr).unwrap();
                        }
                        "core" => {
                            let output = Command::new("make")
                                .current_dir(home)
                                .args(["-C", "tools/reference"])
                                .arg("--no-print-directory")
                                .arg("core")
                                .stdout(out_file)
                                .output()
                                .expect("command failed to execute");

                            io::stdout().write_all(&output.stdout).unwrap();
                            io::stderr().write_all(&output.stderr).unwrap();
                        }
                        "prelude" => {
                            panic!()
                        }
                        _ => {
                            eprintln!("mux repl: unmapped namespace {ns_str}");
                            std::process::exit(-1)
                        }
                    };
                }
                _ => panic!(),
            },
            None => {
                match ns_str {
                    "mu" => {
                        let output = Command::new("make")
                            .current_dir(home)
                            .args(["-C", "tools/reference"])
                            .arg("--no-print-directory")
                            .arg("mu")
                            .output()
                            .expect("command failed to execute");

                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    "core" => {
                        let output = Command::new("make")
                            .current_dir(home)
                            .args(["-C", "tools/reference"])
                            .arg("--no-print-directory")
                            .arg("core")
                            .output()
                            .expect("command failed to execute");

                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    "prelude" => {
                        panic!()
                    }
                    _ => panic!(),
                };
            }
        }
    }
}
