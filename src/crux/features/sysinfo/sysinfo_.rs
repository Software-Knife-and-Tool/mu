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
        indirect_vector::VecType,
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vector::Core as _,
    },
};
use sysinfo_dot_h::{self};

// env function dispatch table
lazy_static! {
    static ref SYSINFO_SYMBOLS: Vec<CoreFnDef> = vec![("sysinfo", 0, Sysinfo::sysinfo),];
}

pub struct Sysinfo {}

pub trait Core {
    fn feature() -> Feature;
}

impl Core for Sysinfo {
    fn feature() -> Feature {
        Feature {
            symbols: SYSINFO_SYMBOLS.to_vec(),
            namespace: "sysinfo".to_string(),
        }
    }
}

pub trait CoreFunction {
    fn sysinfo(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Sysinfo {
    fn sysinfo(env: &Env, fp: &mut Frame) -> exception::Result<()> {
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
                        Cons::new(Symbol::keyword("uptime"), sysinfo.uptime.into()).evict(env),
                        Cons::new(
                            Symbol::keyword("loads"),
                            vec![
                                sysinfo.loads[0] as f32,
                                sysinfo.loads[1] as f32,
                                sysinfo.loads[2] as f32,
                            ]
                            .to_vector()
                            .evict(env),
                        )
                        .evict(env),
                        Cons::new(Symbol::keyword("totlram"), sysinfo.totalram.into()).evict(env),
                        Cons::new(Symbol::keyword("freeram"), sysinfo.freeram.into()).evict(env),
                        Cons::new(Symbol::keyword("shrdram"), sysinfo.sharedram.into()).evict(env),
                        Cons::new(Symbol::keyword("bufram"), sysinfo.bufferram.into()).evict(env),
                        Cons::new(Symbol::keyword("totswap"), sysinfo.totalswap.into()).evict(env),
                        Cons::new(Symbol::keyword("freswap"), sysinfo.freeswap.into()).evict(env),
                        Cons::new(Symbol::keyword("procs"), sysinfo.procs.into()).evict(env),
                        Cons::new(Symbol::keyword("tothigh"), sysinfo.totalhigh.into()).evict(env),
                        Cons::new(Symbol::keyword("frehigh"), sysinfo.freehigh.into()).evict(env),
                        Cons::new(Symbol::keyword("meenvnit"), sysinfo.mem_unit.into()).evict(env),
                    ],
                )];

                Struct::new(env, "sysinfo", sysinfo).evict(env)
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
