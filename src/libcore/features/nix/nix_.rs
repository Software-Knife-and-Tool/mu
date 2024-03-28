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
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vector::{Core as _, Vector},
    },
};
// use nix::{self};

// mu function dispatch table
lazy_static! {
    static ref NIX_SYMBOLS: Vec<CoreFunctionDef> = vec![];
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

pub trait MuFunction {}

impl MuFunction for Nix {}

#[cfg(test)]
mod tests {}
