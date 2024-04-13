//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image management
use crate::core::{
    env::Env,
    exception::{self},
};

pub trait Core {
    fn save_and_exit(_: &Env, _: String) -> exception::Result<()>;
}

impl Core for Env {
    fn save_and_exit(_: &Env, _: String) -> exception::Result<()> {
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {}
