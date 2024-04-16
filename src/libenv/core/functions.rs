//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! lib functions
use crate::{
    core::{
        apply::LibFunction as _,
        compile::{Compile, LibFunction as _},
        dynamic::LibFunction as _,
        env::Env,
        exception::{self, Exception, LibFunction as _},
        frame::{Frame, LibFunction as _},
        future::{Future, LibFunction as _},
        gc::{Gc, LibFunction as _},
        heap::{Heap, LibFunction as _},
        namespace::{LibFunction as _, Namespace},
        types::{LibFunction as _, Tag},
        utime::LibFunction as _,
    },
    streams::{read::LibFunction as _, write::LibFunction as _},
    types::{
        cons::{Cons, LibFunction as _},
        fixnum::{Fixnum, LibFunction as _},
        float::{Float, LibFunction as _},
        stream::Stream,
        streams::LibFunction as _,
        struct_::{LibFunction as _, Struct},
        symbol::{LibFunction as _, Symbol},
        vector::{LibFunction as _, Vector},
    },
};

pub type LibFn = fn(&Env, &mut Frame) -> exception::Result<()>;
pub type LibFnDef = (&'static str, u16, LibFn);

lazy_static! {
    pub static ref LIB_SYMBOLS: Vec<LibFnDef> = vec![
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
        // gc
        ( "gc",      0, Gc::lib_gc ),
        // heap
        ( "hp-info", 0, Heap::lib_hp_info ),
        ( "hp-stat", 0, Heap::lib_hp_stat ),
        ( "hp-size", 1, Heap::lib_hp_size ),
        // env
        ( "apply",   2, Env::lib_apply ),
        ( "eval",    1, Env::lib_eval ),
        ( "frames",  0, Env::lib_frames ),
        ( "fix",     2, Env::lib_fix ),
        // futures
        ( "fwait",   1, Future::lib_future_wait ),
        ( "future",  2, Future::lib_future ),
        ( "fdone",   1, Future::lib_future_complete ),
        // exceptions
        ( "with-ex", 2, Exception::lib_with_ex ),
        ( "raise",   2, Exception::lib_raise ),
        // frames
        ( "fr-pop",  1, Frame::lib_fr_pop ),
        ( "fr-push", 1, Frame::lib_fr_push ),
        ( "fr-ref",  2, Frame::lib_fr_ref ),
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
        ( "intern",  3, Namespace::lib_intern ),
        ( "make-ns", 1, Namespace::lib_make_ns ),
        ( "ns-find", 2, Namespace::lib_ns_find ),
        ( "ns-map",  0, Namespace::lib_ns_map ),
        ( "ns-syms", 2, Namespace::lib_ns_symbols ),
        ( "unbound", 2, Namespace::lib_unbound ),
        // read/write
        ( "read",    3, Env::lib_read ),
        ( "write",   3, Env::lib_write ),
        // symbols
        ( "boundp",  1, Symbol::lib_boundp ),
        ( "keyword", 1, Symbol::lib_keyword ),
        ( "symbol",  1, Symbol::lib_symbol ),
        ( "sy-name", 1, Symbol::lib_name ),
        ( "sy-ns",   1, Symbol::lib_ns ),
        ( "sy-val",  1, Symbol::lib_value ),
        // simple vectors
        ( "vector",  2, Vector::lib_make_vector ),
        ( "sv-len",  1, Vector::lib_length ),
        ( "sv-ref",  2, Vector::lib_svref ),
        ( "sv-type", 1, Vector::lib_type ),
        // structs
        ( "struct",  2, Struct::lib_make_struct ),
        ( "st-type", 1, Struct::lib_struct_type ),
        ( "st-vec",  1, Struct::lib_struct_vector ),
        // streams
        ( "close",   1, Stream::lib_close ),
        ( "flush",   1, Stream::lib_flush ),
        ( "get-str", 1, Stream::lib_get_string ),
        ( "open",    3, Stream::lib_open ),
        ( "openp",   1, Stream::lib_openp ),
        ( "rd-byte", 3, Stream::lib_read_byte ),
        ( "rd-char", 3, Stream::lib_read_char ),
        ( "un-char", 2, Stream::lib_unread_char ),
        ( "wr-byte", 2, Stream::lib_write_byte ),
        ( "wr-char", 2, Stream::lib_write_char ),
        // utime
        ( "utime",   0, Env::lib_utime ),
    ];
}
