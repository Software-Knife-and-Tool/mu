//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! nix interface
#![allow(unused_imports)]
use crate::{
    core::{
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        lib::LibFnDef,
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

// env function dispatch table
lazy_static! {
    static ref NIX_SYMBOLS: Vec<LibFnDef> = vec![("uname", 0, Nix::uname)];
}

pub struct Nix {}

pub trait Core {
    fn make_feature(_: &Env) -> Feature;
}

impl Core for Nix {
    fn make_feature(_: &Env) -> Feature {
        Feature {
            symbols: NIX_SYMBOLS.to_vec(),
            namespace: "nix".to_string(),
        }
    }
}

pub trait LibFunction {
    fn uname(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Nix {
    fn uname(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match nix::sys::utsname::uname() {
            Err(_) => return Err(Exception::new(Condition::Type, "uname", Tag::nil())),
            Ok(info) => {
                let uname = vec![Cons::vlist(
                    env,
                    &[
                        Cons::new(
                            Symbol::keyword("sysname"),
                            Vector::from_string(info.sysname().to_str().unwrap()).evict(env),
                        )
                        .evict(env),
                        Cons::new(
                            Symbol::keyword("node"),
                            Vector::from_string(info.nodename().to_str().unwrap()).evict(env),
                        )
                        .evict(env),
                        Cons::new(
                            Symbol::keyword("release"),
                            Vector::from_string(info.release().to_str().unwrap()).evict(env),
                        )
                        .evict(env),
                        Cons::new(
                            Symbol::keyword("version"),
                            Vector::from_string(info.version().to_str().unwrap()).evict(env),
                        )
                        .evict(env),
                        Cons::new(
                            Symbol::keyword("machine"),
                            Vector::from_string(info.machine().to_str().unwrap()).evict(env),
                        )
                        .evict(env),
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
