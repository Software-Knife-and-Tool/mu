//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {crate::options::Options, std::process::Command};

pub struct Counts {}

impl Counts {
    pub fn counts(_options: &Options) {
        let path = std::path::Path::new("./tools/symbol-counts");
        std::env::set_current_dir(&path).unwrap();
        let mut child = Command::new("make").arg("core").spawn().unwrap();

        let _ = child.wait();
    }
}
