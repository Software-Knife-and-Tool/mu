//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env system functions
use futures::executor::block_on;
use futures_locks::RwLock;
use {
    crate::{
        allocators::bump_allocator::BumpAllocator,
        async_::context::{Context, LibFunction as _},
        core::{
            apply::LibFunction as _,
            compile::{Compile, LibFunction as _},
            direct::{DirectInfo, DirectTag, DirectType},
            dynamic::LibFunction as _,
            env::Env,
            exception::{self, Exception, LibFunction as _},
            frame::{Frame, LibFunction as _},
            gc::{Gc, LibFunction as _},
            heap::{Heap, LibFunction as _},
            namespace::{LibFunction as _, Namespace},
            types::{LibFunction as _, Tag},
            utime::LibFunction as _,
        },
        streams::{
            read::LibFunction as _,
            write::{Core as _, LibFunction as _},
        },
        types::{
            cons::{Cons, LibFunction as _},
            fixnum::{Fixnum, LibFunction as _},
            float::{Float, LibFunction as _},
            function::Function,
            stream::Stream,
            streams::LibFunction as _,
            struct_::{LibFunction as _, Struct},
            symbol::{Core as _, LibFunction as _, Symbol, UNBOUND},
            vector::{LibFunction as _, Vector},
        },
    },
    std::collections::HashMap,
};

//
// native functions
//
pub type LibFn = fn(&Env, &mut Frame) -> exception::Result<()>;
pub type LibFnDef = (&'static str, u16, LibFn);

lazy_static! {
    static ref LIB_SYMBOLS: Vec<LibFnDef> = vec![
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
        // async
        ( "await",   1, Context::lib_await ),
        ( "abort",   1, Context::lib_abort ),
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

pub struct Lib {
    pub eol: Tag,
    pub heap: RwLock<BumpAllocator>,
    pub streams: RwLock<Vec<RwLock<Stream>>>,
    pub symbols: RwLock<HashMap<String, Tag>>,
    pub functions: RwLock<HashMap<u64, LibFn>>,
    pub version: &'static str,
}

impl Lib {
    pub const VERSION: &'static str = "0.0.45";

    pub fn new() -> Self {
        let lib = Lib {
            eol: DirectTag::to_direct(0, DirectInfo::Length(0), DirectType::Keyword),
            functions: RwLock::new(HashMap::new()),
            heap: RwLock::new(BumpAllocator::new(10, Tag::NTYPES)),
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

    pub fn symbols() -> &'static RwLock<HashMap<String, Tag>> {
        &LIB.symbols
    }

    // lib symbols
    pub fn lib_symbols(env: &Env) {
        let mut functions = block_on(LIB.functions.write());

        functions.insert(Tag::as_u64(&Symbol::keyword("if")), Compile::if__);
        Namespace::intern_symbol(
            env,
            env.lib_ns,
            "if".to_string(),
            Function::new(Tag::from(3i64), Symbol::keyword("if")).evict(env),
        );

        functions.extend(LIB_SYMBOLS.iter().map(|(name, nreqs, libfn)| {
            let fn_key = Symbol::keyword(name);
            let func = Function::new(Tag::from(*nreqs as i64), fn_key).evict(env);

            Namespace::intern_symbol(env, env.lib_ns, name.to_string(), func);

            (Tag::as_u64(&fn_key), *libfn)
        }));
    }

    // features
    pub fn feature_symbols(env: &Env, ns: Tag, symbols: Vec<LibFnDef>) {
        let mut functions = block_on(LIB.functions.write());

        functions.extend(symbols.iter().map(|(name, nreqs, featurefn)| {
            let form = Namespace::intern_symbol(env, ns, name.to_string(), *UNBOUND);
            let func = Function::new(Tag::from(*nreqs as i64), form).evict(env);

            Namespace::intern_symbol(env, ns, name.to_string(), func);

            (Tag::as_u64(&form), *featurefn)
        }));
    }
}

lazy_static! {
    pub static ref LIB: Lib = Lib::new();
}

pub trait Core {
    fn debug_vprintln(&self, _: &str, _: bool, _: Tag);
    fn debug_vprint(&self, _: &str, _: bool, _: Tag);
}

impl Core for Env {
    // debug printing
    fn debug_vprint(&self, label: &str, verbose: bool, tag: Tag) {
        print!("{}: ", label);
        self.write_stream(tag, verbose, self.stdout).unwrap();
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
