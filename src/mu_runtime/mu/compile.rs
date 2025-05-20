//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! compile:
//!     function calls
//!     special forms
use crate::{
    mu::{
        apply::Apply as _,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::{cons::Cons, fixnum::Fixnum, function::Function, symbol::Symbol},
};

// special forms
type SpecFn = fn(&Env, Tag, &mut Vec<(Tag, Vec<Tag>)>) -> exception::Result<Tag>;

lazy_static! {
    static ref SPECMAP: Vec<(Tag, SpecFn)> = vec![
        (Symbol::keyword("if"), Env::if_),
        (Symbol::keyword("lambda"), Env::lambda),
        (Symbol::keyword("quote"), Env::quote_),
    ];
}

// lexical environment
type LexEnv = Vec<(Tag, Vec<Tag>)>;

pub trait Compile {
    fn compile(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn if_(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn is_quoted(&self, _: &Tag) -> bool;
    fn lambda(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn lexical(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn list(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn quote(&self, _: &Tag) -> Tag;
    fn quote_(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn quoted_form(&self, _: &Tag) -> Tag;
    fn special_form(&self, _: Tag, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
}

impl Compile for Env {
    fn if_(&self, args: Tag, env: &mut LexEnv) -> exception::Result<Tag> {
        if Cons::length(self, args) != Some(3) {
            return Err(Exception::new(self, Condition::Syntax, "mu:%if", args));
        }

        let lambda = Symbol::keyword("lambda");

        let if_vec = vec![
            Namespace::intern(self, self.mu_ns, "%if".into(), Tag::nil()).unwrap(),
            Cons::nth(self, 0, args).unwrap(),
            Cons::list(
                self,
                &[lambda, Tag::nil(), Cons::nth(self, 1, args).unwrap()],
            ),
            Cons::list(
                self,
                &[lambda, Tag::nil(), Cons::nth(self, 2, args).unwrap()],
            ),
        ];

        self.compile(Cons::list(self, &if_vec), env)
    }

    fn lambda(&self, args: Tag, env: &mut LexEnv) -> exception::Result<Tag> {
        fn compile_frame_symbols(env: &Env, lambda: Tag) -> exception::Result<Vec<Tag>> {
            let mut symvec = Vec::new();

            for cons in Cons::iter(env, lambda) {
                let symbol = Cons::car(env, cons);
                if symbol.type_of() == Type::Symbol {
                    match symvec.iter().rev().position(|lex| symbol.eq_(lex)) {
                        Some(_) => {
                            return Err(Exception::new(
                                env,
                                Condition::Syntax,
                                "mu:compile",
                                symbol,
                            ))
                        }
                        _ => symvec.push(symbol),
                    }
                } else {
                    return Err(Exception::new(env, Condition::Type, "mu:compile", symbol));
                }
            }

            Ok(symvec)
        }

        let (lambda, body) = match args.type_of() {
            Type::Cons => {
                let lambda = Cons::car(self, args);

                match lambda.type_of() {
                    Type::Null | Type::Cons => (lambda, Cons::cdr(self, args)),
                    _ => return Err(Exception::new(self, Condition::Type, "mu:compile", args)),
                }
            }
            _ => return Err(Exception::new(self, Condition::Syntax, "mu:compile", args)),
        };

        let func = Function::new(
            Fixnum::with_or_panic(Cons::length(self, lambda).unwrap()),
            Tag::nil(),
        )
        .evict(self);

        env.push((func, compile_frame_symbols(self, lambda)?));

        let form = self.list(body, env)?;
        let mut function = Function::to_image(self, func);

        function.form = form;
        Function::update(self, &function, func);

        env.pop();

        Ok(func)
    }

    fn quote_(&self, list: Tag, _: &mut LexEnv) -> exception::Result<Tag> {
        Ok(self.quote(&list))
    }

    // utilities
    fn special_form(&self, name: Tag, args: Tag, env: &mut LexEnv) -> exception::Result<Tag> {
        match SPECMAP.iter().copied().find(|spec| name.eq_(&spec.0)) {
            Some(spec) => spec.1(self, args, env),
            None => Err(Exception::new(self, Condition::Syntax, "mu:compile", args)),
        }
    }

    fn list(&self, body: Tag, env: &mut LexEnv) -> exception::Result<Tag> {
        let compile_results: exception::Result<Vec<Tag>> = Cons::iter(self, body)
            .map(|cons| self.compile(Cons::car(self, cons), env))
            .collect();

        Ok(Cons::list(self, &compile_results?))
    }

    fn lexical(&self, symbol: Tag, env: &mut LexEnv) -> exception::Result<Tag> {
        for frame in env.iter().rev() {
            let (tag, symbols) = frame;

            if let Some(nth) = symbols.iter().position(|lex| symbol.eq_(lex)) {
                let lex_ref = vec![
                    Namespace::intern(self, self.mu_ns, "%frame-ref".into(), Tag::nil()).unwrap(),
                    *tag,
                    Fixnum::with_or_panic(nth),
                ];

                return self.compile(Cons::list(self, &lex_ref), env);
            }
        }

        if Symbol::is_bound(self, symbol) {
            let value = Symbol::value(self, symbol);
            match value.type_of() {
                Type::Cons | Type::Symbol => Ok(symbol),
                _ => Ok(value),
            }
        } else {
            Ok(symbol)
        }
    }

    fn quote(&self, form: &Tag) -> Tag {
        Cons::cons(self, Symbol::keyword("quote"), *form)
    }

    fn quoted_form(&self, form: &Tag) -> Tag {
        if Self::is_quoted(self, form) {
            Cons::cdr(self, *form)
        } else {
            panic!()
        }
    }

    fn is_quoted(&self, form: &Tag) -> bool {
        match form.type_of() {
            Type::Cons => Cons::car(self, *form).eq_(&Symbol::keyword("quote")),
            _ => false,
        }
    }

    fn compile(&self, expr: Tag, env: &mut LexEnv) -> exception::Result<Tag> {
        match expr.type_of() {
            Type::Symbol => self.lexical(expr, env),
            Type::Cons => {
                let func = Cons::car(self, expr);
                let args = Cons::cdr(self, expr);

                match func.type_of() {
                    Type::Keyword => Ok(self.special_form(func, args, env)?),
                    Type::Symbol => {
                        let args = self.list(args, env)?;

                        if Symbol::is_bound(self, func) {
                            let fn_ = Symbol::value(self, func);
                            match fn_.type_of() {
                                Type::Function => Ok(Cons::cons(self, fn_, args)),
                                _ => Err(Exception::new(self, Condition::Type, "mu:compile", func)),
                            }
                        } else {
                            Ok(Cons::cons(self, func, args))
                        }
                    }
                    Type::Function => Ok(Cons::cons(self, func, self.list(args, env)?)),
                    Type::Cons => {
                        let arglist = self.list(args, env)?;
                        let fn_ = self.compile(func, env)?;

                        match fn_.type_of() {
                            Type::Function => Ok(Cons::cons(self, fn_, arglist)),
                            _ => Err(Exception::new(self, Condition::Type, "mu:compile", func)),
                        }
                    }
                    _ => Err(Exception::new(self, Condition::Type, "mu:compile", func)),
                }
            }
            _ => Ok(expr),
        }
    }
}

pub trait CoreFunction {
    fn mu_compile(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_if(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn mu_if(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let test = fp.argv[0];
        let true_fn = fp.argv[1];
        let false_fn = fp.argv[2];

        env.fp_argv_check("mu:%if", &[Type::T, Type::Function, Type::Function], fp)?;

        fp.value = env.apply(
            if Tag::null_(&test) { false_fn } else { true_fn },
            Tag::nil(),
        )?;

        Ok(())
    }

    fn mu_compile(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let mut lexical_env: LexEnv = vec![];

        fp.value = env.compile(fp.argv[0], &mut lexical_env)?;

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
            types::{Tag, Type},
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
