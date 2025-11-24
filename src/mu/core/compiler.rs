//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//
//  runtime compiler
//
#[rustfmt::skip]
use {
    crate::{
        core::{
            apply::Apply as _,
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            tag::Tag,
            type_::Type,
        },
        namespaces::namespace::Namespace,
        types::{
            cons::Cons,
            fixnum::Fixnum,
            function::Function,
            symbol::{Symbol, UNBOUND},
        },
    },
    std::sync::LazyLock,
};

static COMPILER: LazyLock<Compiler> = LazyLock::new(|| Compiler {
    lambda: Symbol::keyword("lambda"),
    quote: Symbol::keyword("quote"),
    specmap: vec![
        (Symbol::keyword("alambda"), Compiler::compile_alambda),
        (Symbol::keyword("if"), Compiler::compile_if),
        (Symbol::keyword("lambda"), Compiler::compile_lambda),
        (
            Symbol::keyword("quote"),
            (|env: &Env, list: Tag, _: &mut LexEnv| Ok(Compiler::quote(env, &list))),
        ),
    ],
});

type LexEnv = Vec<(Tag, Vec<Tag>)>;
type CompilerSpecFn = fn(&Env, Tag, &mut LexEnv) -> exception::Result<Tag>;

pub struct Compiler {
    lambda: Tag,
    quote: Tag,
    specmap: Vec<(Tag, CompilerSpecFn)>,
}

impl Compiler {
    // special forms
    fn compile_lambda(env: &Env, form: Tag, lex_env: &mut LexEnv) -> exception::Result<Tag> {
        let (lambda, body, symbols) = Self::lambda(env, form)?;

        let function = Function::new(
            Fixnum::with_or_panic(Cons::length(env, lambda).unwrap()),
            Tag::nil(),
        );

        let func = function.with_heap(env);
        let mut function = Function::to_image(env, func);

        lex_env.push((func, symbols));

        function.form = Self::list(env, body, lex_env)?;
        Function::update(env, &function, func);

        lex_env.pop();

        Ok(func)
    }

    // needs implementation
    fn compile_alambda(env: &Env, form: Tag, lex_env: &mut LexEnv) -> exception::Result<Tag> {
        let (lambda, body, symbols) = Self::lambda(env, form)?;

        let function = Function::new(
            Fixnum::with_or_panic(Cons::length(env, lambda).unwrap()),
            Tag::nil(),
        );

        let func = function.with_heap(env);
        let mut function = Function::to_image(env, func);

        lex_env.push((func, symbols));

        function.form = Self::list(env, body, lex_env)?;
        Function::update(env, &function, func);

        lex_env.pop();

        Ok(func)
    }

    fn compile_if(env: &Env, args: Tag, lex_env: &mut LexEnv) -> exception::Result<Tag> {
        if Cons::length(env, args) != Some(3) {
            Err(Exception::new(env, Condition::Syntax, ":if", args))?;
        }

        let if_vec = vec![
            Namespace::intern(env, env.mu_ns, "%if".into(), Tag::nil()).unwrap(),
            Cons::nth(env, 0, args).unwrap(),
            Cons::list(
                env,
                &[
                    COMPILER.lambda,
                    Tag::nil(),
                    Cons::nth(env, 1, args).unwrap(),
                ],
            ),
            Cons::list(
                env,
                &[
                    COMPILER.lambda,
                    Tag::nil(),
                    Cons::nth(env, 2, args).unwrap(),
                ],
            ),
        ];

        Self::compile(env, Cons::list(env, &if_vec), lex_env)
    }

    // quoting
    pub fn quote(env: &Env, form: &Tag) -> Tag {
        Cons::cons(env, COMPILER.quote, *form)
    }

    pub fn unquote(env: &Env, form: &Tag) -> Tag {
        assert!(Self::is_quoted(env, form));

        Cons::destruct(env, *form).1
    }

    pub fn is_quoted(env: &Env, form: &Tag) -> bool {
        match form.type_of() {
            Type::Cons => Cons::destruct(env, *form).0.eq_(&COMPILER.quote),
            _ => false,
        }
    }

    // utilities
    fn special_form(
        env: &Env,
        name: Tag,
        args: Tag,
        lex_env: &mut LexEnv,
    ) -> exception::Result<Tag> {
        match COMPILER
            .specmap
            .iter()
            .copied()
            .find(|spec| name.eq_(&spec.0))
        {
            Some(spec) => spec.1(env, args, lex_env),
            None => Err(Exception::new(env, Condition::Syntax, "mu:compile", args))?,
        }
    }

