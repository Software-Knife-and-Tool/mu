//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! sysinfo interface
use crate::{
    core::{
        core::CoreFnDef,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        types::Tag,
    },
    features::feature::Feature,
    types::{cons::Cons, fixnum::Fixnum, vector::Vector},
};

use futures_locks::RwLock;
use std::collections::HashMap;

use sysinfo_dot_h::{self};

lazy_static! {
    pub static ref SYSINFO_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref SYSINFO_FUNCTIONS: Vec<CoreFnDef> =
        vec![("sysinfo", 0, Feature::sysinfo_sysinfo),];
}

pub trait Sysinfo {
    fn feature() -> Feature;
}

impl Sysinfo for Feature {
    fn feature() -> Feature {
        Feature {
            functions: Some(&SYSINFO_FUNCTIONS),
            symbols: Some(&SYSINFO_SYMBOLS),
            namespace: "%sysinfo%".into(),
        }
    }
}

pub trait CoreFunction {
    fn sysinfo_sysinfo(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn sysinfo_sysinfo(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match sysinfo_dot_h::try_collect() {
            Err(_) => {
                return Err(Exception::new(
                    env,
                    Condition::Type,
                    "sysinfo:sysinfo",
                    Tag::nil(),
                ))
            }
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
