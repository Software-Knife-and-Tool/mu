//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! lambda compiler:
use crate::{
    core::{
        compile::Compile,
        env::Env,
        exception::{self, Condition, Exception},
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::{cons::Cons, fixnum::Fixnum, symbol::Symbol},
};

// lexical environment
pub struct Lambda {
    pub function: Tag,
    pub lambda: Vec<String>,
}

impl Lambda {
    fn parse_lambda(env: &Env, args: Tag) -> exception::Result<(Tag, Tag, Vec<Tag>)> {
        let compile_frame_symbols = |lambda: Tag| -> exception::Result<Vec<Tag>> {
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
        };

        let (lambda, body) = match args.type_of() {
            Type::Cons => {
                let lambda = Cons::car(env, args);

                match lambda.type_of() {
                    Type::Null | Type::Cons => (lambda, Cons::cdr(env, args)),
                    _ => return Err(Exception::new(env, Condition::Type, "mu:compile", args)),
                }
            }
            _ => return Err(Exception::new(env, Condition::Syntax, "mu:compile", args)),
        };

        Ok((lambda, body, compile_frame_symbols(lambda)?))
    }

    fn lexical(env: &Env, symbol: Tag, lexical_env: &mut Vec<Lambda>) -> exception::Result<Tag> {
        for frame in lexical_env.iter().rev() {
            let Lambda { function: tag, lambda: symbols } = frame;

            if let Some(nth) = symbols.iter().position(|lex| symbol.eq_(lex)) {
                let lex_ref = vec![
                    Namespace::intern(env, env.mu_ns, "%frame-ref".into(), Tag::nil()).unwrap(),
                    *tag,
                    Fixnum::with_or_panic(nth),
                ];

                return Compile::compile(env, Cons::list(env, &lex_ref), lexical_env);
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
}

#[cfg(test)]
mod tests {
}
