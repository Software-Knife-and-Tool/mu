//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Opt, Options},
    std::process::Command,
};

pub struct Reference {}

impl Reference {
    pub fn reference(options: &Options) {
        let ns_opt = options.options.iter().find(|opt| match opt {
            Opt::Namespace(_) => true,
            _ => false,
        });

        let ns_str = match ns_opt {
            None => "mu",
            Some(opt) => match opt {
                Opt::Namespace(ns) => ns,
                _ => panic!(),
            },
        };

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
