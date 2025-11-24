//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! system feature
#[rustfmt::skip]
use {
    crate::{
        core::{
            apply::Apply as _,
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            tag::Tag,
            type_::Type,
        },
        features::feature::Feature,
        types::{
            cons::Cons,
            fixnum::Fixnum,
            float::Float,
            struct_::Struct,
            symbol::Symbol,
            vector::Vector,
        },
    },
};

#[cfg(not(target_os = "macos"))]
use sysinfo_dot_h::{self};

pub trait System {
    fn feature() -> Feature;
}

impl System for Feature {
    fn feature() -> Feature {
        Feature {
            functions: Some(vec![
                ("exit", 1, Feature::system_exit),
                ("shell", 2, Feature::system_shell),
                ("sleep", 1, Feature::system_sleep),
                #[cfg(not(target_os = "macos"))]
                ("sysinfo", 0, Feature::system_sysinfo),
                ("uname", 0u16, Feature::system_uname),
            ]),
            symbols: None,
            namespace: "feature/system".into(),
        }
    }
}

pub trait CoreFn {
    fn system_uname(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn system_shell(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn system_exit(_: &Env, fp: &mut Frame) -> exception::Result<()>;
    fn system_sleep(_: &Env, fp: &mut Frame) -> exception::Result<()>;
    #[cfg(not(target_os = "macos"))]
    fn system_sysinfo(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Feature {
    fn system_shell(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("system:shell", &[Type::String, Type::List], fp)?;

        let command = fp.argv[0];
        let arg_list = fp.argv[1];

        let type_check = Cons::list_iter(env, arg_list).find(|arg| {
            !matches!(arg.type_of(), Type::Vector if Vector::type_of(env, *arg) == Type::Char)
        });

        let argv: Vec<String> = match type_check {
            Some(arg) => Err(Exception::new(env, Condition::Type, "system:shell", arg))?,
            None => Cons::list_iter(env, arg_list)
                .map(|arg| Vector::as_string(env, arg))
                .collect(),
        };

        let status = std::process::Command::new(Vector::as_string(env, command))
            .args(argv)
            .status();

        fp.value = match status {
            Err(_) => {
                return Err(Exception::new(
                    env,
                    Condition::Open,
                    "system:shell",
                    command,
                ))
            }
            Ok(exit_status) => {
                Fixnum::with_or_panic(usize::try_from(exit_status.code().unwrap()).unwrap())
            }
        };

        Ok(())
    }

    fn system_exit(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("system:exit", &[Type::Fixnum], fp)?;

        let rc = fp.argv[0];

        std::process::exit(i32::try_from(Fixnum::as_i64(rc)).unwrap());
    }

    fn system_sleep(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("system:sleep", &[Type::Float], fp)?;

        fp.value = fp.argv[0];

        let secs: f32 = Float::as_f32(env, fp.value);
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let usecs: u64 = (1e6 * secs) as u64;

        std::thread::sleep(std::time::Duration::from_micros(usecs));

        Ok(())
    }

    fn system_uname(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match nix::sys::utsname::uname() {
            Err(_) => {
                return Err(Exception::new(
                    env,
                    Condition::Type,
                    "system:uname",
                    Tag::nil(),
                ))
            }
            Ok(info) => {
                let uname = vec![Cons::list(
                    env,
                    &[
                        Cons::cons(
                            env,
                            Symbol::keyword("sysname"),
                            Vector::from(info.sysname().to_str().unwrap()).with_heap(env),
                        ),
                        Cons::cons(
                            env,
                            Symbol::keyword("node"),
                            Vector::from(info.nodename().to_str().unwrap()).with_heap(env),
                        ),
                        Cons::cons(
                            env,
                            Symbol::keyword("release"),
                            Vector::from(info.release().to_str().unwrap()).with_heap(env),
                        ),
                        Cons::cons(
                            env,
                            Symbol::keyword("version"),
                            Vector::from(info.version().to_str().unwrap()).with_heap(env),
                        ),
                        Cons::cons(
                            env,
                            Symbol::keyword("machine"),
                            Vector::from(info.machine().to_str().unwrap()).with_heap(env),
                        ),
                    ],
                )];

                Struct::new(env, "uname", uname).with_heap(env)
            }
        };

        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    fn system_sysinfo(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match sysinfo_dot_h::try_collect() {
            Err(_) => Err(Exception::new(
                env,
                Condition::Type,
                "sysinfo:sysinfo",
                Tag::nil(),
            ))?,
            Ok(sysinfo) => {
                #[allow(clippy::cast_precision_loss)]
                let loads0: f32 = sysinfo.loads[0] as f32;
                #[allow(clippy::cast_precision_loss)]
                let loads1: f32 = sysinfo.loads[1] as f32;
                #[allow(clippy::cast_precision_loss)]
                let loads2: f32 = sysinfo.loads[2] as f32;

                let sysinfo = vec![Cons::list(
                    env,
                    &[
                        Cons::cons(
                            env,
                            Vector::from("uptime").with_heap(env),
                            #[allow(clippy::cast_sign_loss)]
                            Fixnum::with_u64(env, sysinfo.uptime as u64)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("loads").with_heap(env),
                            Vector::from(vec![loads0, loads1, loads2]).with_heap(env),
                        ),
                        Cons::cons(
                            env,
                            Vector::from("totalram").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.totalram)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("freeram").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.freeram)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("sharedram").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.sharedram)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("bufferram").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.bufferram)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("totalswap").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.totalswap)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("freeswap").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.freeswap)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("procs").with_heap(env),
                            sysinfo.procs.into(),
                        ),
                        Cons::cons(
                            env,
                            Vector::from("totalhigh").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.totalhigh)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("freehigh").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.freehigh)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("mem_unit").with_heap(env),
                            sysinfo.mem_unit.into(),
                        ),
                    ],
                )];

                Vector::from(sysinfo).with_heap(env)
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
