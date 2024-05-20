//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! lib symbols
use crate::{
    core::{
        apply::CoreFunction as _,
        compile::{Compile, CoreFunction as _},
        dynamic::CoreFunction as _,
        env::Env,
        exception::{self, CoreFunction as _, Exception},
        frame::{CoreFunction as _, Frame},
        future::{CoreFunction as _, Future},
        gc::{CoreFunction as _, Gc},
        heap::{CoreFunction as _, Heap},
        types::{CoreFunction as _, Tag},
    },
    streams::{read::CoreFunction as _, write::CoreFunction as _},
    system::{
        utime::{CoreFunction as _}
    },
    types::{
        cons::{Cons, CoreFunction as _},
        core_stream::Stream,
        fixnum::{CoreFunction as _, Fixnum},
        float::{CoreFunction as _, Float},
        namespace::{CoreFunction as _, Namespace},
        stream::CoreFunction as _,
        struct_::{CoreFunction as _, Struct},
        symbol::{CoreFunction as _, Symbol},
        vector::{CoreFunction as _, Vector},
    },
};

pub type CoreFn = fn(&Env, &mut Frame) -> exception::Result<()>;
pub type CoreFnDef = (&'static str, u16, CoreFn);

lazy_static! {
    pub static ref CORE_SYMBOLS: Vec<CoreFnDef> = vec![
        // types
        ( "eq",      2, Tag::core_eq ),
        ( "type-of", 1, Tag::core_typeof ),
        ( "repr",    2, Tag::core_repr ),
        ( "view",    1, Tag::core_view ),
        // conses and lists
        ( "append",  2, Cons::core_append ),
        ( "car",     1, Cons::core_car ),
        ( "cdr",     1, Cons::core_cdr ),
        ( "cons",    2, Cons::core_cons ),
        ( "length",  1, Cons::core_length ),
        ( "nth",     2, Cons::core_nth ),
        ( "nthcdr",  2, Cons::core_nthcdr ),
        // compiler
        ( "compile", 1, Compile::core_compile ),
        ( "%if",     3, Compile::core_if),
        // gc
        ( "gc",      0, Gc::core_gc ),
        // heap
        ( "heap-info", 0, Heap::core_hp_info ),
        ( "heap-stat", 0, Heap::core_hp_stat ),
        ( "heap-size", 1, Heap::core_hp_size ),
        // env
        ( "apply",   2, Env::core_apply ),
        ( "eval",    1, Env::core_eval ),
        ( "frames",  0, Env::core_frames ),
        ( "fix",     2, Env::core_fix ),
        // futures
        ( "defer",   2, Future::core_future_defer ),
        ( "detach",  2, Future::core_future_detach ),
        ( "poll",    1, Future::core_future_poll ),
        ( "force",   1, Future::core_future_force ),
        // exceptions
        ( "unwind-protect",  2, Exception::core_unwind_protect ),
        ( "raise",           2, Exception::core_raise ),
        // frames
        ( "frame-pop",  1, Frame::core_fr_pop ),
        ( "frame-push", 1, Frame::core_fr_push ),
        ( "frame-ref",  2, Frame::core_fr_ref ),
        // fixnums
        ( "ash",         2, Fixnum::core_ash ),
        ( "sum",         2, Fixnum::core_fxadd ),
        ( "difference",  2, Fixnum::core_fxsub ),
        ( "less-than",   2, Fixnum::core_fxlt ),
        ( "product",     2, Fixnum::core_fxmul ),
        ( "quotient",    2, Fixnum::core_fxdiv ),
        ( "logand",  2, Fixnum::core_logand ),
        ( "logor",   2, Fixnum::core_logor ),
        ( "lognot",  1, Fixnum::core_lognot ),
        // floats
        ( "fl-add",  2, Float::core_fladd ),
        ( "fl-sub",  2, Float::core_flsub ),
        ( "fl-lt",   2, Float::core_fllt ),
        ( "fl-mul",  2, Float::core_flmul ),
        ( "fl-div",  2, Float::core_fldiv ),
        // namespaces
        ( "find",      2, Namespace::core_find ),
        ( "find-ns",   1, Namespace::core_find_ns ),
        ( "intern",    3, Namespace::core_intern ),
        ( "make-ns",   1, Namespace::core_make_ns ),
        ( "ns-map",    0, Namespace::core_ns_map ),
        ( "ns-name",   1, Namespace::core_ns_name ),
        ( "symbols",   1, Namespace::core_symbols ),
        ( "unintern",  1, Namespace::core_unintern ),
        ( "makunbound",  1, Namespace::core_makunbound ),
        // read/write
        ( "read",    3, Env::core_read ),
        ( "write",   3, Env::core_write ),
        // symbols
        ( "boundp",        1, Symbol::core_boundp ),
        ( "make-symbol",   1, Symbol::core_symbol ),
        ( "symbol-name",   1, Symbol::core_name ),
        ( "symbol-ns",     1, Symbol::core_ns ),
        ( "symbol-value",  1, Symbol::core_value ),
        // simple vectors
        ( "make-vector",  2, Vector::core_make_vector ),
        ( "vector-len",  1, Vector::core_length ),
        ( "vector-ref",  2, Vector::core_svref ),
        ( "vector-type", 1, Vector::core_type ),
        // structs
        ( "make-struct",  2, Struct::core_make_struct ),
        ( "struct-type", 1, Struct::core_struct_type ),
        ( "struct-vec",  1, Struct::core_struct_vector ),
        // streams
        ( "close",   1, Stream::core_close ),
        ( "flush",   1, Stream::core_flush ),
        ( "get-string", 1, Stream::core_get_string ),
        ( "open",    3, Stream::core_open ),
        ( "openp",   1, Stream::core_openp ),
        ( "read-byte", 3, Stream::core_read_byte ),
        ( "read-char", 3, Stream::core_read_char ),
        ( "unread-char", 2, Stream::core_unread_char ),
        ( "write-byte", 2, Stream::core_write_byte ),
        ( "write-char", 2, Stream::core_write_char ),
        // system
        ( "utime",   0, Env::core_utime ),
    ];
}
