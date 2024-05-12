//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! process time
use crate::{
    core::{
        env::Env,
        exception::{self},
        frame::Frame,
    },
    types::fixnum::Fixnum,
};
use cpu_time::ProcessTime;

pub trait CoreFunction {
    fn core_utime(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn core_utime(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match ProcessTime::try_now() {
            Ok(_) => match env.start_time.try_elapsed() {
                Ok(delta) => Fixnum::as_tag(delta.as_micros() as i64),
                Err(_) => panic!(),
            },
            Err(_) => panic!(),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
