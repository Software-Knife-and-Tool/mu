//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use crate::options::{Opt, Options};

use std::{fs::File, process::Command};

pub struct Profile {}

impl Profile {
    pub fn profile(options: &Options) {
        let config_opt = options.options.iter().find(|opt| match opt {
            Opt::Config(_) => true,
            _ => false,
        });

        let output_opt = options.options.iter().find(|opt| match opt {
            Opt::Output(_) => true,
            _ => false,
        });

        match config_opt {
            Some(opt) => match opt {
                Opt::Config(config) => {
                    let profile_expr =
                        &format!("((:lambda (config)                          \
                                      (prelude:load (mu:car config) ())       \
                                      ((:lambda (entry)                       \
                                          (prof:prof-control :on)             \
                                          (mu:apply (mu:symbol-value entry) (mu:nthcdr 2 config)) \
                                          (prof:prof-control :off)            \
                                          (core:%map-vector                   \
                                            (:lambda (fn-count)               \
                                               (core:format :t \"~A~T~A~%\" `(,(mu:car fn-count) ,(mu:cdr fn-count)))) \
                                            (prof:prof-control :get)))           \
                                        (mu:intern mu:%null-ns% (mu:nth 1 config) ()))) \
                                    '{})", config);

                    match output_opt {
                        Some(opt) => match opt {
                            Opt::Output(path) => {
                                let out_file =
                                    File::create(path).expect(&format!("failed to open {path}"));

                                Command::new("mu-sys")
                                    .arg("-p")
                                    .args(["-l", "/opt/mu/dist/core.l"])
                                    .args(["-l", "/opt/mu/dist/common.l"])
                                    .args(["-l", "/opt/mu/dist/prelude.l"])
                                    .args(["-e", profile_expr])
                                    .stdout(out_file)
                                    .spawn()
                                    .unwrap();
                            }
                            _ => (),
                        },
                        None => {
                            Command::new("mu-sys")
                                .arg("-p")
                                .args(["-l", "/opt/mu/dist/core.l"])
                                .args(["-l", "/opt/mu/dist/common.l"])
                                .args(["-l", "/opt/mu/dist/prelude.l"])
                                .args(["-e", profile_expr])
                                .spawn()
                                .unwrap();
                        }
                    }
                }
                _ => panic!(),
            },
            None => {
                eprintln!("profile switches: config output annotate");
                if options.options.is_empty() {
                    eprintln!("     config option required")
                } else {
                    eprintln!("     unrecognized switch(es)");
                    for opt in &options.options {
                        eprintln!("     {}", Options::option_name(opt.clone()))
                    }
                }
            }
        }
    }
}
