//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use {
    crate::options::{Opt, Options},
    std::fs::ReadDir,
    std::path::{Path, PathBuf},
};

pub struct Env {}

impl Env {
    pub fn mu_home(_options: &Options) -> Option<String> {
        let mut cwd: PathBuf = std::env::current_dir().unwrap();
        loop {
            match Path::read_dir(&cwd) {
                Ok(mut dir) => match dir.find(|entry| match entry {
                    Ok(entry) => entry.file_name() == ".mu",
                    _ => false,
                }) {
                    Some(_) => return Some(cwd.to_str().unwrap().to_string()),
                    None => (),
                },
                _ => return None,
            }

            cwd = match cwd.parent() {
                Some(path) => path.to_path_buf(),
                None => return None,
            };
        }
    }

    pub fn printenv(options: &Options) {
        let ewd = std::env::current_dir().unwrap();

        match options.find_opt(&Opt::Verbose) {
            Some(_) => println!("mux env:"),
            None => (),
        };

        match Self::mu_home(options) {
            Some(path) => println!("{:?}", path),
            None => {
                println!(
                    "error: could not find `.mu` in {:?} or any parent directory",
                    ewd.to_str().unwrap()
                )
            }
        }
    }
}
