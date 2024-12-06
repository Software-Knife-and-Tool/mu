//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Mode, Opt, Options},
    std::{
        io::{self, Write},
        process::Command,
    },
};

pub struct Symbols {}

impl Symbols {
    pub fn symbols(argv: &Vec<String>, home: &str) {
        match Options::parse_options(
            argv,
            &["crossref", "reference", "metrics"],
            &["module", "namespace", "verbose"],
        ) {
            None => (),
            Some(options) => {
                if options.modes.len() != 1 {
                    eprintln!("illegal options: {argv:?}");
                    std::process::exit(-1)
                }

                let mode = &options.modes[0];

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("mux symbols: {:?}", mode),
                    None => (),
                };

                match options.modes[0] {
                    Mode::Crossref => Self::crossref(&options, home),
                    Mode::Metrics => Self::metrics(&options, home),
                    Mode::Reference => Self::reference(&options, home),
                    _ => panic!(),
                }
            }
        }
    }

    fn metrics(options: &Options, home: &str) {
        let ns_opt = options.options.iter().find(|opt| match opt {
            Opt::Namespace(_) => true,
            _ => false,
        });

        let ns_str = match ns_opt {
            None => "core",
            Some(opt) => match opt {
                Opt::Namespace(ns) => ns,
                _ => panic!(),
            },
        };

        match ns_str {
            "core" => {
                let output = Command::new("make")
                    .current_dir(home)
                    .args(["-C", "tools/metrics"])
                    .arg("core")
                    .arg("--no-print-directory")
                    .output()
                    .expect("command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
            "prelude" => {
                let output = Command::new("make")
                    .current_dir(home)
                    .args(["-C", "tools/metrics"])
                    .arg("prelude")
                    .arg("--no-print-directory")
                    .output()
                    .expect("command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
            _ => panic!(),
        }
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

        let ns_str = match ns_opt {
            None => "core",
            Some(opt) => match opt {
                Opt::Namespace(ns) => ns,
                _ => panic!(),
            },
        };

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
            _ => {
                let module = match Options::opt_value(&options, &Opt::Module("".to_string())) {
                    Some(name) => name,
                    None => {
                        eprintln!("symbols reference: --module required");
                        std::process::exit(-1)
                    }
                };

                let ns = match Options::opt_value(&options, &Opt::Namespace("".to_string())) {
                    Some(name) => name,
                    None => {
                        eprintln!("symbols reference: --namespace required");
                        std::process::exit(-1)
                    }
                };

                let output = Command::new("/opt/mu/bin/mu-sys")
                    .current_dir(format!("{home}/tools/reference"))
                    .args(["-l", "/opt/mu/dist/core.l"])
                    .args(["-q", &format!("(core:require \"{module}\")")])
                    .args(["-l", "./reference.l"])
                    .args(["-q", &format!("(reference \"{ns}\" \"reference.out\")")])
                    .output()
                    .expect("mu-sys command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();

                let output = Command::new("python3")
                    .current_dir(format!("{home}/tools/reference"))
                    .arg("reference.py")
                    .arg("reference.out")
                    .output()
                    .expect("command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();

                /*
                let output = Command::new("rm")
                    .current_dir(format!("{home}/tools/reference"))
                    .arg("reference.out")
                    .output()
                    .expect("command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
                */
            }
        }
    }
}
