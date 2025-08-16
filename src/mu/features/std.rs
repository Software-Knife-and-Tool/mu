//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! std interface
use {
    crate::{
        core::{
            apply::Apply as _,
            core::CoreFunctionDef,
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            types::{Tag, Type},
        },
        features::feature::Feature,
        types::{cons::Cons, fixnum::Fixnum, float::Float, vector::Vector},
    },
    futures_locks::RwLock,
    std::collections::HashMap,
};

lazy_static! {
    pub static ref STD_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref STD_FUNCTIONS: Vec<CoreFunctionDef> = vec![
        ("command", 2, Feature::std_command),
        ("env", 0, Feature::std_env),
        ("exit", 1, Feature::std_exit),
        ("sleep", 1, Feature::std_sleep),
    ];
}

pub trait Std {
    fn feature() -> Feature;
}

impl Std for Feature {
    fn feature() -> Feature {
        Feature {
            functions: Some(&STD_FUNCTIONS),
            symbols: Some(&STD_SYMBOLS),
            namespace: "mu/std".into(),
        }
    }
}

pub trait CoreFunction {
    fn std_command(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn std_env(_: &Env, fp: &mut Frame) -> exception::Result<()>;
    fn std_exit(_: &Env, fp: &mut Frame) -> exception::Result<()>;
    fn std_sleep(_: &Env, fp: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn std_command(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("std:command", &[Type::String, Type::List], fp)?;

        let command = fp.argv[0];
        let args = fp.argv[1];
        let mut argv = vec![];

        for cons in Cons::iter(env, args) {
            let string = Cons::car(env, cons);

            match string.type_of() {
                Type::Vector if Vector::type_of(env, string) == Type::Char => {
                    let str = Vector::as_string(env, string);
                    argv.push(str)
                }
                _ => return Err(Exception::new(env, Condition::Type, "std:command", string)),
            }
        }

        let status = std::process::Command::new(Vector::as_string(env, command))
            .args(argv)
            .status();

        fp.value = match status {
            Err(_) => return Err(Exception::new(env, Condition::Open, "std:command", command)),
            Ok(exit_status) => Fixnum::with_or_panic(exit_status.code().unwrap() as usize),
        };

        Ok(())
    }

    fn std_exit(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("std:exit", &[Type::Fixnum], fp)?;

        let rc = fp.argv[0];

        std::process::exit(Fixnum::as_i64(rc) as i32);
    }

    fn std_env(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Cons::list(
            env,
            &std::env::vars()
                .map(|(key, value)| {
                    Cons::cons(
                        env,
                        Vector::from(key).evict(env),
                        Vector::from(value).evict(env),
                    )
                })
                .collect::<Vec<Tag>>(),
        );

        Ok(())
    }

    fn std_sleep(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("std:sleep", &[Type::Float], fp)?;

        fp.value = fp.argv[0];

        std::thread::sleep(std::time::Duration::from_micros(
            (1e6 * Float::as_f32(env, fp.value)) as u64,
        ));

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
