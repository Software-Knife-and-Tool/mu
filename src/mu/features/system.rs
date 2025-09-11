//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! system feature
#[rustfmt::skip]
use {
    crate::{
        core::{
            apply::Apply as _,
            core::CoreFunctionDef,
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
    futures_locks::RwLock,
    std::collections::HashMap,
};

#[cfg(not(target_os = "macos"))]
use sysinfo_dot_h::{self};

lazy_static! {
    pub static ref SYSTEM_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref SYSTEM_FUNCTIONS: Vec<CoreFunctionDef> = vec![
        ("exit", 1, Feature::system_exit),
        ("shell", 2, Feature::system_shell),
        ("sleep", 1, Feature::system_sleep),
        #[cfg(not(target_os = "macos"))]
        ("sysinfo", 0, Feature::system_sysinfo),
        ("uname", 0u16, Feature::system_uname),
    ];
}

pub trait System {
    fn feature() -> Feature;
}

impl System for Feature {
    fn feature() -> Feature {
        Feature {
            functions: Some(&SYSTEM_FUNCTIONS),
            symbols: Some(&SYSTEM_SYMBOLS),
            namespace: "mu/system".into(),
        }
    }
}

pub trait CoreFunction {
    fn system_uname(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn system_shell(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn system_exit(_: &Env, fp: &mut Frame) -> exception::Result<()>;
    fn system_sleep(_: &Env, fp: &mut Frame) -> exception::Result<()>;
    #[cfg(not(target_os = "macos"))]
    fn system_sysinfo(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn system_shell(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("system:shell", &[Type::String, Type::List], fp)?;

        let command = fp.argv[0];
        let args = fp.argv[1];
        let mut argv = vec![];

        for cons in Cons::iter(env, args) {
            let string = Cons::car(env, cons);

            match string.type_of() {
                Type::Vector if Vector::type_of(env, string) == Type::Char => {
                    let str = Vector::as_string(env, string);
                    argv.push(str)
                }
                _ => return Err(Exception::new(env, Condition::Type, "std:command", string)),
            }
        }

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
            Ok(exit_status) => Fixnum::with_or_panic(exit_status.code().unwrap() as usize),
        };

        Ok(())
    }

    fn system_exit(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("system:exit", &[Type::Fixnum], fp)?;

        let rc = fp.argv[0];

        std::process::exit(Fixnum::as_i64(rc) as i32);
    }

    fn system_sleep(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("system:sleep", &[Type::Float], fp)?;

        fp.value = fp.argv[0];

        std::thread::sleep(std::time::Duration::from_micros(
            (1e6 * Float::as_f32(env, fp.value)) as u64,
        ));

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
                            Vector::from(info.sysname().to_str().unwrap()).evict(env),
                        ),
                        Cons::cons(
                            env,
                            Symbol::keyword("node"),
                            Vector::from(info.nodename().to_str().unwrap()).evict(env),
                        ),
                        Cons::cons(
                            env,
                            Symbol::keyword("release"),
                            Vector::from(info.release().to_str().unwrap()).evict(env),
                        ),
                        Cons::cons(
                            env,
                            Symbol::keyword("version"),
                            Vector::from(info.version().to_str().unwrap()).evict(env),
                        ),
                        Cons::cons(
                            env,
                            Symbol::keyword("machine"),
                            Vector::from(info.machine().to_str().unwrap()).evict(env),
                        ),
                    ],
                )];

                Struct::new(env, "uname", uname).evict(env)
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
                "mu/sysinfo:sysinfo",
                Tag::nil(),
            ))?,
            Ok(sysinfo) => {
                let sysinfo = vec![Cons::list(
                    env,
                    &[
                        Cons::cons(
                            env,
                            Vector::from("uptime").evict(env),
                            Fixnum::with_u64(env, sysinfo.uptime as u64)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("loads").evict(env),
                            Vector::from(vec![
                                sysinfo.loads[0] as f32,
                                sysinfo.loads[1] as f32,
                                sysinfo.loads[2] as f32,
                            ])
                            .evict(env),
                        ),
                        Cons::cons(
                            env,
                            Vector::from("totalram").evict(env),
                            Fixnum::with_u64(env, sysinfo.totalram)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("freeram").evict(env),
                            Fixnum::with_u64(env, sysinfo.freeram)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("sharedram").evict(env),
                            Fixnum::with_u64(env, sysinfo.sharedram)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("bufferram").evict(env),
                            Fixnum::with_u64(env, sysinfo.bufferram)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("totalswap").evict(env),
                            Fixnum::with_u64(env, sysinfo.totalswap)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("freeswap").evict(env),
                            Fixnum::with_u64(env, sysinfo.freeswap)?,
                        ),
                        Cons::cons(env, Vector::from("procs").evict(env), sysinfo.procs.into()),
                        Cons::cons(
                            env,
                            Vector::from("totalhigh").evict(env),
                            Fixnum::with_u64(env, sysinfo.totalhigh)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("freehigh").evict(env),
                            Fixnum::with_u64(env, sysinfo.freehigh)?,
                        ),
                        Cons::cons(
                            env,
                            Vector::from("mem_unit").evict(env),
                            sysinfo.mem_unit.into(),
                        ),
                    ],
                )];

                Vector::from(sysinfo).evict(env)
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
