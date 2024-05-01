//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! process time
use crate::{
    core::{
        env::Env,
        exception::{self},
        frame::Frame,
        types::Tag,
    },
    types::fixnum::Fixnum,
};
use cpu_time::ProcessTime;

pub trait Core {
    fn utime(_: &Env, _: String) -> exception::Result<Tag>;
}

impl Core for Env {
    fn utime(_: &Env, _: String) -> exception::Result<Tag> {
        Ok(Tag::nil())
    }
}

pub trait CoreFunction {
    fn lib_utime(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn lib_utime(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match ProcessTime::try_now() {
            Err(_) => panic!(),
            Ok(_) => match env.start_time.try_elapsed() {
                Err(_) => panic!(),
                Ok(delta) => Fixnum::as_tag(delta.as_micros() as i64),
            },
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
