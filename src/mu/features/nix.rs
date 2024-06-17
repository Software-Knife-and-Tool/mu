//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! nix interface
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
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vector::Vector,
        vector_image::Core as _,
    },
};
use nix::{self};

// env function dispatch table
lazy_static! {
    static ref NIX_SYMBOLS: Vec<CoreFnDef> =
        vec![("uname", 0, <Feature as CoreFunction>::nix_uname)];
}

pub trait Nix {
    fn feature() -> Feature;
}

impl Nix for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: NIX_SYMBOLS.to_vec(),
            namespace: "nix".to_string(),
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
                let uname = vec![Cons::vlist(
                    env,
                    &[
                        Cons::new(
                            Symbol::keyword("sysname"),
                            Vector::from(info.sysname().to_str().unwrap()).evict(env),
                        )
                        .evict(env),
                        Cons::new(
                            Symbol::keyword("node"),
                            Vector::from(info.nodename().to_str().unwrap()).evict(env),
                        )
                        .evict(env),
                        Cons::new(
                            Symbol::keyword("release"),
                            Vector::from(info.release().to_str().unwrap()).evict(env),
                        )
                        .evict(env),
                        Cons::new(
                            Symbol::keyword("version"),
                            Vector::from(info.version().to_str().unwrap()).evict(env),
                        )
                        .evict(env),
                        Cons::new(
                            Symbol::keyword("machine"),
                            Vector::from(info.machine().to_str().unwrap()).evict(env),
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
