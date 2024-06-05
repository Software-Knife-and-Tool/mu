//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! compile:
//!     function calls
//!     special forms
use crate::{
    core::{
        apply::Core as _,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, Core as _},
        fixnum::{Core as _, Fixnum},
        function::Function,
        namespace::Namespace,
        symbol::{Core as _, Symbol},
    },
};

// special forms
type SpecFn = fn(&Env, Tag, &mut Vec<(Tag, Vec<Tag>)>) -> exception::Result<Tag>;

lazy_static! {
    static ref SPECMAP: Vec<(Tag, SpecFn)> = vec![
        (Symbol::keyword("if"), Env::if_),
        (Symbol::keyword("lambda"), Env::lambda),
        (Symbol::keyword("quote"), Env::quoted_list),
    ];
}

// lexical environment
type LexEnv = Vec<(Tag, Vec<Tag>)>;

pub trait Compile {
    fn if_(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn quoted_list(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn special_form(&self, _: Tag, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn list(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn lambda(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn lexical(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
    fn compile(&self, _: Tag, _: &mut LexEnv) -> exception::Result<Tag>;
}

impl Compile for Env {
    fn if_(&self, args: Tag, env: &mut LexEnv) -> exception::Result<Tag> {
        if Cons::length(self, args) != Some(3) {
            return Err(Exception::new(self, Condition::Syntax, "crux:%if", args));
        }

        let lambda = Symbol::keyword("lambda");

        let if_vec = vec![
            Namespace::intern(self, self.crux_ns, "%if".to_string(), Tag::nil()).unwrap(),
            Cons::vlist(
                self,
                &[lambda, Tag::nil(), Cons::nth(self, 0, args).unwrap()],
            ),
            Cons::vlist(
                self,
                &[lambda, Tag::nil(), Cons::nth(self, 1, args).unwrap()],
            ),
            Cons::vlist(
                self,
                &[lambda, Tag::nil(), Cons::nth(self, 2, args).unwrap()],
            ),
        ];

        self.compile(Cons::vlist(self, &if_vec), env)
    }

    fn quoted_list(&self, list: Tag, _: &mut LexEnv) -> exception::Result<Tag> {
        if Cons::length(self, list) != Some(1) {
            return Err(Exception::new(
                self,
                Condition::Syntax,
                "crux:compile",
                list,
            ));
        }

        Ok(Cons::new(Symbol::keyword("quote"), list).evict(self))
    }

    fn special_form(&self, name: Tag, args: Tag, env: &mut LexEnv) -> exception::Result<Tag> {
        match SPECMAP.iter().copied().find(|spec| name.eq_(&spec.0)) {
            Some(spec) => spec.1(self, args, env),
            None => Err(Exception::new(
                self,
                Condition::Syntax,
                "crux:compile",
                args,
            )),
        }
    }

    // utilities
    fn list(&self, body: Tag, env: &mut LexEnv) -> exception::Result<Tag> {
        let compile_results: exception::Result<Vec<Tag>> = Cons::iter(self, body)
            .map(|cons| self.compile(Cons::car(self, cons), env))
            .collect();

        Ok(Cons::vlist(self, &compile_results?))
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
                                "crux:compile",
                                symbol,
                            ))
                        }
                        _ => symvec.push(symbol),
                    }
                } else {
                    return Err(Exception::new(env, Condition::Type, "crux:compile", symbol));
                }
            }

            Ok(symvec)
        }

        let (lambda, body) = match args.type_of() {
            Type::Cons => {
                let lambda = Cons::car(self, args);

                match lambda.type_of() {
                    Type::Null | Type::Cons => (lambda, Cons::cdr(self, args)),
                    _ => return Err(Exception::new(self, Condition::Type, "crux:compile", args)),
                }
            }
            _ => {
                return Err(Exception::new(
                    self,
                    Condition::Syntax,
                    "crux:compile",
                    args,
                ))
            }
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

    fn lexical(&self, symbol: Tag, env: &mut LexEnv) -> exception::Result<Tag> {
        for frame in env.iter().rev() {
            let (tag, symbols) = frame;

            if let Some(nth) = symbols.iter().position(|lex| symbol.eq_(lex)) {
                let lex_ref = vec![
                    Namespace::intern(self, self.crux_ns, "frame-ref".to_string(), Tag::nil())
                        .unwrap(),
                    Fixnum::with_u64_or_panic(tag.as_u64()),
                    Fixnum::with_or_panic(nth),
                ];

                return self.compile(Cons::vlist(self, &lex_ref), env);
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
                                Type::Function => Ok(Cons::new(fn_, args).evict(self)),
                                _ => {
                                    Err(Exception::new(self, Condition::Type, "crux:compile", func))
                                }
                            }
                        } else {
                            Ok(Cons::new(func, args).evict(self))
                        }
                    }
                    Type::Function => Ok(Cons::new(func, self.list(args, env)?).evict(self)),
                    Type::Cons => {
                        let arglist = self.list(args, env)?;
                        let fn_ = self.compile(func, env)?;

                        match fn_.type_of() {
                            Type::Function => Ok(Cons::new(fn_, arglist).evict(self)),
                            _ => Err(Exception::new(self, Condition::Type, "crux:compile", func)),
                        }
                    }
                    _ => Err(Exception::new(self, Condition::Type, "crux:compile", func)),
                }
            }
            _ => Ok(expr),
        }
    }
}

pub trait CoreFunction {
    fn crux_compile(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_if(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn crux_if(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let test = fp.argv[0];
        let true_fn = fp.argv[1];
        let false_fn = fp.argv[2];

        env.fp_argv_check("crux:%if", &[Type::T, Type::Function, Type::Function], fp)?;

        let test = if env.apply(test, Tag::nil())?.null_() {
            false_fn
        } else {
            true_fn
        };

        fp.value = env.apply(test, Tag::nil())?;

        Ok(())
    }

    fn crux_compile(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let mut lexical_env: LexEnv = vec![];

        fp.value = env.compile(fp.argv[0], &mut lexical_env)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        compile::Compile,
        config::Config,
        env::Env,
        types::{Tag, Type},
    };

    #[test]
    fn compile_test() {
        let config = Config::new(None);
        let env: &Env = &Env::new(config.unwrap(), None);

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
