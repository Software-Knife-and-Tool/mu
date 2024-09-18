//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {crate::options::Options, std::process::Command};

pub struct Install {}

impl Install {
    pub fn install(_options: &Options) {
        let mut install = Command::new("make")
            .args(["-C", "./dist"])
            .args(["-f", "install.mk"])
            .arg("install")
            .arg("--no-print-directory")
            .spawn()
            .unwrap();

        let _ = install.wait();
    }
}
