//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// lambda compiler
#![allow(dead_code)]
#[rustfmt::skip]
use crate::{
    core_::{
        env::Env,
        exception::{self, Condition, Exception},
        namespace::Namespace,
        type_::{Type},
        tag::{Tag},
    },
    types::{
        cons::Cons,
        fixnum::Fixnum,
        symbol::Symbol,
        vector::Vector
    },
};

// lexical environment
pub struct Lambda {
    pub function: Tag,
    pub symbols: Vec<String>,
}

impl Lambda {
    fn parse_lambda(env: &Env, args: Tag) -> exception::Result<(Tag, Tag, Vec<Tag>)> {
        let compile_frame_symbols = |lambda: Tag| -> exception::Result<Vec<Tag>> {
            let mut symvec = Vec::new();

            for symbol in Cons::list_iter(env, lambda) {
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
                    Err(Exception::new(env, Condition::Type, "mu:compile", symbol))?
                }
            }

            Ok(symvec)
        };

        let (lambda, body) = match args.type_of() {
            Type::Cons => {
                let cons = Cons::destruct(env, args);

                match cons.0.type_of() {
                    Type::Null | Type::Cons => cons,
                    _ => return Err(Exception::new(env, Condition::Type, "mu:compile", args)),
                }
            }
            _ => return Err(Exception::new(env, Condition::Syntax, "mu:compile", args)),
        };

        Ok((lambda, body, compile_frame_symbols(lambda)?))
    }

    fn lexical(env: &Env, symbol: Tag, lexical_env: &mut [Lambda]) -> exception::Result<Tag> {
        assert_eq!(symbol.type_of(), Type::Symbol);

        let name = Vector::as_string(env, Symbol::destruct(env, symbol).1);
        for frame in lexical_env.iter().rev() {
            let Lambda { function, symbols } = frame;

            if let Some(nth) = symbols.iter().position(|lexical| *lexical == name) {
                let lex_ref = vec![
                    Namespace::intern(env, env.mu_ns, "%frame-ref".into(), Tag::nil()).unwrap(),
                    *function,
                    Fixnum::with_or_panic(nth),
                ];

                return Ok(Cons::list(env, &lex_ref));
            }
        }

        if Symbol::is_bound(env, symbol) {
            let value = Symbol::destruct(env, symbol).2;

            match value.type_of() {
                Type::Cons | Type::Symbol => Ok(symbol),
                _ => Ok(value),
            }
        } else {
            Ok(symbol)
        }
    }
}

#[cfg(test)]
mod tests {}
