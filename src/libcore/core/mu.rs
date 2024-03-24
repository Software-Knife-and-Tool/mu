//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu environment
//!    Mu
#![allow(clippy::type_complexity)]
use {
    crate::{
        allocators::bump_allocator::BumpAllocator,
        async_::context::Context,
        core::{
            apply::{Core as _, CoreFunction},
            config::Config,
            exception::{self, Condition, Exception},
            frame::Frame,
            namespace::Namespace,
            reader::{Core as _, Reader},
            types::{Tag, Type},
        },
        features::{Core as _, Features},
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
    std::{cell::RefCell, collections::HashMap},
};

// locking protocols
use futures_locks::RwLock;

// mu environment
pub struct Mu {
    pub version: Tag,

    // configuration
    config: Config,
    features: Features,

    // heap
    pub heap: RwLock<BumpAllocator>,
    pub gc_root: RwLock<Vec<Tag>>,

    // environments
    pub dynamic: RwLock<Vec<(u64, usize)>>,
    pub lexical: RwLock<HashMap<u64, RwLock<Vec<Frame>>>>,

    // ns/async maps
    pub async_index: RwLock<HashMap<u64, Context>>,
    pub ns_index: RwLock<HashMap<u64, (Tag, RwLock<HashMap<String, Tag>>)>>,

    // native function map
    pub native_map: HashMap<u64, CoreFunction>,

    // internal functions
    pub if_: Tag,

    // namespaces
    pub keyword_ns: Tag,
    pub core_ns: Tag,
    pub null_ns: Tag,

    // reader
    pub reader: Reader,

    // streams
    pub streams: RwLock<Vec<RefCell<Stream>>>,

    pub stdin: Tag,
    pub stdout: Tag,
    pub errout: Tag,

    // system
    pub start_time: ProcessTime,
}

pub trait Core {
    const VERSION: &'static str = "0.0.39";

    fn new(config: &Config) -> Self;
    fn apply(&self, _: Tag, _: Tag) -> exception::Result<Tag>;
    fn apply_(&self, _: Tag, _: Vec<Tag>) -> exception::Result<Tag>;
    fn eval(&self, _: Tag) -> exception::Result<Tag>;
}

impl Core for Mu {
    fn new(config: &Config) -> Self {
        let mut mu = Mu {
            async_index: RwLock::new(HashMap::new()),
            config: *config,
            core_ns: Tag::nil(),
            dynamic: RwLock::new(Vec::new()),
            errout: Tag::nil(),
            features: Features::new(),
            gc_root: RwLock::new(Vec::<Tag>::new()),
            heap: RwLock::new(BumpAllocator::new(config.npages, Tag::NTYPES)),
            if_: Tag::nil(),
            keyword_ns: Tag::nil(),
            lexical: RwLock::new(HashMap::new()),
            native_map: HashMap::new(),
            ns_index: RwLock::new(HashMap::new()),
            null_ns: Tag::nil(),
            reader: Reader::new(),
            start_time: ProcessTime::now(),
            stdin: Tag::nil(),
            stdout: Tag::nil(),
            streams: RwLock::new(Vec::new()),
            version: Tag::nil(),
        };

        // establish namespaces
        mu.keyword_ns = Symbol::keyword("keyword");

        mu.core_ns = Symbol::keyword("core");
        match Namespace::add_ns(&mu, mu.core_ns) {
            Ok(_) => (),
            Err(_) => panic!(),
        };

        mu.null_ns = Tag::nil();
        match Namespace::add_ns(&mu, mu.null_ns) {
            Ok(_) => (),
            Err(_) => panic!(),
        };

        // version string
        mu.version = Vector::from_string(<Mu as Core>::VERSION).evict(&mu);
        Namespace::intern_symbol(&mu, mu.core_ns, "version".to_string(), mu.version);

        // standard streams
        mu.stdin = match StreamBuilder::new().stdin().build(&mu) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        mu.stdout = match StreamBuilder::new().stdout().build(&mu) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        mu.errout = match StreamBuilder::new().errout().build(&mu) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        // standard stream symbols
        Namespace::intern_symbol(&mu, mu.core_ns, "std-in".to_string(), mu.stdin);
        Namespace::intern_symbol(&mu, mu.core_ns, "std-out".to_string(), mu.stdout);
        Namespace::intern_symbol(&mu, mu.core_ns, "err-out".to_string(), mu.errout);

        // core functions
        mu.native_map = Self::install_core_functions(&mu);
        mu.if_ = Function::new(Tag::from(3i64), Symbol::keyword("if")).evict(&mu);

        // the reader, has to be last
        mu.reader = mu.reader.build(&mu);

        mu
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
                        if Symbol::is_unbound(self, func) {
                            Err(Exception::new(Condition::Unbound, "eval", func))
                        } else {
                            let fn_ = Symbol::value(self, func);
                            match fn_.type_of() {
                                Type::Function => self.apply(fn_, args),
                                _ => Err(Exception::new(Condition::Type, "eval", func)),
                            }
                        }
                    }
                    Type::Function => self.apply(func, args),
                    _ => Err(Exception::new(Condition::Type, "eval", func)),
                }
            }
            Type::Symbol => {
                if Symbol::is_unbound(self, expr) {
                    Err(Exception::new(Condition::Unbound, "eval", expr))
                } else {
                    Ok(Symbol::value(self, expr))
                }
            }
            _ => Ok(expr),
        }
    }
}

pub trait MuFunction {
    fn core_feature(_: &Mu, _: &mut Frame) -> exception::Result<()>;

    fn if_(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Mu {
    fn if_(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let test = fp.argv[0];
        let true_fn = fp.argv[1];
        let false_fn = fp.argv[2];

        fp.value = match mu.fp_argv_check("::if", &[Type::T, Type::Function, Type::Function], fp) {
            Ok(_) => match mu.apply(if test.null_() { false_fn } else { true_fn }, Tag::nil()) {
                Ok(tag) => tag,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_feature(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Cons::vlist(mu, &mu.features.installed);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mu() {
        assert_eq!(2 + 2, 4);
    }
}
