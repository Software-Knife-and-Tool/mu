//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! nix interface
#[allow(unused_imports)]
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
        fixnum::{Core as _, Fixnum},
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vecimage::VecType,
        vector::{Core as _, Vector},
    },
};
// use nix::{self};

// mu function dispatch table
lazy_static! {
    static ref NIX_SYMBOLS: Vec<CoreFunctionDef> = vec![("sysinfo", 0, Nix::nix_sysinfo),];
}

pub struct Nix {}

pub trait Core {
    fn make_feature(_: &Mu) -> Feature;
}

impl Core for Nix {
    fn make_feature(_: &Mu) -> Feature {
        Feature {
            symbols: NIX_SYMBOLS.to_vec(),
            namespace: "nix".to_string(),
        }
    }
}

pub trait MuFunction {
    fn nix_sysinfo(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Nix {
    fn nix_sysinfo(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match nix::sys::sysinfo::sysinfo() {
            Err(_) => return Err(Exception::new(Condition::Type, "sysinfo", Tag::nil())),
            Ok(sysinfo) => {
                let sysinfo = vec![Cons::vlist(
                    mu,
                    &[
                        Cons::new(
                            Symbol::keyword("uptime"),
                            Fixnum::as_tag(sysinfo.uptime().as_secs() as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("loads"),
                            vec![
                                sysinfo.load_average().0 as f32,
                                sysinfo.load_average().1 as f32,
                                sysinfo.load_average().2 as f32,
                            ]
                            .to_vector()
                            .evict(mu),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("totlram"),
                            Fixnum::as_tag(sysinfo.ram_total() as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("freeram"),
                            Fixnum::as_tag(sysinfo.ram_unused() as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("totswap"),
                            Fixnum::as_tag(sysinfo.swap_total() as i64),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("freswap"),
                            Fixnum::as_tag(sysinfo.swap_free() as i64),
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
