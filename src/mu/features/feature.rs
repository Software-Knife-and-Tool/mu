//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features
use crate::core::symbols::CoreFn;

#[cfg(feature = "cpu_time")]
use crate::features::cpu_time::CpuTime;
#[cfg(feature = "ffi")]
use crate::features::ffi::Ffi;
#[cfg(feature = "nix")]
use crate::features::nix::Nix;
#[cfg(feature = "prof")]
use crate::features::prof::Prof;
#[cfg(feature = "semispace_heap")]
use crate::features::semispace_heap::SemiSpace;
#[cfg(feature = "std")]
use crate::features::std::Std;
#[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
use crate::features::sysinfo::Sysinfo;

#[derive(Clone)]
pub struct Feature {
    pub symbols: Vec<(&'static str, u16, CoreFn)>,
    pub namespace: String,
}

impl Feature {
    pub fn install_features() -> Vec<Feature> {
        let features = vec![
            #[cfg(feature = "cpu_time")]
            <Feature as CpuTime>::feature(),
            #[cfg(feature = "nix")]
            <Feature as Nix>::feature(),
            #[cfg(feature = "std")]
            <Feature as Std>::feature(),
            #[cfg(feature = "ffi")]
            <Feature as Ffi>::feature(),
            #[cfg(feature = "prof")]
            <Feature as Prof>::feature(),
            #[cfg(feature = "semispace_heap")]
            <Feature as SemiSpace>::feature(),
            #[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
            <Feature as Sysinfo>::feature(),
        ];

        features
    }
}
