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
        utime::CoreFunction as _,
    },
    streams::{read::CoreFunction as _, write::CoreFunction as _},
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
    pub static ref LIB_SYMBOLS: Vec<CoreFnDef> = vec![
        // types
        ( "eq",      2, Tag::lib_eq ),
        ( "type-of", 1, Tag::lib_typeof ),
        ( "repr",    2, Tag::lib_repr ),
        ( "view",    1, Tag::lib_view ),
        // conses and lists
        ( "append",  2, Cons::lib_append ),
        ( "car",     1, Cons::lib_car ),
        ( "cdr",     1, Cons::lib_cdr ),
        ( "cons",    2, Cons::lib_cons ),
        ( "length",  1, Cons::lib_length ),
        ( "nth",     2, Cons::lib_nth ),
        ( "nthcdr",  2, Cons::lib_nthcdr ),
        // compiler
        ( "compile", 1, Compile::lib_compile ),
        ( "%if",     3, Compile::lib_if),
        // gc
        ( "gc",      0, Gc::lib_gc ),
        // heap
        ( "heap-info", 0, Heap::lib_hp_info ),
        ( "heap-stat", 0, Heap::lib_hp_stat ),
        ( "heap-size", 1, Heap::lib_hp_size ),
        // env
        ( "apply",   2, Env::lib_apply ),
        ( "eval",    1, Env::lib_eval ),
        ( "frames",  0, Env::lib_frames ),
        ( "fix",     2, Env::lib_fix ),
        // futures
        ( "defer",   2, Future::lib_future_defer ),
        ( "detach",  2, Future::lib_future_detach ),
        ( "poll",    1, Future::lib_future_poll ),
        ( "force",   1, Future::lib_future_force ),
        // exceptions
        ( "unwind-protect",  2, Exception::lib_unwind_protect ),
        ( "raise",           2, Exception::lib_raise ),
        // frames
        ( "frame-pop",  1, Frame::lib_fr_pop ),
        ( "frame-push", 1, Frame::lib_fr_push ),
        ( "frame-ref",  2, Frame::lib_fr_ref ),
        // fixnums
        ( "ash",     2, Fixnum::lib_ash ),
        ( "fx-add",  2, Fixnum::lib_fxadd ),
        ( "fx-sub",  2, Fixnum::lib_fxsub ),
        ( "fx-lt",   2, Fixnum::lib_fxlt ),
        ( "fx-mul",  2, Fixnum::lib_fxenvl ),
        ( "fx-div",  2, Fixnum::lib_fxdiv ),
        ( "logand",  2, Fixnum::lib_logand ),
        ( "logor",   2, Fixnum::lib_logor ),
        ( "lognot",  1, Fixnum::lib_lognot ),
        // floats
        ( "fl-add",  2, Float::lib_fladd ),
        ( "fl-sub",  2, Float::lib_flsub ),
        ( "fl-lt",   2, Float::lib_fllt ),
        ( "fl-mul",  2, Float::lib_flenvl ),
        ( "fl-div",  2, Float::lib_fldiv ),
        // namespaces
        ( "find",      2, Namespace::lib_find ),
        ( "find-ns",   1, Namespace::lib_find_ns ),
        ( "intern",    3, Namespace::lib_intern ),
        ( "make-ns",   1, Namespace::lib_make_ns ),
        ( "ns-map",    0, Namespace::lib_ns_map ),
        ( "ns-name",   1, Namespace::lib_ns_name ),
        ( "symbols",   1, Namespace::lib_symbols ),
        ( "unintern",  1, Namespace::lib_unintern ),
        ( "makunbound",  1, Namespace::lib_makunbound ),
        // read/write
        ( "read",    3, Env::lib_read ),
        ( "write",   3, Env::lib_write ),
        // symbols
        ( "boundp",        1, Symbol::lib_boundp ),
        ( "make-symbol",   1, Symbol::lib_symbol ),
        ( "symbol-name",   1, Symbol::lib_name ),
        ( "symbol-ns",     1, Symbol::lib_ns ),
        ( "symbol-value",  1, Symbol::lib_value ),
        // simple vectors
        ( "make-vector",  2, Vector::lib_make_vector ),
        ( "vector-len",  1, Vector::lib_length ),
        ( "vector-ref",  2, Vector::lib_svref ),
        ( "vector-type", 1, Vector::lib_type ),
        // structs
        ( "make-struct",  2, Struct::lib_make_struct ),
        ( "struct-type", 1, Struct::lib_struct_type ),
        ( "struct-vec",  1, Struct::lib_struct_vector ),
        // streams
        ( "close",   1, Stream::lib_close ),
        ( "flush",   1, Stream::lib_flush ),
        ( "get-string", 1, Stream::lib_get_string ),
        ( "open",    3, Stream::lib_open ),
        ( "openp",   1, Stream::lib_openp ),
        ( "read-byte", 3, Stream::lib_read_byte ),
        ( "read-char", 3, Stream::lib_read_char ),
        ( "unread-char", 2, Stream::lib_unread_char ),
        ( "write-byte", 2, Stream::lib_write_byte ),
        ( "write-char", 2, Stream::lib_write_char ),
        // utime
        ( "utime",   0, Env::lib_utime ),
    ];
}
