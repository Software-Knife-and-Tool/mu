//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features module
#[allow(unused_imports)]
use crate::core::{
    mu::{Core as _, Mu},
    namespace::Namespace,
    types::{Tag, Type},
};

#[cfg(feature = "std")]
pub mod std;

pub struct Features {
    installed: Vec<(String, Namespace)>,
}

pub trait Core {
    fn new() -> Self;
    fn install(_: &Mu);
}

impl Core for Features {
    fn new() -> Self {
        let features = Features { installed: vec![] };

        features
    }

    fn install(_: &Mu) {}
}
