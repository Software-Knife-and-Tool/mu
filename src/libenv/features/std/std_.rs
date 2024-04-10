//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! std interface
use crate::{
    core::{
        apply::Core as _,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        lib::LibFnDef,
        types::Type,
    },
    features::Feature,
    types::{
        cons::{Cons, Core as _},
        fixnum::Fixnum,
        vector::{Core as _, Vector},
    },
};

// env function dispatch table
lazy_static! {
    static ref STD_SYMBOLS: Vec<LibFnDef> = vec![
        ("command", 2, Std::std_command),
        ("env", 0, Std::std_env),
        ("exit", 1, Std::std_exit),
    ];
}

pub struct Std {}

pub trait Core {
    fn make_feature(_: &Env) -> Feature;
}

impl Core for Std {
    fn make_feature(_: &Env) -> Feature {
        Feature {
            symbols: STD_SYMBOLS.to_vec(),
            namespace: "std".to_string(),
        }
    }
}

pub trait LibFunction {
    fn std_command(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn std_env(_: &Env, fp: &mut Frame) -> exception::Result<()>;
    fn std_exit(_: &Env, fp: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Std {
    fn std_command(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let command = fp.argv[0];
        let args = fp.argv[1];

        fp.value = match env.fp_argv_check("command", &[Type::String, Type::List], fp) {
            Ok(_) => {
                let mut argv = vec![];

                for cons in Cons::iter(env, args) {
                    let string = Cons::car(env, cons);

                    match string.type_of() {
                        Type::Vector if Vector::type_of(env, string) == Type::Char => {
                            let str = Vector::as_string(env, string);
                            argv.push(str)
                        }
                        _ => return Err(Exception::new(Condition::Type, "command", string)),
                    }
                }

                let status = std::process::Command::new(Vector::as_string(env, command))
                    .args(argv)
                    .status();

                match status {
                    Err(_) => return Err(Exception::new(Condition::Open, "command", command)),
                    Ok(exit_status) => Fixnum::as_tag(exit_status.code().unwrap() as i64),
                }
            }

            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn std_exit(_: &Env, fp: &mut Frame) -> exception::Result<()> {
        let rc = fp.argv[0];

        match rc.type_of() {
            Type::Fixnum => std::process::exit(Fixnum::as_i64(rc) as i32),
            _ => Err(Exception::new(Condition::Type, "exit", rc)),
        }
    }

    fn std_env(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = {
            let mut vars = vec![];

            for (key, value) in std::env::vars() {
                vars.push(
                    Cons::new(
                        Vector::from_string(&key).evict(env),
                        Vector::from_string(&value).evict(env),
                    )
                    .evict(env),
                )
            }

            Cons::vlist(env, &vars)
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}