//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! environment bindings
use {
    crate::{
        core::{
            cache::Cache,
            config::Config,
            core::{Core, CORE, CORE_FUNCTIONS},
            frame::Frame,
            heap::Heap,
            namespace::Namespace,
            tag::Tag,
        },
        vectors::cache::VecCacheMap,
    },
    futures_locks::RwLock,
    std::collections::HashMap,
};

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
    pub ns_map: RwLock<Vec<(Tag, String, Namespace)>>,

    pub keyword_ns: Tag,
    pub mu_ns: Tag,
    pub null_ns: Tag,

    // profiling
    #[cfg(feature = "prof")]
    pub prof: RwLock<Vec<(Tag, u64)>>,
    #[cfg(feature = "prof")]
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
            ns_map: RwLock::new(Vec::new()),
            null_ns: Tag::nil(),
            #[cfg(feature = "prof")]
            prof: RwLock::new(Vec::new()),
            #[cfg(feature = "prof")]
            prof_on: RwLock::new(false),
            vector_cache: RwLock::new(HashMap::new()),
        };

        // establish namespaces
        env.null_ns = Namespace::with(&env, "").unwrap();
        env.mu_ns =
            Namespace::with_static(&env, "mu", Some(&CORE.symbols), Some(*CORE_FUNCTIONS)).unwrap();
        env.keyword_ns = Namespace::with_static(&env, "keyword", None, None).unwrap();

        Namespace::intern_static(&env, env.mu_ns, "*null/*".into(), env.null_ns).unwrap();
        Namespace::intern_static(&env, env.mu_ns, "*standard-input*".into(), CORE.stdin()).unwrap();
        Namespace::intern_static(&env, env.mu_ns, "*standard-output*".into(), CORE.stdout())
            .unwrap();
        Namespace::intern_static(&env, env.mu_ns, "*error-output*".into(), CORE.errout()).unwrap();

        Core::symbols(&env);

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
