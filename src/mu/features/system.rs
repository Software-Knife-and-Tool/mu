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
            vector::{Vector, VectorType},
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
            !matches!(arg.type_of(), Type::Vector if Vector::type_of(env, *arg) == VectorType::Char)
        });

        let argv: Vec<String> = match type_check {
            Some(arg) => Err(Exception::err(env, arg, Condition::Type, "system:shell"))?,
            None => Cons::list_iter(env, arg_list)
                .map(|arg| Vector::as_string(env, arg))
                .collect(),
        };

        let status = std::process::Command::new(Vector::as_string(env, command))
            .args(argv)
            .status();

        fp.value = match status {
            Err(_) => {
                return Err(Exception::err(
                    env,
                    command,
                    Condition::Open,
                    "system:shell",
                ))
            }
            Ok(exit_status) => match exit_status.code() {
                Some(rc) => Fixnum::etry_from(env, rc, "system:shell")?,
                None => Fixnum::etry_from(env, -1, "system:shell")?,
            },
        };

        Ok(())
    }

    fn system_exit(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("system:exit", &[Type::Fixnum], fp)?;

        std::process::exit(i32::from(fp.argv[0]));
    }

    fn system_sleep(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("system:sleep", &[Type::Float], fp)?;

        fp.value = fp.argv[0];

        let secs: f32 = Float::as_f32(env, fp.value);

        if secs.is_sign_negative() {
            Err(Exception::err(
                env,
                fp.value,
                Condition::Range,
                "system:sleep",
            ))?;
        }

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let usecs: u64 = 1_000_000 * (secs.abs().round() as u64);

        std::thread::sleep(std::time::Duration::from_micros(usecs));

        Ok(())
    }

    fn system_uname(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match nix::sys::utsname::uname() {
            Err(_) => Err(Exception::err(
                env,
                Tag::nil(),
                Condition::Type,
                "system:uname",
            ))?,
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
            Err(_) => Err(Exception::err(
                env,
                Tag::nil(),
                Condition::Type,
                "sysinfo:sysinfo",
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
                            Fixnum::etry_from(env, sysinfo.uptime, "system:sysinfo")?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("loads").with_heap(env),
                            Vector::from(vec![loads0, loads1, loads2]).with_heap(env),
                        ),
                        Cons::cons(
                            env,
                            Vector::from("totalram").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.totalram, "system:sysinfo")?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("freeram").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.freeram, "system:sysinfo")?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("sharedram").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.sharedram, "system:sysinfo")?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("bufferram").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.bufferram, "system:sysinfo")?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("totalswap").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.totalswap, "system:sysinfo")?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("freeswap").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.freeswap, "system:sysinfo")?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("procs").with_heap(env),
                            sysinfo.procs.into(),
                        ),
                        Cons::cons(
                            env,
                            Vector::from("totalhigh").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.totalhigh, "system:sysinfo")?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("freehigh").with_heap(env),
                            Fixnum::with_u64(env, sysinfo.freehigh, "system:sysinfo")?,
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
