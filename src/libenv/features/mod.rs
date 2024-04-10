//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features module
#[allow(unused_imports)]
use crate::{
    core::{
        env::{Core as _, Env},
        lib::{Core as _, LibFn},
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
    symbols: Vec<(&'static str, u16, LibFn)>,
    namespace: String,
}

impl Feature {
    fn install(env: &Env, feature: Feature) -> Tag {
        let ns = Symbol::keyword(&feature.namespace);
        match Namespace::add_ns(env, ns) {
            Ok(_) => (),
            Err(_) => panic!(),
        };

        Env::feature_functions(env, ns, feature.symbols);

        ns
    }
}

pub trait Core {
    fn install_features(_: &Env) -> Vec<Tag>;
}

impl Core for Feature {
    fn install_features(_env: &Env) -> Vec<Tag> {
        #[allow(clippy::let_and_return)]
        let features = vec![
            #[cfg(feature = "nix")]
            Self::install(_env, Nix::make_feature(_env)),
            #[cfg(feature = "std")]
            Self::install(_env, Std::make_feature(_env)),
            #[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
            Self::install(_env, Sysinfo::make_feature(_env)),
        ];

        features
    }
}
