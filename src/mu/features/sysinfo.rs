//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! sysinfo interface
use crate::{
    core::{
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        symbols::CoreFnDef,
        types::Tag,
    },
    features::feature::Feature,
    types::{
        cons::{Cons, Core as _},
        fixnum::{Core as _, Fixnum},
        indirect_vector::Core as _,
        vector::Vector,
    },
};
use sysinfo_dot_h::{self};

// env function dispatch table
lazy_static! {
    static ref SYSINFO_SYMBOLS: Vec<CoreFnDef> =
        vec![("sysinfo", 0, <Feature as CoreFunction>::sysinfo_sysinfo),];
}

pub trait Sysinfo {
    fn feature() -> Feature;
}

impl Sysinfo for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: SYSINFO_SYMBOLS.to_vec(),
            namespace: "sysinfo".to_string(),
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
                let sysinfo = vec![Cons::vlist(
                    env,
                    &[
                        Cons::new(
                            Vector::from("uptime").evict(env),
                            Fixnum::with_u64(env, sysinfo.uptime as u64)?,
                        )
                        .evict(env),
                        Cons::new(
                            Vector::from("loads").evict(env),
                            Vector::from(vec![
                                sysinfo.loads[0] as f32,
                                sysinfo.loads[1] as f32,
                                sysinfo.loads[2] as f32,
                            ])
                            .evict(env),
                        )
                        .evict(env),
                        Cons::new(
                            Vector::from("totalram").evict(env),
                            Fixnum::with_u64(env, sysinfo.totalram)?,
                        )
                        .evict(env),
                        Cons::new(
                            Vector::from("freeram").evict(env),
                            Fixnum::with_u64(env, sysinfo.freeram)?,
                        )
                        .evict(env),
                        Cons::new(
                            Vector::from("sharedram").evict(env),
                            Fixnum::with_u64(env, sysinfo.sharedram)?,
                        )
                        .evict(env),
                        Cons::new(
                            Vector::from("bufferram").evict(env),
                            Fixnum::with_u64(env, sysinfo.bufferram)?,
                        )
                        .evict(env),
                        Cons::new(
                            Vector::from("totalswap").evict(env),
                            Fixnum::with_u64(env, sysinfo.totalswap)?,
                        )
                        .evict(env),
                        Cons::new(
                            Vector::from("freeswap").evict(env),
                            Fixnum::with_u64(env, sysinfo.freeswap)?,
                        )
                        .evict(env),
                        Cons::new(Vector::from("procs").evict(env), sysinfo.procs.into())
                            .evict(env),
                        Cons::new(
                            Vector::from("totalhigh").evict(env),
                            Fixnum::with_u64(env, sysinfo.totalhigh)?,
                        )
                        .evict(env),
                        Cons::new(
                            Vector::from("freehigh").evict(env),
                            Fixnum::with_u64(env, sysinfo.freehigh)?,
                        )
                        .evict(env),
                        Cons::new(Vector::from("mem_unit").evict(env), sysinfo.mem_unit.into())
                            .evict(env),
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
