//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// features
#[rustfmt::skip]
use {
    crate::core::{
        core_::CoreFnDef,
        tag::Tag
    },
    futures_locks::RwLock,
    std::collections::HashMap,
};

#[cfg(feature = "core")]
use crate::features::core::Core;
#[cfg(feature = "env")]
use crate::features::env::Env;
#[cfg(feature = "instrument")]
use crate::features::instrument::Instrument;
#[cfg(feature = "system")]
use crate::features::system::System;

lazy_static! {
    pub static ref FEATURES: Features = Features::new();
}

#[derive(Clone)]
pub struct Feature {
    pub functions: Option<&'static [CoreFnDef]>,
    pub namespace: String,
    pub symbols: Option<&'static RwLock<HashMap<String, Tag>>>,
}

pub struct Features {
    pub features: Vec<Feature>,
}

impl Features {
    fn new() -> Self {
        let features = vec![
            #[cfg(feature = "core")]
            <Feature as Core>::feature(),
            #[cfg(feature = "env")]
            <Feature as Env>::feature(),
            #[cfg(feature = "system")]
            <Feature as System>::feature(),
            #[cfg(feature = "instrument")]
            <Feature as Instrument>::feature(),
        ];

        Self { features }
    }
}
