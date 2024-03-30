//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! nix interface
#![allow(unused_imports)]
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
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vector::{Core as _, Vector},
    },
};
use nix::{self};

// mu function dispatch table
lazy_static! {
    static ref NIX_SYMBOLS: Vec<CoreFunctionDef> = vec![("uname", 0, Nix::uname)];
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
    fn uname(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Nix {
    fn uname(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match nix::sys::utsname::uname() {
            Err(_) => return Err(Exception::new(Condition::Type, "uname", Tag::nil())),
            Ok(info) => {
                let uname = vec![Cons::vlist(
                    mu,
                    &[
                        Cons::new(
                            Symbol::keyword("sysname"),
                            Vector::from_string(info.sysname().to_str().unwrap()).evict(mu),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("node"),
                            Vector::from_string(info.nodename().to_str().unwrap()).evict(mu),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("release"),
                            Vector::from_string(info.release().to_str().unwrap()).evict(mu),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("version"),
                            Vector::from_string(info.version().to_str().unwrap()).evict(mu),
                        )
                        .evict(mu),
                        Cons::new(
                            Symbol::keyword("machine"),
                            Vector::from_string(info.machine().to_str().unwrap()).evict(mu),
                        )
                        .evict(mu),
                    ],
                )];

                Struct::new(mu, "uname", uname).evict(mu)
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}