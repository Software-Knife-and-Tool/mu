//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::{
        io::{self, Write},
        process::Command,
    },
};

pub struct Annotate {}

impl Annotate {
    pub fn annotate(argv: &Vec<String>, _home: &str) {
        match Options::parse_options(argv, &[], &["prof", "ref", "verbose"]) {
            None => (),
            Some(options) => {
                let mut prof_opt: Option<String> = None;
                let mut ref_opt: Option<String> = None;

                for opt in &options.options {
                    match opt {
                        Opt::Prof(path) => prof_opt = Some(path.to_string()),
                        Opt::Ref(path) => ref_opt = Some(path.to_string()),
                        _ => panic!("opt consistency"),
                    }
                }

                let profile_path = match prof_opt {
                    Some(path) => path,
                    None => panic!(),
                };

                let reference_path = match ref_opt {
                    Some(path) => path,
                    None => "ref.out".to_string(),
                };

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!(
                        "mux annotate: --prof {} --ref {}",
                        profile_path, reference_path
                    ),
                    None => (),
                };

                let output = Command::new("python3")
                    .arg("/opt/mu/bin/annotate.py")
                    .arg(profile_path)
                    .arg(reference_path)
                    .output()
                    .expect("command failed to execute");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
    }
}
