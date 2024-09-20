//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use crate::options::{Opt, Options};

use std::process::Command;

pub struct Annotate {}

impl Annotate {
    pub fn annotate(options: &Options) {
        let profile_opt = options.options.iter().find(|opt| match opt {
            Opt::Prof(_) => true,
            _ => false,
        });

        let profile_path = match profile_opt {
            Some(opt) => match opt {
                Opt::Prof(path) => path,
                _ => panic!(),
            },
            None => panic!(),
        };

        let reference_opt = options.options.iter().find(|opt| match opt {
            Opt::Ref(_) => true,
            _ => false,
        });

        let reference_path = match reference_opt {
            Some(opt) => match opt {
                Opt::Ref(path) => path,
                _ => panic!(),
            },
            None => panic!(),
        };

        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux annotate: prof {} ref {}", profile_path, reference_path),
            None => (),
        };

        let mut annotate = Command::new("python3")
            .arg("/opt/mu/bin/annotate.py")
            .arg(profile_path)
            .arg(reference_path)
            .spawn()
            .unwrap();

        annotate.wait().unwrap();
    }
}
