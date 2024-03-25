//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! posix interface
use {
    crate::{
        core::{
            exception::{self, Condition, Exception},
            frame::Frame,
            mu::Mu,
            types::Tag,
        },
        system::System,
        types::{
            fixnum::Fixnum,
            vector::{Core as _, Vector},
        },
    },
    rustix::process,
};

pub trait MuFunction {
    fn posix_getpid(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn posix_getcwd(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for System {
    fn posix_getpid(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::as_tag(process::getpid().as_raw_nonzero().get() as i64);

        Ok(())
    }

    fn posix_getcwd(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match process::getcwd(vec![]) {
            Ok(cstring) => match cstring.into_string() {
                Ok(string) => Vector::from_string(&string).evict(mu),
                Err(_) => return Err(Exception::new(Condition::Syscall, "getcwd", Tag::nil())),
            },
            Err(_) => return Err(Exception::new(Condition::Syscall, "getcwd", Tag::nil())),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
