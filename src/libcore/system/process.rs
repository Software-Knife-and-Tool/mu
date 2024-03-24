//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! posix interface
use {
    crate::{
        core::{
            apply::Core as _,
            exception::{self, Condition, Exception},
            frame::Frame,
            mu::Mu,
            types::{Tag, Type},
        },
        system::System,
        types::{
            cons::{Cons, Core as _},
            fixnum::Fixnum,
            vector::{Core as _, Vector},
        },
    },
    rustix::process,
};

pub trait MuFunction {
    fn sys_spawn(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn posix_getpid(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn posix_getcwd(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn posix_exit(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for System {
    fn sys_spawn(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let command = fp.argv[0];
        let args = fp.argv[1];

        fp.value = match mu.fp_argv_check("spawn", &[Type::String, Type::List], fp) {
            Ok(_) => {
                let mut argv = vec![];

                for cons in Cons::iter(mu, args) {
                    let string = Cons::car(mu, cons);

                    match string.type_of() {
                        Type::Vector if Vector::type_of(mu, string) == Type::Char => {
                            let str = Vector::as_string(mu, string);
                            argv.push(str)
                        }
                        _ => return Err(Exception::new(Condition::Type, "system", string)),
                    }
                }

                let status = std::process::Command::new(Vector::as_string(mu, command))
                    .args(argv)
                    .status();

                match status {
                    Err(_) => return Err(Exception::new(Condition::Open, "spawn", command)),
                    Ok(exit_status) => Fixnum::as_tag(exit_status.code().unwrap() as i64),
                }
            }

            Err(e) => return Err(e),
        };

        Ok(())
    }

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

    fn posix_exit(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let rc = fp.argv[0];

        match rc.type_of() {
            Type::Fixnum => std::process::exit(Fixnum::as_i64(rc) as i32),
            _ => Err(Exception::new(Condition::Type, "exit", rc)),
        }
    }
}

#[cfg(test)]
mod tests {}
