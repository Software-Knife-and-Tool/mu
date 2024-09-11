//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use crate::options::{Opt, Options};

pub struct Profile {}

impl Profile {
    pub fn profile(options: &Options) {
        for opt in &options.options {
            match opt {
                Opt::Load(path) => println!("load: {path}"),
                Opt::Eval(expr) => println!("eval: {expr}"),
                _ => {
                    eprintln!("profile: (switches: load, eval)");
                    eprintln!(
                        "         unrecognized switch {}",
                        Options::option_name(opt.clone())
                    )
                }
            }
        }
    }
}
