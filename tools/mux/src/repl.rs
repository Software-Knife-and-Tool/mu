//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Mode, Opt, Options},
    std::process::Command,
};

pub struct Repl {}

impl Repl {
    pub fn repl(argv: &Vec<String>, _home: &str) {
        match Options::parse_options(argv, &["mu", "core", "perelude"], &["verbose"]) {
            None => (),
            Some(options) => {
                if options.modes.len() != 1 {
                    eprintln!("illegal options: {argv:?}");
                    std::process::exit(-1)
                }

                let ns = &options.modes[0];

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("mux repl: {ns:?} --verbose"),
                    None => (),
                };

                let mut child = match ns {
                    Mode::Mu => Command::new("mu-sh").spawn().unwrap(),
                    Mode::Core => Command::new("mu-sh")
                        .args(["-l", "/opt/mu/dist/core.l"])
                        .spawn()
                        .unwrap(),
                    Mode::Prelude => Command::new("mu-sys")
                        .args(["-l", "/opt/mu/dist/core.l"])
                        .args(["-q", "(core:require-lib \"prelude\")"])
                        .args(["-q", "(core:require-lib \"repl\")"])
                        .args(["-e", "(repl:repl)"])
                        .spawn()
                        .unwrap(),
                    _ => {
                        eprintln!("mux repl: unmapped namespace {ns:?}");
                        std::process::exit(-1)
                    }
                };

                let _ = child.wait();
            }
        }
    }
}
