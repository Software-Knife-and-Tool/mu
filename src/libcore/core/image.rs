//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image management
#[allow(unused_imports)]
use crate::core::{
    apply::Core as _,
    exception::{self},
    frame::Frame,
    mu::Mu,
    types::{Tag, Type},
};

pub trait Core {
    fn save_and_exit(_: &Mu, _: String) -> exception::Result<()>;
}

impl Core for Mu {
    fn save_and_exit(_: &Mu, _: String) -> exception::Result<()> {
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {}
