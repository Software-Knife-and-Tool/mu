//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {crate::options::Options, std::process::Command};

pub struct Exec {}

impl Exec {
    pub fn repl(_options: &Options) {
        let mut child = Command::new("mu-sys").spawn().unwrap();

        let _ = child.wait();
    }
}
