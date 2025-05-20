//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env exceptions:
//!    Condition
//!    Exception
//!    `Result<Exception>`
use futures_lite::future::block_on;
use {
    crate::{
        mu::{
            apply::Apply as _,
            env::Env,
            frame::Frame,
            types::{Tag, Type},
        },
        types::symbol::Symbol,
    },
    std::fmt,
};

pub type Result<T> = std::result::Result<T, Exception>;

#[derive(Clone)]
pub struct Exception {
    pub object: Tag,
    pub condition: Condition,
    pub source: Tag,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Condition {
    Arity,
    Eof,
    Error,
    Except,
    Exit,
    Future,
    Namespace,
    Open,
    Over,
    Quasi,
    Range,
    Read,
    SigInt,
    Stream,
    Syntax,
    Syscall,
    Type,
    Unbound,
    Under,
    Write,
    ZeroDivide,
}

lazy_static! {
    static ref CONDMAP: Vec<(Tag, Condition)> = vec![
        (Symbol::keyword("arity"), Condition::Arity),
        (Symbol::keyword("div0"), Condition::ZeroDivide),
        (Symbol::keyword("eof"), Condition::Eof),
        (Symbol::keyword("error"), Condition::Error),
        (Symbol::keyword("except"), Condition::Except),
        (Symbol::keyword("exit"), Condition::Exit),
        (Symbol::keyword("future"), Condition::Future),
        (Symbol::keyword("ns"), Condition::Namespace),
        (Symbol::keyword("open"), Condition::Open),
        (Symbol::keyword("over"), Condition::Over),
        (Symbol::keyword("quasi"), Condition::Quasi),
        (Symbol::keyword("range"), Condition::Range),
        (Symbol::keyword("read"), Condition::Read),
        (Symbol::keyword("sigint"), Condition::SigInt),
        (Symbol::keyword("stream"), Condition::Stream),
        (Symbol::keyword("syntax"), Condition::Syntax),
        (Symbol::keyword("syscall"), Condition::Syscall),
        (Symbol::keyword("type"), Condition::Type),
        (Symbol::keyword("unbound"), Condition::Unbound),
        (Symbol::keyword("under"), Condition::Under),
        (Symbol::keyword("write"), Condition::Write),
    ];
}

impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{}", self.condition, self.source)
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{}", self.condition, self.source)
    }
}

impl Exception {
    pub fn new(env: &Env, condition: Condition, symbol: &str, object: Tag) -> Self {
        let source = Symbol::parse(env, symbol.into()).unwrap();

        Exception {
            object,
            condition,
            source,
        }
    }

    fn map_condition(env: &Env, keyword: Tag) -> Result<Condition> {
        let condmap = CONDMAP.iter().find(|cond| keyword.eq_(&cond.0));

        match condmap {
            Some(entry) => Ok(entry.1.clone()),
            _ => Err(Exception::new(env, Condition::Syntax, "mu:raise", keyword)),
        }
    }

    fn map_condkey(cond: Condition) -> Result<Tag> {
        let condmap = CONDMAP.iter().find(|condtab| cond == condtab.1);

        match condmap {
            Some(entry) => Ok(entry.0),
            _ => panic!(),
        }
    }
}

pub trait CoreFunction {
    fn mu_with_exception(env: &Env, fp: &mut Frame) -> Result<()>;
    fn mu_raise(env: &Env, fp: &mut Frame) -> Result<()>;
}

impl CoreFunction for Exception {
    fn mu_raise(env: &Env, fp: &mut Frame) -> Result<()> {
        let src = fp.argv[0];
        let condition = fp.argv[1];

        env.fp_argv_check("mu:raise", &[Type::T, Type::Keyword], fp)?;
        match Self::map_condition(env, condition) {
            Ok(cond) => Err(Self::new(env, cond, "mu:raise", src)),
            Err(_) => Err(Self::new(env, Condition::Type, "mu:raise", condition)),
        }
    }

    fn mu_with_exception(env: &Env, fp: &mut Frame) -> Result<()> {
        let handler = fp.argv[0];
        let thunk = fp.argv[1];

        env.fp_argv_check("mu:with-exception", &[Type::Function, Type::Function], fp)?;

        let dynamic_ref = block_on(env.dynamic.dynamic.read());

        let frame_stack_len = dynamic_ref.len();

        drop(dynamic_ref);

        fp.value = match env.apply(thunk, Tag::nil()) {
            Ok(value) => value,
            Err(e) => {
                let args = vec![e.object, Self::map_condkey(e.condition).unwrap(), e.source];

                let value = env.apply_(handler, args)?;
                let mut dynamic_ref = block_on(env.dynamic.dynamic.write());

                dynamic_ref.resize(frame_stack_len, (0, 0));

                value
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exception() {
        assert!(true)
    }
}
