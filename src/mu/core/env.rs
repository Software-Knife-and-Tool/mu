//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! environment bindings
use {
    crate::{
        core::{config::Config, frame::Frame, lib::Lib, types::Tag},
        heaps::bump_allocator::BumpAllocator,
        types::namespace::Namespace,
        vectors::cache::VecCacheMap,
        LIB,
    },
    std::collections::HashMap,
};

// locking protocols
use futures_locks::RwLock;

// env environment
pub struct Env {
    // configuration
    config: Config,

    // heap
    pub heap: RwLock<BumpAllocator>,
    pub gc_root: RwLock<Vec<Tag>>,
    pub vector_map: RwLock<VecCacheMap>,

    // environments
    pub dynamic: RwLock<Vec<(u64, usize)>>,
    pub lexical: RwLock<HashMap<u64, RwLock<Vec<Frame>>>>,

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
    pub fn new(config: Config, _image: Option<(Vec<u8>, Vec<u8>)>) -> Self {
        let heap = BumpAllocator::new(config.npages, Tag::NTYPES);

        let mut env = Env {
            config,
            dynamic: RwLock::new(Vec::new()),
            env_key: RwLock::new(Tag::nil()),
            gc_root: RwLock::new(Vec::<Tag>::new()),
            heap: RwLock::new(heap),
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

        env.keyword_ns = match Namespace::with_static(&env, "keyword", &LIB.keywords) {
            Ok(ns) => ns,
            Err(_) => panic!(),
        };

        env.mu_ns = match Namespace::with_static(&env, "mu", &LIB.symbols) {
            Ok(ns) => ns,
            Err(_) => panic!(),
        };

        // initialize lib namespaces
        Lib::namespaces(&env);

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
