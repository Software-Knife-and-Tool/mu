//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features module
#[allow(unused_imports)]
use crate::{
    core::{
        apply::{Core as _, CoreFunction},
        mu::{Core as _, Mu},
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::symbol::{Core as _, Symbol},
};

#[cfg(feature = "std")]
use crate::features::std::std_::{Core as _, Std};
#[cfg(feature = "sysinfo")]
use crate::features::sysinfo::sysinfo_::{Core as _, Sysinfo};
#[cfg(feature = "uname")]
use crate::features::uname::uname_::{Core as _, Uname};

#[cfg(feature = "std")]
pub mod std;
#[cfg(feature = "sysinfo")]
pub mod sysinfo;
#[cfg(feature = "uname")]
pub mod uname;

pub struct Feature {
    symbols: Vec<(&'static str, u16, CoreFunction)>,
    namespace: String,
}

impl Feature {
    fn install_feature(mu: &Mu, feature: Feature) -> Tag {
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
    fn add_features(_: &Mu) -> Vec<Tag>;
}

impl Core for Feature {
    fn add_features(_mu: &Mu) -> Vec<Tag> {
        #[allow(clippy::let_and_return)]
        let features = vec![
            #[cfg(feature = "std")]
            Self::install_feature(_mu, Std::make_feature(_mu)),
            #[cfg(feature = "sysinfo")]
            Self::install_feature(_mu, Sysinfo::make_feature(_mu)),
            #[cfg(feature = "uname")]
            Self::install_feature(_mu, Uname::make_feature(_mu)),
        ];

        features
    }
}
