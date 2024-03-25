//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! compile:
//!     function calls
//!     special forms
use crate::{
    async_::context::{Context, Core as _},
    core::{
        apply::Core as _,
        exception::{self, Condition, Exception},
        frame::Frame,
        mu::{Core as _, Mu},
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, Core as _},
        fixnum::Fixnum,
        function::Function,
        symbol::{Core as _, Symbol},
    },
};

// special forms
type SpecFn = fn(&Mu, Tag, &mut Vec<(Tag, Vec<Tag>)>) -> exception::Result<Tag>;
type SpecMap = (Tag, SpecFn);

// lexical environment
type LexicalEnv = Vec<(Tag, Vec<Tag>)>;

lazy_static! {
    static ref SPECMAP: Vec<SpecMap> = vec![
        (Symbol::keyword("async"), Compile::async_),
        (Symbol::keyword("if"), Compile::if_),
        (Symbol::keyword("lambda"), Compile::lambda),
        (Symbol::keyword("quote"), Compile::quoted_list),
    ];
}

pub struct Compile {}

impl Compile {
    pub fn if_(mu: &Mu, args: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        if Cons::length(mu, args) != Some(3) {
            return Err(Exception::new(Condition::Syntax, ":if", args));
        }

        let lambda = Symbol::keyword("lambda");

        let if_vec = vec![
            mu.if_,
            Cons::nth(mu, 0, args).unwrap(),
            Cons::vlist(mu, &[lambda, Tag::nil(), Cons::nth(mu, 1, args).unwrap()]),
            Cons::vlist(mu, &[lambda, Tag::nil(), Cons::nth(mu, 2, args).unwrap()]),
        ];

        Self::compile(mu, Cons::vlist(mu, &if_vec), lexenv)
    }

    pub fn quoted_list(mu: &Mu, list: Tag, _: &mut LexicalEnv) -> exception::Result<Tag> {
        if Cons::length(mu, list) != Some(1) {
            return Err(Exception::new(Condition::Syntax, ":quote", list));
        }

        Ok(Cons::new(Symbol::keyword("quote"), list).evict(mu))
    }

    pub fn special_form(
        mu: &Mu,
        name: Tag,
        args: Tag,
        lexenv: &mut LexicalEnv,
    ) -> exception::Result<Tag> {
        match SPECMAP.iter().copied().find(|spec| name.eq_(&spec.0)) {
            Some(spec) => spec.1(mu, args, lexenv),
            None => Err(Exception::new(Condition::Syntax, "specf", args)),
        }
    }

    // utilities
    pub fn list(mu: &Mu, body: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        let compile_results: exception::Result<Vec<Tag>> = Cons::iter(mu, body)
            .map(|cons| Self::compile(mu, Cons::car(mu, cons), lexenv))
            .collect();

        match compile_results {
            Ok(vec) => Ok(Cons::vlist(mu, &vec)),
            Err(e) => Err(e),
        }
    }

    pub fn lambda(mu: &Mu, args: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        fn compile_frame_symbols(mu: &Mu, lambda: Tag) -> exception::Result<Vec<Tag>> {
            let mut symvec = Vec::new();

            for cons in Cons::iter(mu, lambda) {
                let symbol = Cons::car(mu, cons);
                if symbol.type_of() == Type::Symbol {
                    match symvec.iter().rev().position(|lex| symbol.eq_(lex)) {
                        Some(_) => {
                            return Err(Exception::new(Condition::Syntax, "lexical", symbol))
                        }
                        _ => symvec.push(symbol),
                    }
                } else {
                    return Err(Exception::new(Condition::Type, "lexical", symbol));
                }
            }

            Ok(symvec)
        }

        let (lambda, body) = match args.type_of() {
            Type::Cons => {
                let lambda = Cons::car(mu, args);

                match lambda.type_of() {
                    Type::Null | Type::Cons => (lambda, Cons::cdr(mu, args)),
                    _ => return Err(Exception::new(Condition::Type, "lambda", args)),
                }
            }
            _ => return Err(Exception::new(Condition::Syntax, "lambda", args)),
        };

        let func = Function::new(
            Tag::from(Cons::length(mu, lambda).unwrap() as i64),
            Tag::nil(),
        )
        .evict(mu);

        match compile_frame_symbols(mu, lambda) {
            Ok(lexicals) => {
                lexenv.push((func, lexicals));
            }
            Err(e) => return Err(e),
        };

        let form = match Self::list(mu, body, lexenv) {
            Ok(form) => {
                let mut function = Function::to_image(mu, func);

                function.form = form;
                Function::update(mu, &function, func);

                Ok(func)
            }
            Err(e) => Err(e),
        };

        lexenv.pop();

        form
    }

