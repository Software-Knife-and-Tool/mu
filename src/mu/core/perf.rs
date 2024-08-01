//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! perf
// #![allow(unused_braces)]
// #![allow(clippy::identity_op)]
use crate::core::{
    env::Env,
    exception::{self},
    types::Tag,
};

pub trait Core {
    fn event(&self, _: &str, _: &Vec<Tag>) -> exception::Result<()>;
}

impl Core for Env {
    fn event(&self, _label: &str, _argv: &Vec<Tag>) -> exception::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn perf() {
        assert_eq!(2 + 2, 4);
    }
}
