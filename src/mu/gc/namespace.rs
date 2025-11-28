//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// namespaces
use {
    crate::{
        core::env::Env,
        gc::{gc_::GcContext, symbol::Gc as _},
        namespaces::namespace::Namespace,
        types::symbol::Symbol,
    },
    futures_lite::future::block_on,
};

pub trait Gc {
    #[allow(dead_code)]
    fn gc(&mut self, _: &mut GcContext, _: &Env);
}

impl Gc for Namespace {
    #[allow(dead_code)]
    fn gc(&mut self, gc: &mut GcContext, env: &Env) {
        match self {
            Namespace::Static(static_) => {
                if let Some(hash) = &static_ {
                    for symbol in hash.values() {
                        Symbol::mark(gc, env, *symbol);
                    }
                }
            }
            Namespace::Dynamic(ref hash) => {
                let hash_ref = block_on(hash.read());
                for symbol in hash_ref.values() {
                    Symbol::mark(gc, env, *symbol);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn namespace_test() {
        assert!(true)
    }
}
