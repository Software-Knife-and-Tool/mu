//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! sysinfo interface
use crate::{
    core::{
        apply::CoreFunctionDef,
        exception::{self, Condition, Exception},
        frame::Frame,
        mu::Mu,
        types::Tag,
    },
    features::Feature,
    types::{
        cons::{Cons, Core as _},
        fixnum::Fixnum,
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vecimage::VecType,
        vector::Core as _,
    },
};
use sysinfo_dot_h::{self};

// mu function dispatch table
lazy_static! {
    static ref SYSINFO_SYMBOLS: Vec<CoreFunctionDef> = vec![("sysinfo", 0, Sysinfo::sysinfo),];
}

pub struct Sysinfo {}

pub trait Core {
    fn make_feature(_: &Mu) -> Feature;
}

impl Core for Sysinfo {
    fn make_feature(_: &Mu) -> Feature {
        Feature {
            symbols: SYSINFO_SYMBOLS.to_vec(),
            namespace: "sysinfo".to_string(),
        }
    }
}

pub trait MuFunction {
    fn sysinfo(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Sysinfo {
    fn sysinfo(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match sysinfo_dot_h::try_collect() {
            Err(_) => return Err(Exception::new(Condition::Type, "sysinfo", Tag::nil())),
            Ok(sysinfo) => {
                let sysinfo = vec![Cons::vlist(
                    mu,
                    &[
                        Cons::new(Symbol::keyword("uptime"), Fixnum::as_tag(sysinfo.uptime))
                            .evict(mu),
                        Cons::new(
                            Symbol::keyword("loads"),
                            vec![
                                sysinfo.loads[0] as i64,
                                sysinfo.loads[1] as i64,
                                sysinfo.loads[2] as i64,
                            ]
                            .to_vector()
                            .evict(mu),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("totlram"),
                            Fixnum::as_tag(sysinfo.totalram as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("freeram"),
                            Fixnum::as_tag(sysinfo.freeram as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("shrdram"),
                            Fixnum::as_tag(sysinfo.sharedram as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("bufram"),
                            Fixnum::as_tag(sysinfo.bufferram as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("totswap"),
                            Fixnum::as_tag(sysinfo.totalswap as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("freswap"),
                            Fixnum::as_tag(sysinfo.freeswap as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("procs"),
                            Fixnum::as_tag(sysinfo.procs as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("tothigh"),
                            Fixnum::as_tag(sysinfo.totalhigh as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("frehigh"),
                            Fixnum::as_tag(sysinfo.freehigh as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("memunit"),
                            Fixnum::as_tag(sysinfo.mem_unit as i64),
                        )
                        .evict(mu),
                    ],
                )];

                Struct::new(mu, "sysinfo", sysinfo).evict(mu)
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
