//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::{
        io::{self, Write},
        process::Command,
    },
};

pub struct Build {}

impl Build {
    pub fn build(options: &Options, home: &str) {
        let build_opt = options.options.iter().find(|opt| match opt {
            Opt::Release | Opt::Profile | Opt::Debug => true,
            _ => false,
        });

        match options.find_opt(&Opt::Verbose) {
            Some(_) => match build_opt {
                Some(style) => match style {
                    Opt::Debug => println!("mux: build debug"),
                    Opt::Release => println!("mux: build release"),
                    Opt::Profile => println!("mux: build profile"),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            None => (),
        };

        let dist = &format!("{home}/dist");

        let _ = match build_opt {
            Some(style) => match style {
                Opt::Debug => {
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
                Opt::Release => {
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
                Opt::Profile => {
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
            },

            None => {
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
                    .arg("../target/debug/mux")
                    .arg("../target/debug/sysgen")
                    .arg("../dist")
                    .output()
                    .expect("command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
        };

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
