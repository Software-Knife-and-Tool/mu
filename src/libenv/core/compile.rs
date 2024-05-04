//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! compile:
//!     function calls
//!     special forms
use crate::{
    core::{
        apply::Core as _,
        env::{Core as _, Env},
        exception::{self, Condition, Exception},
        frame::Frame,
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, Core as _},
        function::Function,
        namespace::Namespace,
        symbol::{Core as _, Symbol},
    },
};

// special forms
type SpecFn = fn(&Env, Tag, &mut Vec<(Tag, Vec<Tag>)>) -> exception::Result<Tag>;
type SpecMap = (Tag, SpecFn);

// lexical environment
type LexicalEnv = Vec<(Tag, Vec<Tag>)>;

lazy_static! {
    static ref SPECMAP: Vec<SpecMap> = vec![
        (Symbol::keyword("if"), Compile::if_),
        (Symbol::keyword("lambda"), Compile::lambda),
        (Symbol::keyword("quote"), Compile::quoted_list),
    ];
}

pub struct Compile {}

impl Compile {
    pub fn if_(env: &Env, args: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        if Cons::length(env, args) != Some(3) {
            return Err(Exception::new(Condition::Syntax, ":if", args));
        }

        let lambda = Symbol::keyword("lambda");

        let if_vec = vec![
            Namespace::intern(env, env.lib_ns, "%if".to_string(), Tag::nil()).unwrap(),
            Cons::nth(env, 0, args).unwrap(),
            Cons::vlist(env, &[lambda, Tag::nil(), Cons::nth(env, 1, args).unwrap()]),
            Cons::vlist(env, &[lambda, Tag::nil(), Cons::nth(env, 2, args).unwrap()]),
        ];

        Self::compile(env, Cons::vlist(env, &if_vec), lexenv)
    }

    pub fn quoted_list(env: &Env, list: Tag, _: &mut LexicalEnv) -> exception::Result<Tag> {
        if Cons::length(env, list) != Some(1) {
            return Err(Exception::new(Condition::Syntax, ":quote", list));
        }

        Ok(Cons::new(Symbol::keyword("quote"), list).evict(env))
    }

    pub fn special_form(
        env: &Env,
        name: Tag,
        args: Tag,
        lexenv: &mut LexicalEnv,
    ) -> exception::Result<Tag> {
        match SPECMAP.iter().copied().find(|spec| name.eq_(&spec.0)) {
            Some(spec) => spec.1(env, args, lexenv),
            None => Err(Exception::new(Condition::Syntax, "specf", args)),
        }
    }

    // utilities
    pub fn list(env: &Env, body: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        let compile_results: exception::Result<Vec<Tag>> = Cons::iter(env, body)
            .map(|cons| Self::compile(env, Cons::car(env, cons), lexenv))
            .collect();

        match compile_results {
            Ok(vec) => Ok(Cons::vlist(env, &vec)),
            Err(e) => Err(e),
        }
    }

    pub fn lambda(env: &Env, args: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        fn compile_frame_symbols(env: &Env, lambda: Tag) -> exception::Result<Vec<Tag>> {
            let mut symvec = Vec::new();

            for cons in Cons::iter(env, lambda) {
                let symbol = Cons::car(env, cons);
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
                let lambda = Cons::car(env, args);

                match lambda.type_of() {
                    Type::Null | Type::Cons => (lambda, Cons::cdr(env, args)),
                    _ => return Err(Exception::new(Condition::Type, "lambda", args)),
                }
            }
            _ => return Err(Exception::new(Condition::Syntax, "lambda", args)),
        };

        let func = Function::new(
            Tag::from(Cons::length(env, lambda).unwrap() as i64),
            Tag::nil(),
        )
        .evict(env);

        match compile_frame_symbols(env, lambda) {
            Ok(lexicals) => {
                lexenv.push((func, lexicals));
            }
            Err(e) => return Err(e),
        };

        let form = match Self::list(env, body, lexenv) {
            Ok(form) => {
                let mut function = Function::to_image(env, func);

                function.form = form;
                Function::update(env, &function, func);

                Ok(func)
            }
            Err(e) => Err(e),
        };

        lexenv.pop();

        form
    }

