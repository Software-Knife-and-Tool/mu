//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! lib environment
use {
    crate::{
        core::{
            env::Env,
            future::{Future, FuturePool},
            symbols::{CoreFn, LIB_SYMBOLS},
            types::Tag,
        },
        features::feature::{Core as _, Feature},
        streams::write::Core as _,
        types::{
            core_stream::Stream,
            fixnum::{Core as _, Fixnum},
            function::Function,
            indirect_vector::Core as _,
            namespace::Namespace,
            stream::StreamBuilder,
            symbol::{Core as _, Symbol},
            vector::Vector,
        },
    },
    std::collections::HashMap,
};
use {futures::executor::block_on, futures_locks::RwLock};

lazy_static! {
    pub static ref LIB: Lib = Lib::new().features().stdio();
}

pub struct Lib {
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

impl Lib {
    pub const VERSION: &'static str = "0.1.66";

    pub fn new() -> Self {
        Lib {
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

    // lib symbols
    pub fn namespaces(env: &Env) {
        let mut functions = block_on(LIB.functions.write());

        Namespace::intern_static(
            env,
            env.mu_ns,
            "*version*".to_string(),
            Vector::from(LIB.version).evict(env),
        )
        .unwrap();

        Namespace::intern_static(env, env.mu_ns, "*standard-input*".to_string(), LIB.stdin())
            .unwrap();
        Namespace::intern_static(
            env,
            env.mu_ns,
            "*standard-output*".to_string(),
            LIB.stdout(),
        )
        .unwrap();

        Namespace::intern_static(env, env.mu_ns, "*error-output*".to_string(), LIB.errout())
            .unwrap();

        for (name, nreqs, fn_) in &*LIB_SYMBOLS {
            let vec = vec![
                env.mu_ns,
                Vector::from(*name).evict(env),
                Fixnum::with_or_panic(functions.len()),
            ];

            let fn_vec = Vector::from(vec).evict(env);
            let func = Function::new((*nreqs).into(), fn_vec).evict(env);

            Namespace::intern_static(env, env.mu_ns, name.to_string(), func).unwrap();

            functions.push(*fn_)
        }

        let features = block_on(LIB.features.read());

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

                Namespace::intern(env, ns, name.to_string(), func).unwrap();

                functions.push(*fn_)
            }
        }
    }
}

pub trait Core {
    fn add_env(_: Env) -> Tag;
}

impl Core for Env {
    fn add_env(env: Env) -> Tag {
        let mut env_map_ref = block_on(LIB.env_map.write());
        let mut tag_ref = block_on(env.tag.write());

        let key = Symbol::keyword(&format!("{:07x}", env_map_ref.len()));

        *tag_ref = key;
        env_map_ref.insert(key.as_u64(), env);

        key
    }
}

pub trait Debug {
    fn debug_vprintln(&self, _: &str, _: bool, _: Tag);
    fn debug_vprint(&self, _: &str, _: bool, _: Tag);
}

impl Debug for Env {
    // debug printing
    fn debug_vprint(&self, label: &str, verbose: bool, tag: Tag) {
        let stdio = block_on(LIB.stdio.read());

        print!("{}: ", label);
        self.write_stream(tag, verbose, stdio.1).unwrap();
    }

    fn debug_vprintln(&self, label: &str, verbose: bool, tag: Tag) {
        self.debug_vprint(label, verbose, tag);
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
