//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! nix interface
use crate::{
    core::{
        core::CoreFnDef,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        types::Tag,
    },
    features::feature::Feature,
    types::{cons::Cons, struct_::Struct, symbol::Symbol, vector::Vector},
};
use futures_locks::RwLock;
use std::collections::HashMap;

use nix::{self};

lazy_static! {
    pub static ref NIX_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref NIX_FUNCTIONS: Vec<CoreFnDef> = vec![("uname", 0u16, Feature::nix_uname),];
}

pub trait Nix {
    fn feature() -> Feature;
}

impl Nix for Feature {
    fn feature() -> Feature {
        Feature {
            functions: Some(&NIX_FUNCTIONS),
            symbols: Some(&NIX_SYMBOLS),
            namespace: "mu/nix".into(),
        }
    }
}

pub trait CoreFunction {
    fn nix_uname(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn nix_uname(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match nix::sys::utsname::uname() {
            Err(_) => {
                return Err(Exception::new(
                    env,
                    Condition::Type,
                    "nix:uname",
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
}

#[cfg(test)]
mod tests {}
