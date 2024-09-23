//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::{
        env::Env,
        options::{Opt, Options},
    },
    std::process::Command,
};

pub struct Build {}

impl Build {
    pub fn build(options: &Options) {
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

        let dist = match Env::mu_home(options) {
            Some(path) => format!("{path}/dist"),
            None => {
                let cwd = std::env::current_dir().unwrap();

                eprintln!(
                    "error: could not find `.mux` in {:?} or any parent directory",
                    cwd.to_str().unwrap()
                );
                return;
            }
        };

        let _ = match build_opt {
            Some(style) => match style {
                Opt::Debug => {
                    let mut build = Command::new("cargo")
                        .arg("build")
                        .arg("--workspace")
                        .spawn()
                        .unwrap();

                    let _ = build.wait();

                    let mut cp = Command::new("cp")
                        .arg("./target/debug/mu-exec")
                        .arg("./target/debug/mu-ld")
                        .arg("./target/debug/mu-server")
                        .arg("./target/debug/mu-sys")
                        .arg("./target/debug/mu-sh")
                        .arg("./target/debug/sysgen")
                        .arg(dist)
                        .spawn()
                        .unwrap();

                    cp.wait()
                }
                Opt::Release => {
                    let mut build = Command::new("cargo")
                        .arg("build")
                        .arg("--release")
                        .arg("--workspace")
                        .spawn()
                        .unwrap();

                    let _ = build.wait();

                    let mut cp = Command::new("cp")
                        .arg("./target/release/mu-exec")
                        .arg("./target/release/mu-ld")
                        .arg("./target/release/mu-server")
                        .arg("./target/release/mu-sh")
                        .arg("./target/release/mu-sys")
                        .arg("./target/release/sysgen")
                        .arg(dist)
                        .spawn()
                        .unwrap();

                    cp.wait()
                }
                Opt::Profile => {
                    let mut build = Command::new("cargo")
                        .arg("build")
                        .args(["--release"])
                        .args(["-F", "prof"])
                        .arg("--workspace")
                        .spawn()
                        .unwrap();

                    let _ = build.wait();

                    let mut cp = Command::new("cp")
                        .arg("./target/release/mu-exec")
                        .arg("./target/release/mu-ld")
                        .arg("./target/release/mu-sh")
                        .arg("./target/release/mu-server")
                        .arg("./target/release/mu-sys")
                        .arg("./target/release/sysgen")
                        .arg(dist)
                        .spawn()
                        .unwrap();

                    cp.wait()
                }
                _ => panic!(),
            },

            None => {
                let mut build = Command::new("cargo")
                    .arg("build")
                    .arg("--workspace")
                    .spawn()
                    .unwrap();

                let _ = build.wait();

                let mut cp = Command::new("cp")
                    .arg("./target/debug/mu-exec")
                    .arg("./target/debug/mu-ld")
                    .arg("./target/debug/mu-server")
                    .arg("./target/debug/mu-sys")
                    .arg("./target/debug/mux")
                    .arg("./target/debug/sysgen")
                    .arg("./dist")
                    .spawn()
                    .unwrap();

                cp.wait()
            }
        };

        // let the dist makefile decide how to do this
        let mut dist = Command::new("make")
            .args(["-C", "./dist"])
            .arg("--no-print-directory")
            .spawn()
            .unwrap();

        let _ = dist.wait();
    }
}
