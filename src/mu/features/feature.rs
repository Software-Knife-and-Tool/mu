//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features
use {
    crate::core::{core::CoreFunctionDef, tag::Tag},
    futures_locks::RwLock,
    std::collections::HashMap,
};

#[cfg(feature = "core")]
use crate::features::core::Core;
#[cfg(feature = "env")]
use crate::features::env::Env;
#[cfg(feature = "prof")]
use crate::features::prof::Prof;
#[cfg(feature = "system")]
use crate::features::system::System;

#[derive(Clone)]
pub struct Feature {
    pub functions: Option<&'static Vec<CoreFunctionDef>>,
    pub namespace: String,
    pub symbols: Option<&'static RwLock<HashMap<String, Tag>>>,
}

impl Feature {
    pub fn install_features() -> Vec<Feature> {
        let features = vec![
            #[cfg(feature = "core")]
            <Feature as Core>::feature(),
            #[cfg(feature = "env")]
            <Feature as Env>::feature(),
            #[cfg(feature = "system")]
            <Feature as System>::feature(),
            #[cfg(feature = "prof")]
            <Feature as Prof>::feature(),
        ];

        features
    }
}
