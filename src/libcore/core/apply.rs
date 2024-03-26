//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu functions
use {
    crate::{
        async_::context::{Context, MuFunction as _},
        core::{
            compile::{Compile, MuFunction as _},
            dynamic::MuFunction as _,
            exception::{self, Condition, Exception, MuFunction as _},
            frame::{Frame, MuFunction as _},
            gc::{Gc, MuFunction as _},
            heap::{Heap, MuFunction as _},
            mu::{Core as __, Mu},
            namespace::{MuFunction as _, Namespace},
            system::MuFunction as _,
            types::{MuFunction as _, Tag, Type},
        },
        system::{process::MuFunction as _, time::MuFunction as _, uname::MuFunction as _, System},
        types::{
            cons::{Cons, Core as _, MuFunction as _},
            fixnum::{Fixnum, MuFunction as _},
            float::{Float, MuFunction as _},
            function::Function,
            stream::Stream,
            streams::MuFunction as _,
            struct_::{MuFunction as _, Struct},
            symbol::{Core as _, MuFunction as _, Symbol, UNBOUND},
            vector::{MuFunction as _, Vector},
        },
    },
    std::collections::HashMap,
};

//
// native functions
//
pub type CoreFunction = fn(&Mu, &mut Frame) -> exception::Result<()>;

