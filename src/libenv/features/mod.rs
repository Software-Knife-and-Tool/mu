//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features module
use crate::core::lib::LibFn;

#[cfg(feature = "nix")]
use crate::features::nix::nix_::{Core as _, Nix};
#[cfg(feature = "std")]
use crate::features::std::std_::{Core as _, Std};
#[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
use crate::features::sysinfo::sysinfo_::{Core as _, Sysinfo};

#[cfg(feature = "nix")]
pub mod nix;
#[cfg(feature = "std")]
pub mod std;
#[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
pub mod sysinfo;

#[derive(Clone)]
pub struct Feature {
    pub symbols: Vec<(&'static str, u16, LibFn)>,
    pub namespace: String,
}

pub trait Core {
    fn install_features() -> Vec<Feature>;
}

impl Core for Feature {
    fn install_features() -> Vec<Feature> {
        #[allow(clippy::let_and_return)]
        let features = vec![
            #[cfg(feature = "nix")]
            Nix::feature(),
            #[cfg(feature = "std")]
            Std::feature(),
            #[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
            Sysinfo::feature(),
        ];

        features
    }
}
