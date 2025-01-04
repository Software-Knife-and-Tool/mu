//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features
use crate::core::{core::CoreFnDef, types::Tag};

use futures_locks::RwLock;
use std::collections::HashMap;

#[cfg(feature = "cpu_time")]
use crate::features::cpu_time::CpuTime;
#[cfg(feature = "env")]
use crate::features::env::Env;
#[cfg(feature = "ffi")]
use crate::features::ffi::Ffi;
#[cfg(feature = "nix")]
use crate::features::nix::Nix;
#[cfg(feature = "prof")]
use crate::features::prof::Prof;
#[cfg(feature = "semispace")]
use crate::features::semispace_heap::SemiSpace;
#[cfg(feature = "std")]
use crate::features::std::Std;
#[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
use crate::features::sysinfo::Sysinfo;

#[derive(Clone)]
pub struct Feature {
    pub functions: Option<&'static Vec<CoreFnDef>>,
    pub namespace: String,
    pub symbols: Option<&'static RwLock<HashMap<String, Tag>>>,
}

impl Feature {
    pub fn install_features() -> Vec<Feature> {
        let features = vec![
            #[cfg(feature = "cpu_time")]
            <Feature as CpuTime>::feature(),
            #[cfg(feature = "env")]
            <Feature as Env>::feature(),
            #[cfg(feature = "nix")]
            <Feature as Nix>::feature(),
            #[cfg(feature = "std")]
            <Feature as Std>::feature(),
            #[cfg(feature = "ffi")]
            <Feature as Ffi>::feature(),
            #[cfg(feature = "prof")]
            <Feature as Prof>::feature(),
            #[cfg(feature = "semispace")]
            <Feature as SemiSpace>::feature(),
            #[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
            <Feature as Sysinfo>::feature(),
        ];

        features
    }
}
