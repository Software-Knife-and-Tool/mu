//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu functions
use {
    crate::{
        async_::context::{Context, MuFunction as _},
        core::{
            compiler::{Compiler, MuFunction as _},
            dynamic::MuFunction as _,
            exception::{self, Condition, Exception, MuFunction as _},
            frame::{Frame, MuFunction as _},
            gc::{Gc, MuFunction as _},
            heap::{Heap, MuFunction as _},
            mu::{Mu, MuFunction as _},
            namespace::{MuFunction as _, Namespace},
            system::MuFunction as _,
            types::{MuFunction as _, Tag, Type},
        },
        system::{process::MuFunction as _, time::MuFunction as _, uname::MuFunction as _, System},
        types::{
            cons::{Cons, MuFunction as _},
            fixnum::{Fixnum, MuFunction as _},
            float::{Float, MuFunction as _},
            function::Function,
            stream::Stream,
            streams::MuFunction as _,
            struct_::{MuFunction as _, Struct},
            symbol::{Core as _, MuFunction as _, Symbol},
            vector::{MuFunction as _, Vector},
        },
    },
    std::collections::HashMap,
};

//
// native functions
//
pub type LibMuFunction = fn(&Mu, &mut Frame) -> exception::Result<()>;

// mu function dispatch table
lazy_static! {
    static ref MU_SYMBOLS: Vec<(&'static str, u16, LibMuFunction)> = vec![
        // types
        ( "eq",      2, Tag::mu_eq ),
        ( "type-of", 1, Tag::mu_typeof ),
        ( "repr",    2, Tag::mu_repr ),
        ( "view",    1, Tag::mu_view ),
        // conses and lists
        ( "append",  2, Cons::mu_append ),
        ( "car",     1, Cons::mu_car ),
        ( "cdr",     1, Cons::mu_cdr ),
        ( "cons",    2, Cons::mu_cons ),
        ( "length",  1, Cons::mu_length ),
        ( "nth",     2, Cons::mu_nth ),
        ( "nthcdr",  2, Cons::mu_nthcdr ),
        // async
        ( "await",   1, Context::mu_await ),
        ( "abort",   1, Context::mu_abort ),
        // heap
        ( "gc",      0, Gc::mu_gc ),
        ( "hp-info", 0, Heap::mu_hp_info ),
        ( "hp-stat", 0, Heap::mu_hp_stat ),
        ( "hp-size", 1, Heap::mu_hp_size ),
        // mu
        ( "apply",   2, Mu::mu_apply ),
        ( "compile", 1, Compiler::mu_compile ),
        ( "eval",    1, Mu::mu_eval ),
        ( "frames",  0, Mu::mu_frames ),
        ( "fix",     2, Mu::mu_fix ),
        // exceptions
        ( "with-ex", 2, Exception::mu_with_ex ),
        ( "raise",   2, Exception::mu_raise ),
        // frames
        ( "fr-pop",  1, Frame::mu_fr_pop ),
        ( "fr-push", 1, Frame::mu_fr_push ),
        ( "fr-ref",  2, Frame::mu_fr_ref ),
        // fixnums
        ( "ash",     2, Fixnum::mu_ash ),
        ( "fx-add",  2, Fixnum::mu_fxadd ),
        ( "fx-sub",  2, Fixnum::mu_fxsub ),
        ( "fx-lt",   2, Fixnum::mu_fxlt ),
        ( "fx-mul",  2, Fixnum::mu_fxmul ),
        ( "fx-div",  2, Fixnum::mu_fxdiv ),
        ( "logand",  2, Fixnum::mu_logand ),
        ( "logor",   2, Fixnum::mu_logor ),
        ( "lognot",  1, Fixnum::mu_lognot ),
        // floats
        ( "fl-add",  2, Float::mu_fladd ),
        ( "fl-sub",  2, Float::mu_flsub ),
        ( "fl-lt",   2, Float::mu_fllt ),
        ( "fl-mul",  2, Float::mu_flmul ),
        ( "fl-div",  2, Float::mu_fldiv ),
        // namespaces
        ( "untern",  2, Namespace::mu_untern ),
        ( "intern",  3, Namespace::mu_intern ),
        ( "make-ns", 1, Namespace::mu_make_ns ),
        ( "ns-syms", 2, Namespace::mu_ns_symbols ),
        ( "ns-find", 2, Namespace::mu_ns_find ),
        ( "ns-map",  0, Namespace::mu_ns_map ),
        // read/write
        ( "read",    3, Mu::mu_read ),
        ( "write",   3, Mu::mu_write ),
        // symbols
        ( "boundp",  1, Symbol::mu_boundp ),
        ( "keyword", 1, Symbol::mu_keyword ),
        ( "symbol",  1, Symbol::mu_symbol ),
        ( "sy-name", 1, Symbol::mu_name ),
        ( "sy-ns",   1, Symbol::mu_ns ),
        ( "sy-val",  1, Symbol::mu_value ),
        // simple vectors
        ( "vector",  2, Vector::mu_make_vector ),
        ( "sv-len",  1, Vector::mu_length ),
        ( "sv-ref",  2, Vector::mu_svref ),
        ( "sv-type", 1, Vector::mu_type ),
        // structs
        ( "struct",  2, Struct::mu_make_struct ),
        ( "st-type", 1, Struct::mu_struct_type ),
        ( "st-vec",  1, Struct::mu_struct_vector ),
        // streams
        ( "close",   1, Stream::mu_close ),
        ( "flush",   1, Stream::mu_flush ),
        ( "get-str", 1, Stream::mu_get_string ),
        ( "open",    3, Stream::mu_open ),
        ( "openp",   1, Stream::mu_openp ),
        ( "rd-byte", 3, Stream::mu_read_byte ),
        ( "rd-char", 3, Stream::mu_read_char ),
        ( "un-char", 2, Stream::mu_unread_char ),
        ( "wr-byte", 2, Stream::mu_write_byte ),
        ( "wr-char", 2, Stream::mu_write_char ),
        // system
        ( "sys-tm",  0, System::sys_getrealtime ),
        ( "proc-tm", 0, System::sys_getproctime ),
        ( "getpid",  0, System::posix_getpid ),
        ( "getcwd",  0, System::posix_getcwd ),
        ( "uname",   0, System::posix_uname ),
        ( "spawn",   2, System::sys_spawn ),
        ( "sysinfo", 0, System::posix_sysinfo ),
        ( "exit",    1, System::posix_exit ),
    ];
}

