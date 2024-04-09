//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env environment
//!    Env
#![allow(clippy::type_complexity)]
use {
    crate::{
        allocators::bump_allocator::BumpAllocator,
        async_::context::Context,
        core::{
            config::Config,
            exception::{self, Condition, Exception},
            frame::Frame,
            lib::{Core as _, LibFn, LIB},
            namespace::Namespace,
            types::{Tag, Type},
        },
        features::{Core as _, Feature},
        types::{
            cons::{Cons, Core as _},
            function::Function,
            stream::Stream,
            streambuilder::StreamBuilder,
            symbol::{Core as _, Symbol},
            vector::{Core as _, Vector},
        },
    },
    cpu_time::ProcessTime,
    std::collections::HashMap,
};

// locking protocols
use futures_locks::RwLock;

// env environment
pub struct Env {
    version: Tag,

    // configuration
    config: Config,

    // heap
    pub heap: RwLock<BumpAllocator>,
    pub gc_root: RwLock<Vec<Tag>>,

    // environments
    pub dynamic: RwLock<Vec<(u64, usize)>>,
    pub lexical: RwLock<HashMap<u64, RwLock<Vec<Frame>>>>,

    // ns/async maps
    pub async_index: RwLock<HashMap<u64, Context>>,
    pub ns_index: RwLock<HashMap<u64, (Tag, RwLock<HashMap<String, Tag>>)>>,

    // core function map
    pub functions: RwLock<HashMap<u64, LibFn>>,

    // internal functions
    pub if_: Tag,

    // namespaces
    features: Vec<Tag>,

    pub keyword_ns: Tag,
    pub lib_ns: Tag,
    pub null_ns: Tag,

    // streams
    pub streams: RwLock<Vec<RwLock<Stream>>>,

    pub stdin: Tag,
    pub stdout: Tag,
    pub errout: Tag,

    // system
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
            async_index: RwLock::new(HashMap::new()),
            config: *config,
            dynamic: RwLock::new(Vec::new()),
            errout: Tag::nil(),
            features: Vec::new(),
            functions: RwLock::new(HashMap::new()),
            gc_root: RwLock::new(Vec::<Tag>::new()),
            heap: RwLock::new(BumpAllocator::new(config.npages, Tag::NTYPES)),
            if_: Tag::nil(),
            keyword_ns: Tag::nil(),
            lexical: RwLock::new(HashMap::new()),
            lib_ns: Tag::nil(),
            ns_index: RwLock::new(HashMap::new()),
            null_ns: Tag::nil(),
            start_time: ProcessTime::now(),
            stdin: Tag::nil(),
            stdout: Tag::nil(),
            streams: RwLock::new(Vec::new()),
            version: Tag::nil(),
        };

        // establish namespaces
        env.keyword_ns = Symbol::keyword("keyword");

        env.lib_ns = Symbol::keyword("lib");
        match Namespace::add_ns(&env, env.lib_ns) {
            Ok(_) => (),
            Err(_) => panic!(),
        };

        env.null_ns = Tag::nil();
        match Namespace::add_ns(&env, env.null_ns) {
            Ok(_) => (),
            Err(_) => panic!(),
        };

        // version string
        env.version = Vector::from_string(LIB.version).evict(&env);
        Namespace::intern_symbol(&env, env.lib_ns, "version".to_string(), env.version);

        // standard streams
        env.stdin = match StreamBuilder::new().stdin().build(&env) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        env.stdout = match StreamBuilder::new().stdout().build(&env) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        env.errout = match StreamBuilder::new().errout().build(&env) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        // standard stream symbols
        Namespace::intern_symbol(&env, env.lib_ns, "std-in".to_string(), env.stdin);
        Namespace::intern_symbol(&env, env.lib_ns, "std-out".to_string(), env.stdout);
        Namespace::intern_symbol(&env, env.lib_ns, "err-out".to_string(), env.errout);

        // core functions
        env.functions = RwLock::new(Self::install_lib_functions(&env));
        env.if_ = Function::new(Tag::from(3i64), Symbol::keyword("if")).evict(&env);

        env.features = Feature::install_features(&env);

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
