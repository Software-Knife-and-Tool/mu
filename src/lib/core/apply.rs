//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu functions
use futures::executor::block_on;
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
            lib::MuFunction as _,
            mu::{Core as __, Mu},
            namespace::{MuFunction as _, Namespace},
            types::{MuFunction as _, Tag, Type},
            utime::MuFunction as _,
        },
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
    static ref LIB_SYMBOLS: Vec<CoreFunctionDef> = vec![
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
        // mu
        ( "apply",   2, Mu::lib_apply ),
        ( "eval",    1, Mu::lib_eval ),
        ( "frames",  0, Mu::lib_frames ),
        ( "fix",     2, Mu::lib_fix ),
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
        ( "fx-mul",  2, Fixnum::lib_fxmul ),
        ( "fx-div",  2, Fixnum::lib_fxdiv ),
        ( "logand",  2, Fixnum::lib_logand ),
        ( "logor",   2, Fixnum::lib_logor ),
        ( "lognot",  1, Fixnum::lib_lognot ),
        // floats
        ( "fl-add",  2, Float::lib_fladd ),
        ( "fl-sub",  2, Float::lib_flsub ),
        ( "fl-lt",   2, Float::lib_fllt ),
        ( "fl-mul",  2, Float::lib_flmul ),
        ( "fl-div",  2, Float::lib_fldiv ),
        // namespaces
        ( "untern",  2, Namespace::lib_untern ),
        ( "intern",  3, Namespace::lib_intern ),
        ( "make-ns", 1, Namespace::lib_make_ns ),
        ( "ns-syms", 2, Namespace::lib_ns_symbols ),
        ( "ns-find", 2, Namespace::lib_ns_find ),
        ( "ns-map",  0, Namespace::lib_ns_map ),
        // read/write
        ( "read",    3, Mu::lib_read ),
        ( "write",   3, Mu::lib_write ),
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
        ( "utime",   0, Mu::lib_utime ),
    ];
}

pub trait Core {
    fn install_lib_functions(_: &Mu) -> HashMap<u64, CoreFunction>;
    fn install_feature_functions(_: &Mu, _: Tag, _: Vec<CoreFunctionDef>);
    fn fp_argv_check(&self, _: &str, _: &[Type], _: &Frame) -> exception::Result<()>;
}

impl Core for Mu {
    fn install_lib_functions(mu: &Mu) -> HashMap<u64, CoreFunction> {
        let mut functions = HashMap::<u64, CoreFunction>::new();

        functions.insert(Tag::as_u64(&Symbol::keyword("if")), Compile::if__);

        functions.extend(LIB_SYMBOLS.iter().map(|(name, nreqs, libfn)| {
            let fn_key = Symbol::keyword(name);
            let func = Function::new(Tag::from(*nreqs as i64), fn_key).evict(mu);

            Namespace::intern_symbol(mu, mu.lib_ns, name.to_string(), func);

            (Tag::as_u64(&fn_key), *libfn)
        }));

        functions
    }

    fn install_feature_functions(mu: &Mu, ns: Tag, symbols: Vec<CoreFunctionDef>) {
        let mut functions = block_on(mu.functions.write());

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
    fn lib_apply(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_eval(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_fix(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Mu {
    fn lib_eval(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match mu.eval(fp.argv[0]) {
            Ok(tag) => tag,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_apply(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
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

    fn lib_fix(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
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
    fn lib_functions() {
        assert_eq!(2 + 2, 4);
    }
}
