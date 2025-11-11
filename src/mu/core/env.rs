//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! environment bindings
#[rustfmt::skip]
use {
    crate::{
        core::{
            config::Config,
            core_::{CORE, CORE_FUNCTIONS},
            direct::DirectTag,
            frame::Frame,
            tag::Tag,
        },
        namespaces::{
            cache::Cache,
            heap::Heap,
            namespace::{Namespace, StaticSymbols},
        },
        features::feature::FEATURES,
        vectors::cache::VecCacheMap,
    },
    futures_locks::RwLock,
    std::collections::HashMap,
};

#[cfg(feature = "instrument")]
#[allow(unused_imports)]
use crate::core::instrument::Instrument;

pub struct Env {
    // configuration
    pub config: Config,

    // heaps
    pub heap: RwLock<Heap>,
    pub vector_cache: RwLock<VecCacheMap>,
    pub lexical: RwLock<HashMap<u64, Vec<Frame>>>,
    pub cache: RwLock<Cache>,

    // dynamic state
    pub dynamic: RwLock<Vec<(u64, usize)>>,

    // namespaces
    pub ns_map: RwLock<HashMap<String, (Tag, Namespace)>>,

    pub keyword_ns: Tag,
    pub mu_ns: Tag,

    // profiling
    #[cfg(feature = "instrument")]
    pub prof: RwLock<Vec<(Tag, u64)>>,
    #[cfg(feature = "instrument")]
    pub prof_on: RwLock<bool>,
}

impl Env {
    pub fn new(config: &Config) -> Self {
        let mut env = Env {
            cache: RwLock::new(Cache::new()),
            config: config.clone(),
            dynamic: RwLock::new(Vec::new()),
            heap: RwLock::new(Heap::new(config)),
            keyword_ns: Tag::nil(),
            lexical: RwLock::new(HashMap::new()),
            mu_ns: Tag::nil(),
            ns_map: RwLock::new(HashMap::new()),
            vector_cache: RwLock::new(HashMap::new()),
            #[cfg(feature = "instrument")]
            prof: RwLock::new(Vec::new()),
            #[cfg(feature = "instrument")]
            prof_on: RwLock::new(false),
        };

        // establish namespaces
        env.mu_ns = Namespace::with_static(&env, "mu", Some(RwLock::new(HashMap::new()))).unwrap();
        env.keyword_ns =
            Namespace::with_static_defs(&env, "keyword", StaticSymbols(None, None)).unwrap();

        // standard streams
        Namespace::intern_static(&env, env.mu_ns, "*standard-input*".into(), CORE.stdio.0);
        Namespace::intern_static(&env, env.mu_ns, "*standard-output*".into(), CORE.stdio.1);
        Namespace::intern_static(&env, env.mu_ns, "*error-output*".into(), CORE.stdio.2);

        // mu functions
        for (index, desc) in CORE_FUNCTIONS.iter().enumerate() {
            Namespace::intern_static(
                &env,
                env.mu_ns,
                (*desc.0).into(),
                DirectTag::function(index),
            )
        }

        // features
        for feature in &FEATURES.features {
            if feature.namespace.is_empty() {
                continue;
            }

            Namespace::with_static_defs(
                &env,
                &feature.namespace,
                StaticSymbols(feature.symbols.clone(), feature.functions.clone()),
            )
            .unwrap();
        }

        /*
        #[cfg(feature = "instrument")]
        Instrument::eprintln(&env, "env: new, mu ns", true, env.mu_ns);
         */

        env
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn env() {
        assert_eq!(2 + 2, 4);
    }
}
