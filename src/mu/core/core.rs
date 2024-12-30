//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! lib environment
#![allow(dead_code)]
use {
    crate::{
        core::{
            env::Env,
            future::{Future, FuturePool},
            symbols::{CoreFn, LIB_SYMBOLS},
            types::Tag,
        },
        features::feature::Feature,
        streams::{stream::StreamBuilder, write::Write},
        types::{
            fixnum::Fixnum, function::Function, namespace::Namespace, stream::Stream,
            symbol::Symbol, vector::Vector,
        },
    },
    std::collections::HashMap,
};
use {futures::executor::block_on, futures_locks::RwLock};

lazy_static! {
    pub static ref CORE: Core = Core::new().features().stdio();
}

pub struct Core {
    pub version: &'static str,

    pub env_map: RwLock<HashMap<u64, Env>>,
    pub features: RwLock<Vec<Feature>>,
    pub functions: RwLock<Vec<CoreFn>>,
    pub future_id: RwLock<u64>,
    pub futures: RwLock<HashMap<u64, Future>>,
    pub keywords: RwLock<HashMap<String, Tag>>,
    pub stdio: RwLock<(Tag, Tag, Tag)>,
    pub streams: RwLock<Vec<RwLock<Stream>>>,
    pub symbols: RwLock<HashMap<String, Tag>>,
    pub threads: FuturePool,
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

impl Core {
    pub const VERSION: &'static str = "0.1.86";

    pub fn new() -> Self {
        Core {
            env_map: RwLock::new(HashMap::new()),
            features: RwLock::new(Vec::new()),
            functions: RwLock::new(Vec::new()),
            future_id: RwLock::new(0),
            futures: RwLock::new(HashMap::new()),
            keywords: RwLock::new(HashMap::new()),
            threads: FuturePool::new(),
            stdio: RwLock::new((Tag::nil(), Tag::nil(), Tag::nil())),
            streams: RwLock::new(Vec::new()),
            symbols: RwLock::new(HashMap::new()),
            version: Self::VERSION,
        }
    }

    // builders
    pub fn features(self) -> Self {
        let mut features = block_on(self.features.write());

        *features = Feature::install_features();

        self
    }

    pub fn stdio(self) -> Self {
        let mut stdio = block_on(self.stdio.write());

        let stdin = match StreamBuilder::new().stdin().std_build(&self) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        let stdout = match StreamBuilder::new().stdout().std_build(&self) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        let errout = match StreamBuilder::new().errout().std_build(&self) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        *stdio = (stdin, stdout, errout);

        self
    }

    // accessors
    pub fn stdin(&self) -> Tag {
        let stdio = block_on(self.stdio.read());

        stdio.0
    }

    pub fn stdout(&self) -> Tag {
        let stdio = block_on(self.stdio.read());

        stdio.1
    }

    pub fn errout(&self) -> Tag {
        let stdio = block_on(self.stdio.read());

        stdio.2
    }

    // core symbols
    pub fn namespaces(env: &Env) {
        let mut functions = block_on(CORE.functions.write());

        Namespace::intern_static(env, env.mu_ns, "%null-ns%".into(), env.null_ns);

        Namespace::intern_static(env, env.mu_ns, "*standard-input*".into(), CORE.stdin()).unwrap();

        Namespace::intern_static(env, env.mu_ns, "*standard-output*".into(), CORE.stdout())
            .unwrap();

        Namespace::intern_static(env, env.mu_ns, "*error-output*".into(), CORE.errout()).unwrap();

        for (name, nreqs, fn_) in &*LIB_SYMBOLS {
            let vec = vec![
                env.mu_ns,
                Vector::from(*name).evict(env),
                Fixnum::with_or_panic(functions.len()),
            ];

            let fn_vec = Vector::from(vec).evict(env);
            let func = Function::new((*nreqs).into(), fn_vec).evict(env);

            Namespace::intern_static(env, env.mu_ns, (*name).into(), func).unwrap();

            functions.push(*fn_)
        }

        let features = block_on(CORE.features.read());

        for feature in &*features {
            let ns = match Namespace::with(env, &feature.namespace) {
                Ok(ns) => ns,
                Err(_) => panic!(),
            };

            for (name, nreqs, fn_) in &*feature.symbols {
                let vec = vec![
                    ns,
                    Vector::from(*name).evict(env),
                    Fixnum::with_or_panic(functions.len()),
                ];

                let fn_vec = Vector::from(vec).evict(env);
                let func = Function::new((*nreqs).into(), fn_vec).evict(env);

                Namespace::intern(env, ns, (*name).into(), func).unwrap();

                functions.push(*fn_)
            }
        }
    }

    pub fn add_env(env: Env) -> Tag {
        let mut env_map_ref = block_on(CORE.env_map.write());
        let mut tag_ref = block_on(env.env_key.write());

        let key = Symbol::keyword(&format!("{:07x}", env_map_ref.len()));

        *tag_ref = key;
        env_map_ref.insert(key.as_u64(), env);

        key
    }
}

pub trait Debug {
    fn eprint(&self, _: &str, _: bool, _: Tag);
    fn eprintln(&self, _: &str, _: bool, _: Tag);
    fn print(&self, _: &str, _: bool, _: Tag);
    fn println(&self, _: &str, _: bool, _: Tag);
}

impl Debug for Env {
    fn eprint(&self, label: &str, verbose: bool, tag: Tag) {
        let stdio = block_on(CORE.stdio.write());

        eprint!("{}: ", label);
        self.write_stream(tag, verbose, stdio.2).unwrap();
    }

    fn eprintln(&self, label: &str, verbose: bool, tag: Tag) {
        let stdio = block_on(CORE.stdio.write());

        eprint!("{}: ", label);
        self.write_stream(tag, verbose, stdio.2).unwrap();
        eprintln!();
    }

    fn print(&self, label: &str, verbose: bool, tag: Tag) {
        let stdio = block_on(CORE.stdio.write());

        print!("{}: ", label);
        self.write_stream(tag, verbose, stdio.1).unwrap();
    }

    fn println(&self, label: &str, verbose: bool, tag: Tag) {
        let stdio = block_on(CORE.stdio.write());

        print!("{}: ", label);
        self.write_stream(tag, verbose, stdio.1).unwrap();
        println!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