impl Mu {
    pub fn install_lib_functions(mu: &Mu) -> HashMap<u64, LibMuFunction> {
        let mut fn_map = HashMap::<u64, LibMuFunction>::new();

        fn_map.insert(Tag::as_u64(&Symbol::keyword("if")), Mu::if_);

        fn_map.extend(MU_SYMBOLS.iter().map(|(name, nreqs, libfn)| {
            let fn_key = Symbol::keyword(name);
            let func = Function::new(Tag::from(*nreqs as i64), fn_key).evict(mu);

            Namespace::intern_symbol(mu, mu.mu_ns, name.to_string(), func);

            (Tag::as_u64(&fn_key), *libfn)
        }));

        fn_map
    }
}

pub trait Core {
    fn fp_argv_check(&self, _: &str, _: &[Type], _: &Frame) -> exception::Result<()>;
}

impl Core for Mu {
    fn fp_argv_check(&self, fn_name: &str, types: &[Type], fp: &Frame) -> exception::Result<()> {
        for (index, arg_type) in types.iter().enumerate() {
            let fp_arg = fp.argv[index];
            let fp_arg_type = fp_arg.type_of();

            match *arg_type {
                Type::Byte => match fp_arg_type {
                    Type::Fixnum => {
                        let n = Fixnum::as_i64(fp_arg);

                        if !(0..=255).contains(&n) {
                            return Err(Exception::new(Condition::Type, fn_name, fp_arg));
                        }
                    }
                    _ => return Err(Exception::new(Condition::Type, fn_name, fp_arg)),
                },
                Type::List => match fp_arg_type {
                    Type::Cons | Type::Null => (),
                    _ => return Err(Exception::new(Condition::Type, fn_name, fp_arg)),
                },
                Type::String => match fp_arg_type {
                    Type::Vector => {
                        if Vector::type_of(self, fp.argv[index]) != Type::Char {
                            return Err(Exception::new(Condition::Type, fn_name, fp_arg));
                        }
                    }
                    _ => return Err(Exception::new(Condition::Type, fn_name, fp_arg)),
                },
                Type::T => (),
                _ => {
                    if fp_arg_type != *arg_type {
                        return Err(Exception::new(Condition::Type, fn_name, fp_arg));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mu_functions() {
        assert_eq!(2 + 2, 4);
    }
}
