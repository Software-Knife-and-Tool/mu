//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features module
#[allow(unused_imports)]
use crate::{
    core::{
        mu::{Core as _, Mu},
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::symbol::{Core as _, Symbol},
};

#[cfg(feature = "std")]
pub mod std;

pub struct Features {
    pub installed: Vec<Tag>,
}

pub trait Core {
    fn new() -> Self;
    fn install(_: &Mu);
}

impl Core for Features {
    fn new() -> Self {
        #[allow(clippy::let_and_return)]
        let features = Features {
            installed: vec![
                #[cfg(feature = "std")]
                Symbol::keyword("std"),
            ],
        };

        features
    }

    fn install(_: &Mu) {}
}
