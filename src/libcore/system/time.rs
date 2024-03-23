//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! posix interface
use {
    crate::{
        core::{
            exception::{self},
            frame::Frame,
            mu::Mu,
        },
        system::System,
        types::fixnum::Fixnum,
    },
    rustix::time::{self, ClockId},
};

pub trait MuFunction {
    fn sys_getrealtime(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn sys_getproctime(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for System {
    fn sys_getrealtime(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let timespec = time::clock_gettime(ClockId::Realtime);
        fp.value = Fixnum::as_tag(timespec.tv_sec * 1_000_000 + timespec.tv_nsec / 1_000);

        Ok(())
    }

    fn sys_getproctime(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let timespec = time::clock_gettime(ClockId::ProcessCPUTime);
        fp.value = Fixnum::as_tag(timespec.tv_sec * 1_000_000 + timespec.tv_nsec / 1_000);

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
