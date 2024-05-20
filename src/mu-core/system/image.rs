//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image management
use crate::core::{
    env::Env,
    exception::{self},
};
use std::fs;

pub trait Image {
    fn save_and_exit(_: &Env, _: String) -> exception::Result<()>;
    fn load_image(_: &Env, _: String) -> exception::Result<()>;
}

impl Image for Env {
    fn save_and_exit(_env: &Env, path: String) -> exception::Result<()> {
        match fs::File::create(path) {
            Ok(_) => std::process::exit(0),
            Err(_) => Ok(())
        }
    }
    
    fn load_image(_: &Env, path: String) -> exception::Result<()> {
        match fs::File::open(path) {
            Ok(_) => std::process::exit(0),
            Err(_) => Ok(())
        }
    }
}

#[cfg(test)]
mod tests {}
