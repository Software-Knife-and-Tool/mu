//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! environment bindings
#[rustfmt::skip]
use {
    crate::{
        core::{
            config::Config,
            core_::{CORE, CORE_FUNCTIONS},
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

        // establish runtime namespaces
        env.keyword_ns =
            Namespace::with_static(&env, "keyword", StaticSymbols(None, None)).unwrap();

        env.mu_ns = Namespace::with_mu_static(
            &env,
            StaticSymbols(
                Some(vec![
                    ("*standard-input*".to_string(), CORE.stdio.0),
                    ("*standard-output*".to_string(), CORE.stdio.1),
                    ("*error-output*".to_string(), CORE.stdio.2),
                ]),
                Some(CORE_FUNCTIONS.to_vec()),
            ),
        );

        // install feature namespaces
        for feature in &FEATURES.features {
            if feature.namespace.is_empty() {
                continue;
            }

            Namespace::with_static(
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
