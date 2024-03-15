//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image management
#[allow(unused_imports)]
use crate::{
    core::{
        exception::{self},
        frame::Frame,
        funcall::Core as _,
        mu::Mu,
        types::{Tag, Type},
    },
    system::System,
};

pub trait Core {
    fn save_and_exit(_: &Mu, _: String) -> exception::Result<()>;
}

impl Core for System {
    fn save_and_exit(_: &Mu, _: String) -> exception::Result<()> {
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {}
