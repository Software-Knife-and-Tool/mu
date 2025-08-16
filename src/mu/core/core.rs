//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// Core struct
use {
    crate::{
        core::{
            apply::CoreFunction as _,
            compile::CoreFunction as _,
            direct::DirectTag,
            env::Env,
            exception::{self, CoreFunction as _, Exception},
            frame::{CoreFunction as _, Frame},
            gc::{CoreFunction as _, GcContext},
            namespace::{CoreFunction as _, Namespace},
            types::{CoreFunction as _, Tag},
        },
        features::feature::Feature,
        streams::builder::StreamBuilder,
        types::{
            cons::{Cons, CoreFunction as _},
            fixnum::{CoreFunction as _, Fixnum},
            float::{CoreFunction as _, Float},
            function::Function,
            stream::{CoreFunction as _, Stream},
            struct_::{CoreFunction as _, Struct},
            symbol::{CoreFunction as _, Symbol},
            vector::Vector,
        },
        vectors::vector::CoreFunction as _,
    },
    futures_lite::future::block_on,
    futures_locks::RwLock,
    std::collections::HashMap,
};

pub type CoreFunction = fn(&Env, &mut Frame) -> exception::Result<()>;
pub type CoreFunctionDef = (&'static str, u16, CoreFunction);

lazy_static! {
    pub static ref CORE: Core = Core::new().features().stdio();
    pub static ref CORE_FUNCTIONS: Vec<CoreFunctionDef> = vec![
        // types
        ( "eq",          2, Tag::mu_eq ),
        ( "type-of",     1, Tag::mu_typeof ),
        ( "repr",        1, Tag::mu_repr ),
        ( "unrepr",      1, Tag::mu_unrepr ),
        ( "view",        1, Tag::mu_view ),
        // conses and lists
        ( "append",      1, Cons::mu_append ),
        ( "car",         1, Cons::mu_car ),
        ( "cdr",         1, Cons::mu_cdr ),
        ( "cons",    2, Cons::mu_cons ),
        ( "length",  1, Cons::mu_length ),
        ( "nth",     2, Cons::mu_nth ),
        ( "nthcdr",  2, Cons::mu_nthcdr ),
        // compiler
        ( "compile", 1, Env::mu_compile ),
        ( "%if",     3, Env::mu_if),
        // gc
        ( "gc",      0, GcContext::mu_gc ),
        // env
        ( "apply",   2, Env::mu_apply ),
        ( "eval",    1, Env::mu_eval ),
        ( "fix",     2, Env::mu_fix ),
        // exceptions
        ( "with-exception",
                     2, Exception::mu_with_exception ),
        ( "raise",   2, Exception::mu_raise ),
        // frames
        ( "%frame-stack",
                     0, Frame::mu_frames ),
        ( "%frame-pop",
                     1, Frame::mu_frame_pop ),
        ( "%frame-push",
                     1, Frame::mu_frame_push ),
        ( "%frame-ref",
                     2, Frame::mu_frame_ref ),
        // fixnums
        ( "ash",     2, Fixnum::mu_ash ),
        ( "add",     2, Fixnum::mu_fxadd ),
        ( "sub",     2, Fixnum::mu_fxsub ),
        ( "less-than",
                     2, Fixnum::mu_fxlt ),
        ( "mul",     2, Fixnum::mu_fxmul ),
        ( "div",     2, Fixnum::mu_fxdiv ),
        ( "logand",  2, Fixnum::mu_logand ),
        ( "logor",   2, Fixnum::mu_logor ),
        ( "lognot",  1, Fixnum::mu_lognot ),
        // floats
        ( "fadd",    2, Float::mu_fladd ),
        ( "fsub",    2, Float::mu_flsub ),
        ( "fless-than",
                     2, Float::mu_fllt ),
        ( "fmul",    2, Float::mu_flmul ),
        ( "fdiv",    2, Float::mu_fldiv ),
        // namespaces
        ( "find",    2, Namespace::mu_find ),
        ( "find-namespace",
           1, Namespace::mu_find_ns ),
        ( "intern",  3, Namespace::mu_intern ),
        ( "make-namespace",
                     1, Namespace::mu_make_ns ),
        ( "namespace-name",
                     1, Namespace::mu_ns_name ),
        ( "namespace-symbols",
                     1, Namespace::mu_ns_symbols ),
        // read/write
        ( "read",    3, Stream::mu_read ),
        ( "write",   3, Stream::mu_write ),
        // symbols
        ( "boundp",  1, Symbol::mu_boundp ),
        ( "make-symbol",
                     1, Symbol::mu_symbol ),
        ( "symbol-name",
                     1, Symbol::mu_name ),
        ( "symbol-namespace",
                     1, Symbol::mu_ns ),
        ( "symbol-value",
                     1, Symbol::mu_value ),
        // simple vectors
        ( "make-vector",
                     2, Vector::mu_make_vector ),
        ( "svref",   2, Vector::mu_svref ),
        ( "vector-length",
                     1, Vector::mu_length ),
        ( "vector-type",
                         1, Vector::mu_type ),
        // structs
        ( "make-struct",
                         2, Struct::mu_make_struct ),
        ( "struct-type",
                         1, Struct::mu_struct_type ),
        ( "struct-vec",  1, Struct::mu_struct_vector ),
        // streams
        ( "close",       1, Stream::mu_close ),
        ( "flush",       1, Stream::mu_flush ),
        ( "get-string",  1, Stream::mu_get_string ),
        ( "open",        4, Stream::mu_open ),
        ( "openp",       1, Stream::mu_openp ),
        ( "read-byte",   3, Stream::mu_read_byte ),
        ( "read-char",   3, Stream::mu_read_char ),
        ( "unread-char", 2, Stream::mu_unread_char ),
        ( "write-byte",  2, Stream::mu_write_byte ),
        ( "write-char",  2, Stream::mu_write_char ),
    ];
}

pub struct Core {
    pub envs: RwLock<HashMap<u64, Env>>,
    pub features: RwLock<Vec<Feature>>,
    pub stdio: RwLock<(Tag, Tag, Tag)>,
    pub stream_id: RwLock<u64>,
    pub streams: RwLock<HashMap<u64, RwLock<Stream>>>,
    pub symbols: RwLock<HashMap<String, Tag>>,
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

impl Core {
    pub fn new() -> Self {
        Core {
            envs: RwLock::new(HashMap::new()),
            features: RwLock::new(Vec::new()),
            stdio: RwLock::new((Tag::nil(), Tag::nil(), Tag::nil())),
            streams: RwLock::new(HashMap::new()),
            stream_id: RwLock::new(0),
            symbols: RwLock::new(HashMap::new()),
        }
    }

    // accessors
    pub fn features(self) -> Self {
        let mut features = block_on(self.features.write());

        *features = Feature::install_features();

        self
    }

    pub fn stdio(self) -> Self {
        let mut stdio = block_on(self.stdio.write());

        *stdio = (
            StreamBuilder::new().stdin().std_build(&self).unwrap(),
            StreamBuilder::new().stdout().std_build(&self).unwrap(),
            StreamBuilder::new().errout().std_build(&self).unwrap(),
        );

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

    // core/feature symbols
    pub fn symbols(env: &Env) {
        for (index, desc) in CORE_FUNCTIONS.iter().enumerate() {
            let (name, nreqs, _fn) = desc;

            let fn_ = DirectTag::cons(env.mu_ns, Fixnum::with_or_panic(index)).unwrap();
            let func = Function::new((*nreqs).into(), fn_).evict(env);

            Namespace::intern_static(env, env.mu_ns, (*name).into(), func).unwrap();
        }

        let features = block_on(CORE.features.read());

        for feature in &*features {
            let ns =
                Namespace::with_static(env, &feature.namespace, feature.symbols, feature.functions)
                    .unwrap();

            if let Some(functions) = feature.functions {
                for (index, desc) in functions.iter().enumerate() {
                    let (name, nreqs, _fn) = *desc;
                    let fn_ = DirectTag::cons(ns, Fixnum::with_or_panic(index)).unwrap();
                    let func = Function::new((nreqs).into(), fn_).evict(env);

                    Namespace::intern_static(env, ns, (*name).into(), func).unwrap();
                }
            }
        }
    }

    pub fn add_env(env: Env) -> Tag {
        let mut envs_ref = block_on(CORE.envs.write());
        let envs_len = envs_ref.len();
        let id = Symbol::keyword(&format!("{envs_len:06x}"));

        envs_ref.insert(id.as_u64(), env);

        id
    }

    pub fn envs_as_list(env: &Env) -> Tag {
        let envs_ref = block_on(CORE.envs.read());
        let envs = envs_ref
            .keys()
            .map(|key| Tag::from_slice(&key.to_le_bytes()))
            .collect::<Vec<Tag>>();

        Cons::list(env, &envs)
    }

    pub fn features_as_list(env: &Env) -> Tag {
        let features_ref = block_on(CORE.features.read());
        let features = features_ref
            .iter()
            .map(|feature| Vector::from(feature.namespace.clone()).evict(env))
            .collect::<Vec<Tag>>();

        Cons::list(env, &features)
    }

    pub fn nstreams() -> Tag {
        let streams_ref = block_on(CORE.streams.read());

        Fixnum::with_or_panic(streams_ref.len())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
