//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! lib symbols
use crate::{
    core::{
        apply::CoreFunction as _,
        compile::CoreFunction as _,
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
    system::utime::CoreFunction as _,
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
    pub static ref CRUX_SYMBOLS: Vec<CoreFnDef> = vec![
        // types
        ( "eq",      2, Tag::crux_eq ),
        ( "type-of", 1, Tag::crux_typeof ),
        ( "repr",    2, Tag::crux_repr ),
        ( "view",    1, Tag::crux_view ),
        // conses and lists
        ( "append",  2, Cons::crux_append ),
        ( "car",     1, Cons::crux_car ),
        ( "cdr",     1, Cons::crux_cdr ),
        ( "cons",    2, Cons::crux_cons ),
        ( "length",  1, Cons::crux_length ),
        ( "nth",     2, Cons::crux_nth ),
        ( "nthcdr",  2, Cons::crux_nthcdr ),
        // compiler
        ( "compile", 1, Env::crux_compile ),
        ( "%if",     3, Env::crux_if),
        // gc
        ( "gc",      0, Gc::crux_gc ),
        // heap
        ( "heap-info", 0, Heap::crux_hp_info ),
        ( "heap-stat", 0, Heap::crux_hp_stat ),
        ( "heap-size", 1, Heap::crux_hp_size ),
        // env
        ( "apply",   2, Env::crux_apply ),
        ( "eval",    1, Env::crux_eval ),
        ( "frames",  0, Env::crux_frames ),
        ( "fix",     2, Env::crux_fix ),
        // futures
        ( "defer",   2, Future::crux_future_defer ),
        ( "detach",  2, Future::crux_future_detach ),
        ( "poll",    1, Future::crux_future_poll ),
        ( "force",   1, Future::crux_future_force ),
        // exceptions
        ( "unwind-protect",  2, Exception::crux_unwind_protect ),
        ( "raise",           2, Exception::crux_raise ),
        // frames
        ( "frame-pop",  1, Frame::crux_fr_pop ),
        ( "frame-push", 1, Frame::crux_fr_push ),
        ( "frame-ref",  2, Frame::crux_fr_ref ),
        // fixnums
        ( "ash",         2, Fixnum::crux_ash ),
        ( "sum",         2, Fixnum::crux_fxadd ),
        ( "difference",  2, Fixnum::crux_fxsub ),
        ( "less-than",   2, Fixnum::crux_fxlt ),
        ( "product",     2, Fixnum::crux_fxmul ),
        ( "quotient",    2, Fixnum::crux_fxdiv ),
        ( "logand",  2, Fixnum::crux_logand ),
        ( "logor",   2, Fixnum::crux_logor ),
        ( "lognot",  1, Fixnum::crux_lognot ),
        // floats
        ( "fl-add",  2, Float::crux_fladd ),
        ( "fl-sub",  2, Float::crux_flsub ),
        ( "fl-lt",   2, Float::crux_fllt ),
        ( "fl-mul",  2, Float::crux_flmul ),
        ( "fl-div",  2, Float::crux_fldiv ),
        // namespaces
        ( "find",      2, Namespace::crux_find ),
        ( "find-ns",   1, Namespace::crux_find_ns ),
        ( "intern",    3, Namespace::crux_intern ),
        ( "make-ns",   1, Namespace::crux_make_ns ),
        ( "ns-map",    0, Namespace::crux_ns_map ),
        ( "ns-name",   1, Namespace::crux_ns_name ),
        ( "symbols",   1, Namespace::crux_symbols ),
        ( "unintern",  1, Namespace::crux_unintern ),
        ( "makunbound",  1, Namespace::crux_makunbound ),
        // read/write
        ( "read",    3, Env::crux_read ),
        ( "write",   3, Env::crux_write ),
        // symbols
        ( "boundp",        1, Symbol::crux_boundp ),
        ( "make-symbol",   1, Symbol::crux_symbol ),
        ( "symbol-name",   1, Symbol::crux_name ),
        ( "symbol-ns",     1, Symbol::crux_ns ),
        ( "symbol-value",  1, Symbol::crux_value ),
        // simple vectors
        ( "make-vector",  2, Vector::crux_make_vector ),
        ( "vector-len",  1, Vector::crux_length ),
        ( "vector-ref",  2, Vector::crux_svref ),
        ( "vector-type", 1, Vector::crux_type ),
        // structs
        ( "make-struct",  2, Struct::crux_make_struct ),
        ( "struct-type", 1, Struct::crux_struct_type ),
        ( "struct-vec",  1, Struct::crux_struct_vector ),
        // streams
        ( "close",   1, Stream::crux_close ),
        ( "flush",   1, Stream::crux_flush ),
        ( "get-string", 1, Stream::crux_get_string ),
        ( "open",    3, Stream::crux_open ),
        ( "openp",   1, Stream::crux_openp ),
        ( "read-byte", 3, Stream::crux_read_byte ),
        ( "read-char", 3, Stream::crux_read_char ),
        ( "unread-char", 2, Stream::crux_unread_char ),
        ( "write-byte", 2, Stream::crux_write_byte ),
        ( "write-char", 2, Stream::crux_write_char ),
        // system
        ( "utime",   0, Env::crux_utime ),
    ];
}
