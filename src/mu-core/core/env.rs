//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env environment
//!    Env
use {
    crate::{
        allocators::bump_allocator::BumpAllocator,
        core::{
            exception::{self, Condition, Exception},
            frame::Frame,
            lib::Lib,
            types::{Tag, Type},
        },
        system::config::Config,
        types::{
            cons::{Cons, Core as _},
            namespace::Namespace,
            symbol::{Core as _, Symbol},
            vector::VecCacheMap,
        },
        LIB,
    },
    cpu_time::ProcessTime,
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
    pub core_ns: Tag,
    pub null_ns: Tag,

    // system
    pub tag: RwLock<Tag>,
    pub start_time: ProcessTime,
}

pub trait Core {
    fn new(config: Config) -> Self;
    fn apply(&self, _: Tag, _: Tag) -> exception::Result<Tag>;
    fn apply_(&self, _: Tag, _: Vec<Tag>) -> exception::Result<Tag>;
    fn eval(&self, _: Tag) -> exception::Result<Tag>;
}

impl Core for Env {
    fn new(config: Config) -> Self {
        let heap = BumpAllocator::new(config.npages, Tag::NTYPES);

        let mut env = Env {
            config,
            core_ns: Tag::nil(),
            dynamic: RwLock::new(Vec::new()),
            gc_root: RwLock::new(Vec::<Tag>::new()),
            heap: RwLock::new(heap),
            keyword_ns: Tag::nil(),
            lexical: RwLock::new(HashMap::new()),
            ns_map: RwLock::new(Vec::new()),
            null_ns: Tag::nil(),
            start_time: ProcessTime::now(),
            tag: RwLock::new(Tag::nil()),
            vector_map: RwLock::new(HashMap::new()),
        };

        // establish namespaces
        env.null_ns = match Namespace::add_ns(&env, "") {
            Ok(ns) => ns,
            Err(_) => panic!(),
        };

        env.keyword_ns = match Namespace::add_static_ns(&env, "keyword", &LIB.keywords) {
            Ok(ns) => ns,
            Err(_) => panic!(),
        };

        env.core_ns = match Namespace::add_static_ns(&env, "core", &LIB.symbols) {
            Ok(ns) => ns,
            Err(_) => panic!(),
        };

        // initialize lib namespaces
        Lib::namespaces(&env);

        env
    }

    fn apply_(&self, func: Tag, argv: Vec<Tag>) -> exception::Result<Tag> {
        let value = Tag::nil();

        Frame { func, argv, value }.apply(self, func)
    }

    fn apply(&self, func: Tag, args: Tag) -> exception::Result<Tag> {
        let value = Tag::nil();

        let eval_results: exception::Result<Vec<Tag>> = Cons::iter(self, args)
            .map(|cons| self.eval(Cons::car(self, cons)))
            .collect();

        let argv = eval_results?;

        Frame { func, argv, value }.apply(self, func)
    }

    fn eval(&self, expr: Tag) -> exception::Result<Tag> {
        match expr.type_of() {
            Type::Cons => {
                let func = Cons::car(self, expr);
                let args = Cons::cdr(self, expr);

                match func.type_of() {
                    Type::Keyword if func.eq_(&Symbol::keyword("quote")) => {
                        Ok(Cons::car(self, args))
                    }
                    Type::Symbol => {
                        if Symbol::is_bound(self, func) {
                            let fn_ = Symbol::value(self, func);
                            match fn_.type_of() {
                                Type::Function => self.apply(fn_, args),
                                _ => Err(Exception::new(self, Condition::Type, "core:eval", func)),
                            }
                        } else {
                            Err(Exception::new(self, Condition::Unbound, "core:eval", func))
                        }
                    }
                    Type::Function => self.apply(func, args),
                    _ => Err(Exception::new(self, Condition::Type, "core:eval", func)),
                }
            }
            Type::Symbol => {
                if Symbol::is_bound(self, expr) {
                    Ok(Symbol::value(self, expr))
                } else {
                    Err(Exception::new(self, Condition::Unbound, "core:eval", expr))
                }
            }
            _ => Ok(expr),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn env() {
        assert_eq!(2 + 2, 4);
    }
}