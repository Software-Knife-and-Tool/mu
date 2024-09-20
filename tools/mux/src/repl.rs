//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::process::Command,
};

pub struct Repl {}

impl Repl {
    pub fn repl(options: &Options) {
        let ns_str = match options.opt_value(&Opt::Namespace("".to_string())) {
            Some(ns) => &ns.clone(),
            None => "mu",
        };

        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux ns: {ns_str}"),
            None => (),
        };

        let mut child = match ns_str {
            "mu" => Command::new("mu-sys").spawn().unwrap(),
            "core" => Command::new("mu-sys")
                .args(["-l", "/opt/mu/dist/core.l"])
                .spawn()
                .unwrap(),
            "common" => Command::new("mu-sys")
                .args(["-l", "/opt/mu/dist/core.l"])
                .args(["-l", "/opt/mu/dist/common.l"])
                .spawn()
                .unwrap(),
            "prelude" => Command::new("mu-sys")
                .args(["-l", "/opt/mu/dist/core.l"])
                .args(["-l", "/opt/mu/dist/common.l"])
                .args(["-l", "/opt/mu/dist/prelude.l"])
                .args(["-l", "/opt/mu/lib/prelude/repl.l"])
                .args(["-e", "(prelude:repl)"])
                .spawn()
                .unwrap(),
            _ => panic!(),
        };

        let _ = child.wait();
    }
}