    pub fn async_(mu: &Mu, args: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        let (func, arg_list) = match args.type_of() {
            Type::Cons => {
                let fn_arg = match Self::compile(mu, Cons::car(mu, args), lexenv) {
                    Ok(fn_) => match fn_.type_of() {
                        Type::Function => fn_,
                        Type::Symbol => {
                            let sym_val = Symbol::value(mu, fn_);
                            match sym_val.type_of() {
                                Type::Function => sym_val,
                                _ => return Err(Exception::new(Condition::Type, "async", sym_val)),
                            }
                        }
                        _ => return Err(Exception::new(Condition::Type, "async", fn_)),
                    },
                    Err(e) => return Err(e),
                };

                let async_args = match Self::list(mu, Cons::cdr(mu, args), lexenv) {
                    Ok(list) => list,
                    Err(e) => return Err(e),
                };

                let arity = Fixnum::as_i64(Function::arity(mu, fn_arg));
                if arity != Cons::length(mu, async_args).unwrap() as i64 {
                    return Err(Exception::new(Condition::Arity, "async", args));
                }

                (fn_arg, async_args)
            }
            _ => return Err(Exception::new(Condition::Syntax, "async", args)),
        };

        match Context::context(mu, func, arg_list) {
            Ok(asyncid) => Ok(asyncid),
            Err(e) => Err(e),
        }
    }

    pub fn lexical(mu: &Mu, symbol: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        for frame in lexenv.iter().rev() {
            let (tag, symbols) = frame;

            if let Some(nth) = symbols.iter().position(|lex| symbol.eq_(lex)) {
                let lex_ref = vec![
                    Namespace::intern_symbol(mu, mu.core_ns, "fr-ref".to_string(), Tag::nil()),
                    Tag::from(tag.as_u64() as i64),
                    Tag::from(nth as i64),
                ];

                match Self::compile(mu, Cons::vlist(mu, &lex_ref), lexenv) {
                    Ok(lexref) => return Ok(lexref),
                    Err(e) => return Err(e),
                }
            }
        }

        if Symbol::is_unbound(mu, symbol) {
            Ok(symbol)
        } else {
            let value = Symbol::value(mu, symbol);
            match value.type_of() {
                Type::Cons | Type::Symbol => Ok(symbol),
                _ => Ok(value),
            }
        }
    }

    pub fn compile(mu: &Mu, expr: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        match expr.type_of() {
            Type::Symbol => Self::lexical(mu, expr, lexenv),
            Type::Cons => {
                let func = Cons::car(mu, expr);
                let args = Cons::cdr(mu, expr);

                match func.type_of() {
                    Type::Keyword => match Self::special_form(mu, func, args, lexenv) {
                        Ok(form) => Ok(form),
                        Err(e) => Err(e),
                    },
                    Type::Symbol => match Self::list(mu, args, lexenv) {
                        Ok(args) => {
                            if Symbol::is_unbound(mu, func) {
                                Ok(Cons::new(func, args).evict(mu))
                            } else {
                                let fn_ = Symbol::value(mu, func);
                                match fn_.type_of() {
                                    Type::Function => Ok(Cons::new(fn_, args).evict(mu)),
                                    _ => Err(Exception::new(Condition::Type, "compile", func)),
                                }
                            }
                        }
                        Err(e) => Err(e),
                    },
                    Type::Function => match Self::list(mu, args, lexenv) {
                        Ok(args) => Ok(Cons::new(func, args).evict(mu)),
                        Err(e) => Err(e),
                    },
                    Type::Cons => match Self::list(mu, args, lexenv) {
                        Ok(arglist) => match Self::compile(mu, func, lexenv) {
                            Ok(fn_) => match fn_.type_of() {
                                Type::Function => Ok(Cons::new(fn_, arglist).evict(mu)),
                                _ => Err(Exception::new(Condition::Type, "compile", func)),
                            },
                            Err(e) => Err(e),
                        },
                        Err(e) => Err(e),
                    },
                    _ => Err(Exception::new(Condition::Type, "compile", func)),
                }
            }
            _ => Ok(expr),
        }
    }
}

pub trait MuFunction {
    fn core_compile(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn if__(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Compile {
    fn if__(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let test = fp.argv[0];
        let true_fn = fp.argv[1];
        let false_fn = fp.argv[2];

        fp.value = match mu.fp_argv_check("::if", &[Type::T, Type::Function, Type::Function], fp) {
            Ok(_) => match mu.apply(if test.null_() { false_fn } else { true_fn }, Tag::nil()) {
                Ok(tag) => tag,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_compile(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let mut lexenv: LexicalEnv = vec![];

        fp.value = match Self::compile(mu, fp.argv[0], &mut lexenv) {
            Ok(tag) => tag,
            Err(e) => return Err(e),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        compile::Compile,
        mu::{Core, Mu},
        types::{Tag, Type},
    };

    #[test]
    fn compile_test() {
        let config = match Mu::config("".to_string()) {
            Some(config) => config,
            None => return assert!(false),
        };

        let mu: &Mu = &Core::new(&config);

        match Compile::compile(mu, Tag::nil(), &mut vec![]) {
            Ok(form) => match form.type_of() {
                Type::Null => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
        match Compile::list(mu, Tag::nil(), &mut vec![]) {
            Ok(form) => match form.type_of() {
                Type::Null => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }
}
