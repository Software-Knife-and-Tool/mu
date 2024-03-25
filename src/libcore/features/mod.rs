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
    features::std::std_::{Core as _, Std},
    types::symbol::{Core as _, Symbol},
};

#[cfg(feature = "libc")]
pub mod libc;
#[cfg(feature = "std")]
pub mod std;

pub struct Feature {
    symbols: Vec<(&'static str, u16, CoreFunction)>,
    namespace: String,
}

pub trait Core {
    fn install_feature(_: &Mu, _: Feature) -> Tag;
    fn add_features(_: &Mu) -> Vec<Tag>;
}

impl Core for Feature {
    fn add_features(mu: &Mu) -> Vec<Tag> {
        #[allow(clippy::let_and_return)]
        let features = vec![
            #[cfg(feature = "std")]
            Self::install_feature(mu, Std::make_feature(mu)),
            #[cfg(feature = "libc")]
            Self::install_feature(mu, Libc::make_feature(mu)),
        ];

        features
    }

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
