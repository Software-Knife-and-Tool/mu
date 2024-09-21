//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::{fs::File, process::Command},
};

pub struct Symbols {}

impl Symbols {
    pub fn symbols(options: &Options) {
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
                Opt::Crossref => Self::crossref(options),
                Opt::Counts => Self::counts(options),
                Opt::Reference => Self::reference(options),
                _ => panic!(),
            },
            None => panic!(),
        }
    }

    fn counts(_options: &Options) {
        let path = std::path::Path::new("./tools/symbol-counts");
        std::env::set_current_dir(&path).unwrap();
        let mut child = Command::new("make").arg("core").spawn().unwrap();

        let _ = child.wait();
    }

    fn crossref(_options: &Options) {
        let path = std::path::Path::new("./tools/crossref");
        std::env::set_current_dir(&path).unwrap();
        let mut child = Command::new("make").arg("crossref").spawn().unwrap();

        let _ = child.wait();
    }

    fn reference(options: &Options) {
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

                    let mut child = match ns_str {
                        "mu" => Command::new("make")
                            .args(["-C", "tools/reference"])
                            .arg("--no-print-directory")
                            .arg("mu")
                            .stdout(out_file)
                            .spawn()
                            .unwrap(),
                        "core" => Command::new("make")
                            .args(["-C", "tools/reference"])
                            .arg("--no-print-directory")
                            .arg("core")
                            .stdout(out_file)
                            .spawn()
                            .unwrap(),
                        "prelude" => Command::new("mu-sys")
                            .args(["-l", "/opt/mu/lib/core/core.l"])
                            .args(["-l", "/opt/mu/lib/prelude/repl.l"])
                            .args(["-e", "(prelude:repl)"])
                            .stdout(out_file)
                            .spawn()
                            .unwrap(),
                        _ => panic!(),
                    };

                    let _ = child.wait();
                }
                _ => panic!(),
            },
            None => {
                let mut child = match ns_str {
                    "mu" => Command::new("make")
                        .args(["-C", "tools/reference"])
                        .arg("--no-print-directory")
                        .arg("mu")
                        .spawn()
                        .unwrap(),
                    "core" => Command::new("make")
                        .args(["-C", "tools/reference"])
                        .arg("--no-print-directory")
                        .arg("core")
                        .spawn()
                        .unwrap(),
                    "prelude" => Command::new("mu-sys")
                        .args(["-l", "/opt/mu/lib/core/core.l"])
                        .args(["-l", "/opt/mu/lib/prelude/repl.l"])
                        .args(["-q", "(prelude:%init-ns)"])
                        .args(["-e", "(prelude:repl)"])
                        .spawn()
                        .unwrap(),
                    _ => panic!(),
                };

                let _ = child.wait();
            }
        }
    }
}