    fn lambda(env: &Env, form: Tag) -> exception::Result<(Tag, Tag, Vec<Tag>)> {
        let frame_symbols = |lambda: Tag| -> exception::Result<Vec<Tag>> {
            Cons::list_iter(env, lambda).try_fold(Tag::nil(), |_, symbol| {
                if symbol.type_of() != Type::Symbol {
                    Err(Exception::new(env, Condition::Type, "mu:compile", symbol))?;
                }
                Ok(Tag::nil())
            })?;

            let mut symvec = Vec::new();

            for symbol in Cons::list_iter(env, lambda) {
                match symvec.iter().rev().position(|lex| symbol.eq_(lex)) {
                    Some(_) => Err(Exception::new(env, Condition::Syntax, "mu:compile", symbol))?,
                    _ => symvec.push(symbol),
                }
            }

            Ok(symvec)
        };

        let (lambda, body) = match form.type_of() {
            Type::Cons => {
                let cons = Cons::destruct(env, form);

                match cons.0.type_of() {
                    Type::Null | Type::Cons => cons,
                    _ => Err(Exception::new(env, Condition::Type, "mu:compile", form))?,
                }
            }
            _ => Err(Exception::new(env, Condition::Syntax, "mu:compile", form))?,
        };

        Ok((lambda, body, frame_symbols(lambda)?))
    }

    fn list(env: &Env, body: Tag, lex_env: &mut LexEnv) -> exception::Result<Tag> {
        let compile_results: exception::Result<Vec<Tag>> = Cons::list_iter(env, body)
            .map(|expr| Self::compile(env, expr, lex_env))
            .collect();

        Ok(Cons::list(env, &compile_results?))
    }

    fn symbol(env: &Env, symbol: Tag, lex_env: &mut LexEnv) -> exception::Result<Tag> {
        let ns = Symbol::destruct(env, symbol).0;

        if ns.eq_(&UNBOUND) {
            let lex_sym = lex_env.iter().rev().find_map(|frame| {
                frame
                    .1
                    .iter()
                    .position(|lex| symbol.eq_(lex))
                    .map(|nth| (frame.0, nth))
            });

            match lex_sym {
                Some(_) => {
                    let sym_ref = match lex_sym {
                        Some((tag, nth)) => {
                            let frame_ref = Symbol::destruct(
                                env,
                                Namespace::intern(env, env.mu_ns, "%frame-ref".into(), Tag::nil())
                                    .unwrap(),
                            )
                            .2;

                            Cons::list(env, &[frame_ref, tag, Fixnum::with_or_panic(nth)])
                        }
                        None => symbol,
                    };

                    Ok(sym_ref)
                }
                None => Err(Exception::new(env, Condition::Type, "mu:compile", symbol))?,
            }
        } else {
            Ok(symbol)
        }
    }

    pub fn compile(env: &Env, expr: Tag, lex_env: &mut LexEnv) -> exception::Result<Tag> {
        match expr.type_of() {
            Type::Symbol => Self::symbol(env, expr, lex_env),
            Type::Cons => {
                let (func, args) = Cons::destruct(env, expr);

                match func.type_of() {
                    Type::Keyword => Ok(Self::special_form(env, func, args, lex_env)?),
                    Type::Symbol => {
                        let args = Self::list(env, args, lex_env)?;

                        if Symbol::is_bound(env, func) {
                            let fn_ = Symbol::destruct(env, func).2;

                            match fn_.type_of() {
                                Type::Function => Ok(Cons::cons(env, fn_, args)),
                                _ => Err(Exception::new(env, Condition::Type, "mu:compile", func)),
                            }
                        } else {
                            Ok(Cons::cons(env, func, args))
                        }
                    }
                    Type::Function => Ok(Cons::cons(env, func, Self::list(env, args, lex_env)?)),
                    Type::Cons => {
                        let arglist = Self::list(env, args, lex_env)?;
                        let fn_ = Self::compile(env, func, lex_env)?;

                        match fn_.type_of() {
                            Type::Function => Ok(Cons::cons(env, fn_, arglist)),
                            _ => Err(Exception::new(env, Condition::Type, "mu:compile", func)),
                        }
                    }
                    _ => Err(Exception::new(env, Condition::Type, "mu:compile", func)),
                }
            }
            _ => Ok(expr),
        }
    }
}

pub trait CoreFn {
    fn mu_compile(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_if(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Compiler {
    fn mu_if(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check(":if", &[Type::T, Type::Function, Type::Function], fp)?;

        let test = fp.argv[0];
        let true_fn = fp.argv[1];
        let false_fn = fp.argv[2];

        fp.value = env.apply(
            if Tag::null_(&test) { false_fn } else { true_fn },
            Tag::nil(),
        )?;

        Ok(())
    }

    fn mu_compile(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Self::compile(env, fp.argv[0], &mut vec![])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    /*
        use crate::mu::{
            compile::Compile,
            config::Config,
            env::Env,
            type_::{Tag, Type},
        };

        #[test]
        fn compile_test() {
            let config = Config::new(None);
            let env: &Env = &Env::new(&config.unwrap(), None);

            match env.compile(Tag::nil(), &mut vec![]) {
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
        */
}
