//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! process time
use crate::core::{
    env::Env,
    exception::{self},
    frame::Frame,
    types::Tag,
};
use cpu_time::ProcessTime;

pub trait CoreFunction {
    fn crux_utime(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn crux_utime(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match ProcessTime::try_now() {
            Ok(_) => match env.start_time.try_elapsed() {
                Ok(delta) => Tag::from(delta.as_micros() as i64), // this is a u128
                Err(_) => panic!(),
            },
            Err(_) => panic!(),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
