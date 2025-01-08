//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! environment bindings
use {
    crate::{
        core::{
            config::Config,
            core::{Core, CORE},
            frame::Frame,
            heap::HeapAllocator,
            namespace::Namespace,
            symbols::MU_FUNCTIONS,
            types::Tag,
        },
        vectors::cache::VecCacheMap,
    },
    futures_locks::RwLock,
    std::collections::HashMap,
};

pub struct Env {
    // configuration
    pub config: Config,

    // heap
    pub heap: RwLock<HeapAllocator>,
    pub vector_map: RwLock<VecCacheMap>,

    // environments
    pub lexical: RwLock<HashMap<u64, RwLock<Vec<Frame>>>>,

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

    // env map
    pub env_key: RwLock<Tag>,
}

impl Env {
    pub fn new(config: &Config, _image: Option<(Vec<u8>, Vec<u8>)>) -> Self {
        let mut env = Env {
            config: config.clone(),
            dynamic: RwLock::new(Vec::new()),
            env_key: RwLock::new(Tag::nil()),
            heap: RwLock::new(HeapAllocator::new(config)),
            keyword_ns: Tag::nil(),
            lexical: RwLock::new(HashMap::new()),
            mu_ns: Tag::nil(),
            ns_map: RwLock::new(Vec::new()),
            null_ns: Tag::nil(),
            #[cfg(feature = "prof")]
            prof: RwLock::new(Vec::new()),
            #[cfg(feature = "prof")]
            prof_on: RwLock::new(false),
            vector_map: RwLock::new(HashMap::new()),
        };

        // establish namespaces
        env.null_ns = match Namespace::with(&env, "") {
            Ok(ns) => ns,
            Err(_) => panic!(),
        };

        env.mu_ns =
            match Namespace::with_static(&env, "mu", Some(&CORE.symbols), Some(&MU_FUNCTIONS)) {
                Ok(ns) => ns,
                Err(_) => panic!(),
            };

        env.keyword_ns = match Namespace::with_static(&env, "keyword", None, None) {
            Ok(ns) => ns,
            Err(_) => panic!(),
        };

        // initialize core/feature namespaces
        Core::namespaces(&env);

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
