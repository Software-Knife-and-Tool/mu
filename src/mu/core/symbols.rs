//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! core symbols
use crate::{
    core::{
        apply::CoreFunction as _,
        compile::CoreFunction as _,
        config::CoreFunction as _,
        core::CoreFnDef,
        dynamic::CoreFunction as _,
        env::Env,
        exception::{CoreFunction as _, Exception},
        frame::{CoreFunction as _, Frame},
        future::{CoreFunction as _, Future},
        gc::{CoreFunction as _, Gc},
        heap::{CoreFunction as _, HeapAllocator},
        namespace::Namespace,
        types::{CoreFunction as _, Tag},
    },
    streams::{read::CoreFunction as _, write::CoreFunction as _},
    types::{
        cons::{Cons, CoreFunction as _},
        fixnum::{CoreFunction as _, Fixnum},
        float::{CoreFunction as _, Float},
        namespace::CoreFunction as _,
        stream::{CoreFunction as _, Stream},
        struct_::{CoreFunction as _, Struct},
        symbol::{CoreFunction as _, Symbol},
        vector::Vector,
    },
    vectors::vector::CoreFunction as _,
};

lazy_static! {
    pub static ref MU_FUNCTIONS: Vec<CoreFnDef> = vec![
        // types
        ( "eq",     2, Tag::mu_eq ),
        ( "type-of", 1, Tag::mu_typeof ),
        ( "repr",   2, Tag::mu_repr ),
        ( "view",   1, Tag::mu_view ),
        // conses and lists
        ( "append", 1, Cons::mu_append ),
        ( "car",    1, Cons::mu_car ),
        ( "cdr",    1, Cons::mu_cdr ),
        ( "cons",   2, Cons::mu_cons ),
        ( "length", 1, Cons::mu_length ),
        ( "nth",    2, Cons::mu_nth ),
        ( "nthcdr", 2, Cons::mu_nthcdr ),
        // compiler
        ( "compile", 1, Env::mu_compile ),
        ( "%if",    3, Env::mu_if),
        // gc
        ( "gc",     0, Gc::mu_gc ),
        // heap
        ( "heap-info", 0, HeapAllocator::mu_hp_info ),
        ( "heap-stat", 0, HeapAllocator::mu_hp_stat ),
        ( "heap-size", 1, HeapAllocator::mu_hp_size ),
        // env
        ( "config",       0, Env::mu_config ),
        ( "apply",        2, Env::mu_apply ),
        ( "eval",         1, Env::mu_eval ),
        ( "%frame-stack", 0, Env::mu_frames ),
        ( "fix",          2, Env::mu_fix ),
        // futures
        ( "defer",  2, Future::mu_future_defer ),
        ( "detach", 2, Future::mu_future_detach ),
        ( "poll",   1, Future::mu_future_poll ),
        ( "force",  1, Future::mu_future_force ),
        // exceptions
        ( "with-exception", 2, Exception::mu_with_exception ),
        ( "raise",          2, Exception::mu_raise ),
        // frames
        ( "%frame-pop", 1, Frame::mu_fr_pop ),
        ( "%frame-push", 1, Frame::mu_fr_push ),
        ( "%frame-ref", 2, Frame::mu_fr_ref ),
        // fixnums
        ( "ash",        2, Fixnum::mu_ash ),
        ( "add",        2, Fixnum::mu_fxadd ),
        ( "sub", 2, Fixnum::mu_fxsub ),
        ( "less-than",  2, Fixnum::mu_fxlt ),
        ( "mul",    2, Fixnum::mu_fxmul ),
        ( "div",   2, Fixnum::mu_fxdiv ),
        ( "logand", 2, Fixnum::mu_logand ),
        ( "logor",  2, Fixnum::mu_logor ),
        ( "lognot", 1, Fixnum::mu_lognot ),
        // floats
        ( "fadd",        2, Float::mu_fladd ),
        ( "fsub", 2, Float::mu_flsub ),
        ( "fless-than",  2, Float::mu_fllt ),
        ( "fmul",    2, Float::mu_flmul ),
        ( "fdiv",   2, Float::mu_fldiv ),
        // namespaces
        ( "find",            2, Namespace::mu_find ),
        ( "find-namespace",  1, Namespace::mu_find_ns ),
        ( "intern",   	     3, Namespace::mu_intern ),
        ( "make-namespace",  1, Namespace::mu_make_ns ),
        ( "namespace-map",   0, Namespace::mu_ns_map ),
        ( "namespace-name",  1, Namespace::mu_ns_name ),
        ( "namespace-symbols",  1, Namespace::mu_ns_symbols ),
        // read/write
        ( "read",   3, Env::mu_read ),
        ( "write",  3, Env::mu_write ),
        // symbols
        ( "boundp",              1, Symbol::mu_boundp ),
        ( "make-symbol",         1, Symbol::mu_symbol ),
        ( "symbol-name",         1, Symbol::mu_name ),
        ( "symbol-namespace",    1, Symbol::mu_ns ),
        ( "symbol-value",        1, Symbol::mu_value ),
        // simple vectors
        ( "make-vector",  2, Vector::mu_make_vector ),
        ( "svref",        2, Vector::mu_svref ),
        ( "vector-length", 1, Vector::mu_length ),
        ( "vector-type",  1, Vector::mu_type ),
        // structs
        ( "make-struct", 2, Struct::mu_make_struct ),
        ( "struct-type", 1, Struct::mu_struct_type ),
        ( "struct-vec", 1, Struct::mu_struct_vector ),
        // streams
        ( "close",      1, Stream::mu_close ),
        ( "flush",      1, Stream::mu_flush ),
        ( "get-string", 1, Stream::mu_get_string ),
        ( "open",       4, Stream::mu_open ),
        ( "openp",      1, Stream::mu_openp ),
        ( "read-byte",  3, Stream::mu_read_byte ),
        ( "read-char",  3, Stream::mu_read_char ),
        ( "unread-char", 2, Stream::mu_unread_char ),
        ( "write-byte", 2, Stream::mu_write_byte ),
        ( "write-char", 2, Stream::mu_write_char ),
    ];
}
