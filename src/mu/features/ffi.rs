//  SPDX-FileCopyrightText: Copyright 2026 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

//! core feature
#[rustfmt::skip]
use {
    crate::{
        core::{
            apply::Apply,
            core_::{Core as Core_},
            env::Env,
            exception,
            frame::Frame,
            tag::{Tag,},
            type_::{Type},
        },
        features::feature::Feature,
        types::{
            cons::Cons,
            fixnum::Fixnum,
            vector::Vector
        },
    },
};

pub trait Ffi {
    fn feature() -> Feature;
}

impl Ffi for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: None,
            functions: Some(vec![
                ("ffi-info", 0, Feature::ffi_info),
            ]),
            namespace: "feature/ffi".into(),
        }
    }
}

pub trait CoreFn {
    fn ffi_info(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Feature {
    fn ffi_info(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let version = env!("CARGO_PKG_VERSION");
        let alist = vec![
            Cons::cons(
                env,
                Vector::from("version").with_heap(env),
                Vector::from(version).with_heap(env),
            ),
            Cons::cons(
                env,
                Vector::from("features").with_heap(env),
                Core_::features_as_list(env),
            ),
            Cons::cons(
                env,
                Vector::from("streams").with_heap(env),
                Core_::nstreams(env),
            ),
        ];

        fp.value = Cons::list(env, &alist);

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