    pub fn lexical(env: &Env, symbol: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        for frame in lexenv.iter().rev() {
            let (tag, symbols) = frame;

            if let Some(nth) = symbols.iter().position(|lex| symbol.eq_(lex)) {
                let lex_ref = vec![
                    Namespace::intern(env, env.lib_ns, "frame-ref".to_string(), Tag::nil())
                        .unwrap(),
                    Tag::from(tag.as_u64() as i64),
                    Tag::from(nth as i64),
                ];

                match Self::compile(env, Cons::vlist(env, &lex_ref), lexenv) {
                    Ok(lexref) => return Ok(lexref),
                    Err(e) => return Err(e),
                }
            }
        }

        if Symbol::is_bound(env, symbol) {
            let value = Symbol::value(env, symbol);
            match value.type_of() {
                Type::Cons | Type::Symbol => Ok(symbol),
                _ => Ok(value),
            }
        } else {
            Ok(symbol)
        }
    }

    pub fn compile(env: &Env, expr: Tag, lexenv: &mut LexicalEnv) -> exception::Result<Tag> {
        match expr.type_of() {
            Type::Symbol => Self::lexical(env, expr, lexenv),
            Type::Cons => {
                let func = Cons::car(env, expr);
                let args = Cons::cdr(env, expr);

                match func.type_of() {
                    Type::Keyword => match Self::special_form(env, func, args, lexenv) {
                        Ok(form) => Ok(form),
                        Err(e) => Err(e),
                    },
                    Type::Symbol => match Self::list(env, args, lexenv) {
                        Ok(args) => {
                            if Symbol::is_bound(env, func) {
                                let fn_ = Symbol::value(env, func);
                                match fn_.type_of() {
                                    Type::Function => Ok(Cons::new(fn_, args).evict(env)),
                                    _ => Err(Exception::new(Condition::Type, "compile", func)),
                                }
                            } else {
                                Ok(Cons::new(func, args).evict(env))
                            }
                        }
                        Err(e) => Err(e),
                    },
                    Type::Function => match Self::list(env, args, lexenv) {
                        Ok(args) => Ok(Cons::new(func, args).evict(env)),
                        Err(e) => Err(e),
                    },
                    Type::Cons => match Self::list(env, args, lexenv) {
                        Ok(arglist) => match Self::compile(env, func, lexenv) {
                            Ok(fn_) => match fn_.type_of() {
                                Type::Function => Ok(Cons::new(fn_, arglist).evict(env)),
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

pub trait CoreFunction {
    fn lib_compile(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_if(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Compile {
    fn lib_if(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let test = fp.argv[0];
        let true_fn = fp.argv[1];
        let false_fn = fp.argv[2];

        fp.value = match env.fp_argv_check(":if", &[Type::T, Type::Function, Type::Function], fp) {
            Ok(_) => match env.apply(if test.null_() { false_fn } else { true_fn }, Tag::nil()) {
                Ok(tag) => tag,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_compile(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let mut lexenv: LexicalEnv = vec![];

        fp.value = match Self::compile(env, fp.argv[0], &mut lexenv) {
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
        env::{Core, Env},
        types::{Tag, Type},
    };

    #[test]
    fn compile_test() {
        let config = match Env::config(None) {
            Some(config) => config,
            None => return assert!(false),
        };

        let env: &Env = &Core::new(&config);

        match Compile::compile(env, Tag::nil(), &mut vec![]) {
            Ok(form) => match form.type_of() {
                Type::Null => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
        match Compile::list(env, Tag::nil(), &mut vec![]) {
            Ok(form) => match form.type_of() {
                Type::Null => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }
}
