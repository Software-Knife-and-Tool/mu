//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features module
#[allow(unused_imports)]
use crate::{
    core::{
        lib::{Core as _, CoreFunction},
        mu::{Core as _, Mu},
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::symbol::{Core as _, Symbol},
};

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

pub struct Feature {
    symbols: Vec<(&'static str, u16, CoreFunction)>,
    namespace: String,
}

impl Feature {
    fn install(mu: &Mu, feature: Feature) -> Tag {
        let ns = Symbol::keyword(&feature.namespace);
        match Namespace::add_ns(mu, ns) {
            Ok(_) => (),
            Err(_) => panic!(),
        };

        Mu::install_feature_functions(mu, ns, feature.symbols);

        ns
    }
}

pub trait Core {
    fn install_features(_: &Mu) -> Vec<Tag>;
}

impl Core for Feature {
    fn install_features(_mu: &Mu) -> Vec<Tag> {
        #[allow(clippy::let_and_return)]
        let features = vec![
            #[cfg(feature = "nix")]
            Self::install(_mu, Nix::make_feature(_mu)),
            #[cfg(feature = "std")]
            Self::install(_mu, Std::make_feature(_mu)),
            #[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
            Self::install(_mu, Sysinfo::make_feature(_mu)),
        ];

        features
    }
}
