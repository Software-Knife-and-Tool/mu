//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! process time
use crate::{
    core::{
        exception::{self},
        frame::Frame,
        mu::Mu,
        types::Tag,
    },
    types::fixnum::Fixnum,
};
use cpu_time::ProcessTime;

pub trait Core {
    fn utime(_: &Mu, _: String) -> exception::Result<Tag>;
}

impl Core for Mu {
    fn utime(_: &Mu, _: String) -> exception::Result<Tag> {
        Ok(Tag::nil())
    }
}

pub trait LibFunction {
    fn lib_utime(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Mu {
    fn lib_utime(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match ProcessTime::try_now() {
            Err(_) => panic!(),
            Ok(_) => match mu.start_time.try_elapsed() {
                Err(_) => panic!(),
                Ok(delta) => Fixnum::as_tag(delta.as_micros() as i64),
            },
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
