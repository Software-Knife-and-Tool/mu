//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Mode, Opt, Options},
    std::{
        io::{self, Write},
        process::Command,
    },
};

pub struct Build {}

impl Build {
    pub fn build(argv: &Vec<String>, home: &str) {
        match Options::parse_options(argv, &["debug", "release", "profile"], &["verbose"]) {
            None => (),
            Some(options) => {
                if options.modes.len() != 1 {
                    eprintln!("illegal options {:?}", argv);
                    std::process::exit(-1)
                }

                let mode = &options.modes[0];

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("mux build: {:?} --verbose", mode),
                    None => (),
                };

                let dist = &format!("{home}/dist");

                match mode {
                    Mode::Debug => {
                        let output = Command::new("cargo")
                            .arg("build")
                            .arg("--workspace")
                            .output()
                            .expect("command failed to execute");

                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();

                        let output = Command::new("cp")
                            .current_dir(dist.clone())
                            .arg("../target/debug/mu-exec")
                            .arg("../target/debug/mu-ld")
                            .arg("../target/debug/mu-server")
                            .arg("../target/debug/mu-sys")
                            .arg("../target/debug/mu-sh")
                            .arg("../target/debug/sysgen")
                            .arg(dist.clone())
                            .output()
                            .expect("command failed to execute");

                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    Mode::Release => {
                        let output = Command::new("cargo")
                            .arg("build")
                            .arg("--release")
                            .arg("--workspace")
                            .output()
                            .expect("command failed to execute");

                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();

                        let output = Command::new("cp")
                            .current_dir(dist.clone())
                            .arg("../target/release/mu-exec")
                            .arg("../target/release/mu-ld")
                            .arg("../target/release/mu-server")
                            .arg("../target/release/mu-sh")
                            .arg("../target/release/mu-sys")
                            .arg("../target/release/sysgen")
                            .arg(dist.clone())
                            .output()
                            .expect("command failed to execute");

                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    Mode::Profile => {
                        let output = Command::new("cargo")
                            .arg("build")
                            .args(["--release"])
                            .args(["-F", "prof"])
                            .arg("--workspace")
                            .output()
                            .expect("command failed to execute");

                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();

                        let output = Command::new("cp")
                            .current_dir(dist.clone())
                            .arg("../target/release/mu-exec")
                            .arg("../target/release/mu-ld")
                            .arg("../target/release/mu-sh")
                            .arg("../target/release/mu-server")
                            .arg("../target/release/mu-sys")
                            .arg("../target/release/sysgen")
                            .arg(dist.clone())
                            .output()
                            .expect("command failed to execute");

                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    _ => panic!(),
                }

                // let the dist makefile decide how to do this
                let output = Command::new("make")
                    .current_dir(dist.clone())
                    .args(["-C", "../dist"])
                    .arg("--no-print-directory")
                    .output()
                    .expect("command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
    }
}
