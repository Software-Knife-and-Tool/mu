//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env system functions
use {
    crate::{
        core::{
            compile::{Compile, LibFunction as _},
            direct::{DirectInfo, DirectTag, DirectType},
            env::Env,
            functions::{LibFn, LIB_SYMBOLS},
            futures::FuturePool,
            namespace::{Namespace, NsRwLockMap},
            types::Tag,
        },
        features::{Core as _, Feature},
        streams::write::Core as _,
        types::{
            function::Function,
            stream::Stream,
            streambuilder::StreamBuilder,
            symbol::{Core as _, Symbol, UNBOUND},
            vector::{Core as _, Vector},
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

    pub eol: Tag,
    pub features: RwLock<Vec<Feature>>,
    pub functions: RwLock<HashMap<u64, LibFn>>,
    pub future_id: RwLock<u64>,
    pub futures: RwLock<HashMap<u64, std::thread::JoinHandle<Tag>>>,
    pub env_map: RwLock<HashMap<u64, Env>>,
    pub stdio: RwLock<(Tag, Tag, Tag)>,
    pub streams: RwLock<Vec<RwLock<Stream>>>,
    pub symbols: NsRwLockMap,
    pub threads: FuturePool,
}

impl Lib {
    pub const VERSION: &'static str = "0.1.47";

    pub fn new() -> Self {
        let lib = Lib {
            eol: DirectTag::to_direct(0, DirectInfo::Length(0), DirectType::Keyword),
            env_map: RwLock::new(HashMap::new()),
            features: RwLock::new(Vec::new()),
            functions: RwLock::new(HashMap::new()),
            future_id: RwLock::new(0),
            futures: RwLock::new(HashMap::new()),
            threads: FuturePool::new(),
            stdio: RwLock::new((Tag::nil(), Tag::nil(), Tag::nil())),
            streams: RwLock::new(Vec::new()),
            symbols: RwLock::new(HashMap::new()),
            version: Self::VERSION,
        };
        let mut functions = block_on(lib.functions.write());

        // native functions
        functions.insert(Tag::as_u64(&Symbol::keyword("if")), Compile::if__);
        functions.extend(
            LIB_SYMBOLS
                .iter()
                .map(|(name, _, libfn)| (Tag::as_u64(&Symbol::keyword(name)), *libfn)),
        );

        lib
    }

    pub fn features(self) -> Self {
        let mut features = block_on(self.features.write());

        *features = Feature::install_features();

        self
    }

    pub fn stdio(self) -> Self {
        let mut stdio = block_on(self.stdio.write());

        let stdin = match StreamBuilder::new().stdin().build(&self) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        let stdout = match StreamBuilder::new().stdout().build(&self) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        let errout = match StreamBuilder::new().errout().build(&self) {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        *stdio = (stdin, stdout, errout);

        self
    }

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

    pub fn symbols() -> &'static RwLock<HashMap<String, Tag>> {
        &LIB.symbols
    }

    // lib symbols
    pub fn lib_namespaces(env: &Env) {
        let mut functions = block_on(LIB.functions.write());

        Namespace::intern_symbol(
            env,
            env.lib_ns,
            "version".to_string(),
            Vector::from_string(LIB.version).evict(env),
        );

        Namespace::intern_symbol(
            env,
            env.lib_ns,
            "if".to_string(),
            Function::new(Tag::from(3i64), Symbol::keyword("if")).evict(env),
        );

        Namespace::intern_symbol(env, env.lib_ns, "std-in".to_string(), LIB.stdin());

        Namespace::intern_symbol(env, env.lib_ns, "std-out".to_string(), LIB.stdout());

        Namespace::intern_symbol(env, env.lib_ns, "err-out".to_string(), LIB.errout());

        functions.insert(Tag::as_u64(&Symbol::keyword("if")), Compile::if__);

        functions.extend(LIB_SYMBOLS.iter().map(|(name, nreqs, libfn)| {
            let fn_key = Symbol::keyword(name);
            let func = Function::new(Tag::from(*nreqs as i64), fn_key).evict(env);

            Namespace::intern_symbol(env, env.lib_ns, name.to_string(), func);

            (Tag::as_u64(&fn_key), *libfn)
        }));

        let features = block_on(LIB.features.read());

        for feature in &*features {
            let ns = Symbol::keyword(&feature.namespace);
            match Namespace::add_ns(env, ns) {
                Ok(_) => (),
                Err(_) => panic!(),
            };

            functions.extend(feature.symbols.iter().map(|(name, nreqs, featurefn)| {
                let form = Namespace::intern_symbol(env, ns, name.to_string(), *UNBOUND);
                let func = Function::new(Tag::from(*nreqs as i64), form).evict(env);

                Namespace::intern_symbol(env, ns, name.to_string(), func);

                (Tag::as_u64(&form), *featurefn)
            }));
        }
    }
}

pub trait Core {
    fn add_env(_: Env) -> Tag;
}

impl Core for Env {
    fn add_env(env: Env) -> Tag {
        let mut env_map_ref = block_on(LIB.env_map.write());
        let key = Symbol::keyword(&format!("{:07x}", env_map_ref.len()));

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