pub type CoreFunctionDef = (&'static str, u16, CoreFunction);

// mu function dispatch table
lazy_static! {
    static ref LIBCORE_SYMBOLS: Vec<CoreFunctionDef> = vec![
        // types
        ( "eq",      2, Tag::libcore_eq ),
        ( "type-of", 1, Tag::libcore_typeof ),
        ( "repr",    2, Tag::libcore_repr ),
        ( "view",    1, Tag::libcore_view ),
        // conses and lists
        ( "append",  2, Cons::libcore_append ),
        ( "car",     1, Cons::libcore_car ),
        ( "cdr",     1, Cons::libcore_cdr ),
        ( "cons",    2, Cons::libcore_cons ),
        ( "length",  1, Cons::libcore_length ),
        ( "nth",     2, Cons::libcore_nth ),
        ( "nthcdr",  2, Cons::libcore_nthcdr ),
        // async
        ( "await",   1, Context::libcore_await ),
        ( "abort",   1, Context::libcore_abort ),
        // compiler
        ( "compile", 1, Compile::libcore_compile ),
        // gc
        ( "gc",      0, Gc::libcore_gc ),
        // heap
        ( "hp-info", 0, Heap::libcore_hp_info ),
        ( "hp-stat", 0, Heap::libcore_hp_stat ),
        ( "hp-size", 1, Heap::libcore_hp_size ),
        // mu
        ( "apply",   2, Mu::libcore_apply ),
        ( "eval",    1, Mu::libcore_eval ),
        ( "frames",  0, Mu::libcore_frames ),
        ( "fix",     2, Mu::libcore_fix ),
        // exceptions
        ( "with-ex", 2, Exception::libcore_with_ex ),
        ( "raise",   2, Exception::libcore_raise ),
        // frames
        ( "fr-pop",  1, Frame::libcore_fr_pop ),
        ( "fr-push", 1, Frame::libcore_fr_push ),
        ( "fr-ref",  2, Frame::libcore_fr_ref ),
        // fixnums
        ( "ash",     2, Fixnum::libcore_ash ),
        ( "fx-add",  2, Fixnum::libcore_fxadd ),
        ( "fx-sub",  2, Fixnum::libcore_fxsub ),
        ( "fx-lt",   2, Fixnum::libcore_fxlt ),
        ( "fx-mul",  2, Fixnum::libcore_fxmul ),
        ( "fx-div",  2, Fixnum::libcore_fxdiv ),
        ( "logand",  2, Fixnum::libcore_logand ),
        ( "logor",   2, Fixnum::libcore_logor ),
        ( "lognot",  1, Fixnum::libcore_lognot ),
        // floats
        ( "fl-add",  2, Float::libcore_fladd ),
        ( "fl-sub",  2, Float::libcore_flsub ),
        ( "fl-lt",   2, Float::libcore_fllt ),
        ( "fl-mul",  2, Float::libcore_flmul ),
        ( "fl-div",  2, Float::libcore_fldiv ),
        // namespaces
        ( "untern",  2, Namespace::libcore_untern ),
        ( "intern",  3, Namespace::libcore_intern ),
        ( "make-ns", 1, Namespace::libcore_make_ns ),
        ( "ns-syms", 2, Namespace::libcore_ns_symbols ),
        ( "ns-find", 2, Namespace::libcore_ns_find ),
        ( "ns-map",  0, Namespace::libcore_ns_map ),
        // read/write
        ( "read",    3, Mu::libcore_read ),
        ( "write",   3, Mu::libcore_write ),
        // symbols
        ( "boundp",  1, Symbol::libcore_boundp ),
        ( "keyword", 1, Symbol::libcore_keyword ),
        ( "symbol",  1, Symbol::libcore_symbol ),
        ( "sy-name", 1, Symbol::libcore_name ),
        ( "sy-ns",   1, Symbol::libcore_ns ),
        ( "sy-val",  1, Symbol::libcore_value ),
        // simple vectors
        ( "vector",  2, Vector::libcore_make_vector ),
        ( "sv-len",  1, Vector::libcore_length ),
        ( "sv-ref",  2, Vector::libcore_svref ),
        ( "sv-type", 1, Vector::libcore_type ),
        // structs
        ( "struct",  2, Struct::libcore_make_struct ),
        ( "st-type", 1, Struct::libcore_struct_type ),
        ( "st-vec",  1, Struct::libcore_struct_vector ),
        // streams
        ( "close",   1, Stream::libcore_close ),
        ( "flush",   1, Stream::libcore_flush ),
        ( "get-str", 1, Stream::libcore_get_string ),
        ( "open",    3, Stream::libcore_open ),
        ( "openp",   1, Stream::libcore_openp ),
        ( "rd-byte", 3, Stream::libcore_read_byte ),
        ( "rd-char", 3, Stream::libcore_read_char ),
        ( "un-char", 2, Stream::libcore_unread_char ),
        ( "wr-byte", 2, Stream::libcore_write_byte ),
        ( "wr-char", 2, Stream::libcore_write_char ),
        // system
        ( "sys-tm",  0, System::sys_getrealtime ),
        ( "proc-tm", 0, System::sys_getproctime ),
        ( "getpid",  0, System::posix_getpid ),
        ( "getcwd",  0, System::posix_getcwd ),
        ( "uname",   0, System::posix_uname ),
        ( "sysinfo", 0, System::posix_sysinfo ),
    ];
}

pub trait Core {
    fn install_libcore_functions(_: &Mu) -> HashMap<u64, CoreFunction>;
    fn install_feature_functions(_: &Mu, _: Tag, _: Vec<CoreFunctionDef>);
    fn fp_argv_check(&self, _: &str, _: &[Type], _: &Frame) -> exception::Result<()>;
}

impl Core for Mu {
    fn install_libcore_functions(mu: &Mu) -> HashMap<u64, CoreFunction> {
        let mut functions = HashMap::<u64, CoreFunction>::new();

        functions.insert(Tag::as_u64(&Symbol::keyword("if")), Compile::if__);

        functions.extend(LIBCORE_SYMBOLS.iter().map(|(name, nreqs, libfn)| {
            let fn_key = Symbol::keyword(name);
            let func = Function::new(Tag::from(*nreqs as i64), fn_key).evict(mu);

            Namespace::intern_symbol(mu, mu.libcore_ns, name.to_string(), func);

            (Tag::as_u64(&fn_key), *libfn)
        }));

        functions
    }

    fn install_feature_functions(mu: &Mu, ns: Tag, symbols: Vec<CoreFunctionDef>) {
        let mut functions = mu.functions.borrow_mut();

        functions.extend(symbols.iter().map(|(name, nreqs, featurefn)| {
            let form = Namespace::intern_symbol(mu, ns, name.to_string(), *UNBOUND);
            let func = Function::new(Tag::from(*nreqs as i64), form).evict(mu);

            Namespace::intern_symbol(mu, ns, name.to_string(), func);

            (Tag::as_u64(&form), *featurefn)
        }));
    }

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

pub trait MuFunction {
    fn libcore_apply(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn libcore_eval(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn libcore_fix(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Mu {
    fn libcore_eval(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match mu.eval(fp.argv[0]) {
            Ok(tag) => tag,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn libcore_apply(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        fp.value = match mu.fp_argv_check("apply", &[Type::Function, Type::List], fp) {
            Ok(_) => {
                match (Frame {
                    func,
                    argv: Cons::iter(mu, args)
                        .map(|cons| Cons::car(mu, cons))
                        .collect::<Vec<Tag>>(),
                    value: Tag::nil(),
                })
                .apply(mu, func)
                {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn libcore_fix(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];

        fp.value = fp.argv[1];

        match func.type_of() {
            Type::Function => {
                loop {
                    let value = Tag::nil();
                    let argv = vec![fp.value];
                    let result = Frame { func, argv, value }.apply(mu, func);

                    fp.value = match result {
                        Ok(value) => {
                            if value.eq_(&fp.value) {
                                break;
                            }

                            value
                        }
                        Err(e) => return Err(e),
                    };
                }

                Ok(())
            }
            _ => Err(Exception::new(Condition::Type, "fix", func)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn libcore_functions() {
        assert_eq!(2 + 2, 4);
    }
}
