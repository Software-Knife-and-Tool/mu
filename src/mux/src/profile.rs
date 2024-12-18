//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use crate::options::{Opt, Options};

use std::process::Command;

pub struct Profile {}

impl Profile {
    pub fn profile(argv: &Vec<String>, _home: &str) {
        match Options::parse_options(argv, &[], &["profile-config", "verbose"]) {
            None => (),
            Some(options) => {
                let config_opt = Options::opt_value(&options, &Opt::Config("".to_string()));

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("mux profile: --config {config_opt:?} --verbose"),
                    None => (),
                };

                match config_opt {
                    Some(config) => {
                        let profile_expr =
                            &format!("((:lambda (config)                      \
                                      (prelude:load (mu:car config) ())       \
                                      ((:lambda (entry)                       \
                                      (prof:prof-control :on)                 \
                                      (mu:apply (mu:symbol-value entry) (mu:nthcdr 2 config)) \
                                      (prof:prof-control :off)                \
                                      (core:%map-vector                       \
                                      (:lambda (fn-count)                     \
                                      (core:%format :t \"~A~T~A~%\" `(,(mu:car fn-count) ,(mu:cdr fn-count)))) \
                                      (prof:prof-control :get)))              \
                                      (mu:intern mu:%null-ns% (mu:nth 1 config) ()))) \
                                      '{})", config);

                        Command::new("mu-sys")
                            .args(["-l", "/opt/mu/dist/core.l"])
                            .args(["-q", "(core:require-lib \"common\")"])
                            .args(["-q", "(core:require-lib \"prelude\")"])
                            .args(["-e", profile_expr])
                            .spawn()
                            .expect("command failed to exexcute");
                    }
                    None => {
                        eprintln!("profile switches: config output annotate");
                        if options.options.is_empty() {
                            eprintln!("     config option required")
                        } else {
                            eprintln!("     unrecognized switch(es)");
                            for opt in options.options {
                                eprintln!("     {:?}", opt.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
