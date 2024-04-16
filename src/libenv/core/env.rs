//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env environment
//!    Env
use {
    crate::{
        allocators::bump_allocator::BumpAllocator,
        core::{
            config::Config,
            exception::{self, Condition, Exception},
            frame::Frame,
            lib::Lib,
            namespace::{Namespace, NsRwLockIndex},
            types::{Tag, Type},
        },
        types::{
            cons::{Cons, Core as _},
            symbol::{Core as _, Symbol},
        },
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

    // environments
    pub dynamic: RwLock<Vec<(u64, usize)>>,
    pub lexical: RwLock<HashMap<u64, RwLock<Vec<Frame>>>>,

    // map
    pub ns_map: NsRwLockIndex,

    // namespaces
    pub keyword_ns: Tag,
    pub lib_ns: Tag,
    pub null_ns: Tag,

    // system
    pub tag: RwLock<Tag>,
    pub start_time: ProcessTime,
}

pub trait Core {
    fn new(config: &Config) -> Self;
    fn apply(&self, _: Tag, _: Tag) -> exception::Result<Tag>;
    fn apply_(&self, _: Tag, _: Vec<Tag>) -> exception::Result<Tag>;
    fn eval(&self, _: Tag) -> exception::Result<Tag>;
}

impl Core for Env {
    fn new(config: &Config) -> Self {
        let mut env = Env {
            config: *config,
            dynamic: RwLock::new(Vec::new()),
            gc_root: RwLock::new(Vec::<Tag>::new()),
            heap: RwLock::new(BumpAllocator::new(config.npages, Tag::NTYPES)),
            keyword_ns: Tag::nil(),
            lexical: RwLock::new(HashMap::new()),
            lib_ns: Tag::nil(),
            ns_map: RwLock::new(HashMap::new()),
            null_ns: Tag::nil(),
            start_time: ProcessTime::now(),
            tag: RwLock::new(Tag::nil()),
        };

        // establish namespaces
        env.keyword_ns = Symbol::keyword("keyword");

        env.lib_ns = Symbol::keyword("lib");
        match Namespace::add_static_ns(&env, env.lib_ns, Lib::symbols()) {
            Ok(_) => (),
            Err(_) => panic!(),
        };

        Lib::lib_namespaces(&env);

        env.null_ns = Tag::nil();
        match Namespace::add_ns(&env, env.null_ns) {
            Ok(_) => (),
            Err(_) => panic!(),
        };

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

        match eval_results {
            Ok(argv) => Frame { func, argv, value }.apply(self, func),
            Err(e) => Err(e),
        }
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
                                _ => Err(Exception::new(Condition::Type, "eval", func)),
                            }
                        } else {
                            Err(Exception::new(Condition::Unbound, "eval", func))
                        }
                    }
                    Type::Function => self.apply(func, args),
                    _ => Err(Exception::new(Condition::Type, "eval", func)),
                }
            }
            Type::Symbol => {
                if Symbol::is_bound(self, expr) {
                    Ok(Symbol::value(self, expr))
                } else {
                    Err(Exception::new(Condition::Unbound, "eval", expr))
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
