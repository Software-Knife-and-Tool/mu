//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// core struct
use {
    crate::{
        core::{
            apply::CoreFn as _,
            compiler::{Compiler, CoreFn as _},
            direct::DirectTag,
            env::Env,
            exception::{self, CoreFn as _, Exception},
            frame::{CoreFn as _, Frame},
            tag::{CoreFn as _, Tag},
        },
        features::feature::{Feature, FEATURES},
        namespaces::{
            gc::{CoreFn as _, GcContext},
            namespace::{CoreFn as _, Namespace},
        },
        streams::builder::StreamBuilder,
        types::{
            cons::{Cons, CoreFn as _},
            fixnum::{CoreFn as _, Fixnum},
            float::{CoreFn as _, Float},
            stream::{CoreFn as _, Stream},
            struct_::{CoreFn as _, Struct},
            symbol::{CoreFn as _, Symbol},
            vector::Vector,
        },
        vectors::vector::CoreFn as _,
    },
    futures_lite::future::block_on,
    futures_locks::RwLock,
    std::collections::HashMap,
};

lazy_static! {
    pub static ref CORE: Core = Core::new();
    pub static ref CORE_FUNCTIONS: &'static [CoreFnDef] = &[
        // types
        ( "eq",         2, Tag::mu_eq ),
        ( "type-of",    1, Tag::mu_typeof ),
        ( "repr",       1, Tag::mu_repr ),
        ( "unrepr",     1, Tag::mu_unrepr ),
        ( "view",       1, Tag::mu_view ),
        // conses and lists
        ( "append",     1, Cons::mu_append ),
        ( "car",        1, Cons::mu_car ),
        ( "cdr",        1, Cons::mu_cdr ),
        ( "cons",       2, Cons::mu_cons ),
        ( "length",     1, Cons::mu_length ),
        ( "nth",        2, Cons::mu_nth ),
        ( "nthcdr",     2, Cons::mu_nthcdr ),
        // compiler
        ( "compile",    1, Compiler::mu_compile ),
        ( "%if",        3, Compiler::mu_if),
        // gc
        ( "gc",         0, GcContext::mu_gc ),
        // env
        ( "apply",      2, Env::mu_apply ),
        ( "eval",       1, Env::mu_eval ),
        ( "fix",        2, Env::mu_fix ),
        // exceptions
        ( "with-exception",
                        2, Exception::mu_with_exception ),
        ( "raise",      2, Exception::mu_raise ),
        ( "raise-from", 3, Exception::mu_raise_from ),
        // frames
        ( "%frame-stack",
                        0, Frame::mu_frames ),
        ( "%frame-pop", 1, Frame::mu_frame_pop ),
        ( "%frame-push",
                        1, Frame::mu_frame_push ),
        ( "%frame-ref", 2, Frame::mu_frame_ref ),
        // fixnums
        ( "ash",        2, Fixnum::mu_ash ),
        ( "add",        2, Fixnum::mu_fxadd ),
        ( "sub",        2, Fixnum::mu_fxsub ),
        ( "less-than",  2, Fixnum::mu_fxlt ),
        ( "mul",        2, Fixnum::mu_fxmul ),
        ( "div",        2, Fixnum::mu_fxdiv ),
        ( "logand",     2, Fixnum::mu_logand ),
        ( "logor",      2, Fixnum::mu_logor ),
        ( "lognot",     1, Fixnum::mu_lognot ),
        // floats
        ( "fadd",       2, Float::mu_fladd ),
        ( "fsub",       2, Float::mu_flsub ),
        ( "fless-than", 2, Float::mu_fllt ),
        ( "fmul",       2, Float::mu_flmul ),
        ( "fdiv",       2, Float::mu_fldiv ),
        // namespaces
        ( "find",       2, Namespace::mu_find ),
        ( "find-namespace",
                        1, Namespace::mu_find_ns ),
        ( "intern",     3, Namespace::mu_intern ),
        ( "make-namespace",
                        1, Namespace::mu_make_ns ),
        ( "namespace-name",
                        1, Namespace::mu_ns_name ),
        // read/write
        ( "read",       3, Stream::mu_read ),
        ( "write",      3, Stream::mu_write ),
        // symbols
        ( "boundp",     1, Symbol::mu_boundp ),
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
        ( "svref",      2, Vector::mu_svref ),
        ( "vector-length",
                        1, Vector::mu_length ),
        ( "vector-type",
                        1, Vector::mu_type ),
        // structs
        ( "make-struct",
                        2, Struct::mu_make_struct ),
        ( "struct-type",
                        1, Struct::mu_struct_type ),
        ( "struct-vec", 1, Struct::mu_struct_vector ),
        // streams
        ( "close",      1, Stream::mu_close ),
        ( "flush",      1, Stream::mu_flush ),
        ( "get-string", 1, Stream::mu_get_string ),
        ( "open",       4, Stream::mu_open ),
        ( "openp",      1, Stream::mu_openp ),
        ( "read-byte",  3, Stream::mu_read_byte ),
        ( "read-char",  3, Stream::mu_read_char ),
        ( "unread-char",
                        2, Stream::mu_unread_char ),
        ( "write-byte", 2, Stream::mu_write_byte ),
        ( "write-char", 2, Stream::mu_write_char ),
    ];
}

pub type CoreFn = fn(&Env, &mut Frame) -> exception::Result<()>;
pub type CoreFnDef = (&'static str, u16, CoreFn);

pub struct Core {
    pub envs: RwLock<HashMap<u64, Env>>,
    pub features: Vec<Feature>,
    pub core_defs: Vec<CoreFnDef>,
    pub stdio: (Tag, Tag, Tag),
    pub stream_id: RwLock<u64>,
    pub streams: RwLock<HashMap<u64, RwLock<Stream>>>,
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

impl Core {
    pub fn new() -> Self {
        let mut core = Core {
            envs: RwLock::new(HashMap::new()),
            core_defs: FEATURES
                .features
                .iter()
                .filter(|feature| !feature.namespace.is_empty() && feature.functions.is_some())
                .flat_map(|feature| feature.functions.as_ref().unwrap().to_vec())
                .collect::<Vec<CoreFnDef>>(),
            features: FEATURES.features.clone(),
            stdio: (Tag::nil(), Tag::nil(), Tag::nil()),
            stream_id: RwLock::new(0),
            streams: RwLock::new(HashMap::new()),
        };

        core.stdio = (
            StreamBuilder::new().stdin().std_build(&core).unwrap(),
            StreamBuilder::new().stdout().std_build(&core).unwrap(),
            StreamBuilder::new().errout().std_build(&core).unwrap(),
        );

        core
    }

    pub fn map_core_function(func: Tag) -> CoreFnDef {
        let offset = DirectTag::function_destruct(func);
        let cf_len = CORE_FUNCTIONS.len();

        if offset < cf_len {
            CORE_FUNCTIONS[offset]
        } else {
            CORE.core_defs[offset - cf_len]
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
        Cons::list(
            env,
            &block_on(CORE.envs.read())
                .keys()
                .map(|key| Tag::from_slice(&key.to_le_bytes()))
                .collect::<Vec<Tag>>(),
        )
    }

    pub fn features_as_list(env: &Env) -> Tag {
        Cons::list(
            env,
            &CORE
                .features
                .iter()
                .map(|feature| Vector::from(feature.namespace.clone()).with_heap(env))
                .collect::<Vec<Tag>>(),
        )
    }

    pub fn nstreams() -> Tag {
        Fixnum::with_or_panic(block_on(CORE.streams.read()).len())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn core_test() {
        assert!(true);
    }
}
