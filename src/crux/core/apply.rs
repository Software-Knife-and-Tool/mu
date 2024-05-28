//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env functions
use crate::{
    core::{
        env::{Core as _, Env},
        exception::{self, Condition, Exception},
        frame::Frame,
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, Core as _},
        fixnum::Fixnum,
        vector::Vector,
    },
};

pub trait Core {
    fn fp_argv_check(&self, _: &str, _: &[Type], _: &Frame) -> exception::Result<()>;
}

impl Core for Env {
    fn fp_argv_check(&self, source: &str, types: &[Type], fp: &Frame) -> exception::Result<()> {
        for (index, arg_type) in types.iter().enumerate() {
            let fp_arg = fp.argv[index];
            let fp_arg_type = fp_arg.type_of();

            match *arg_type {
                Type::Byte => match fp_arg_type {
                    Type::Fixnum => {
                        let n = Fixnum::as_i64(fp_arg);

                        if !(0..=255).contains(&n) {
                            return Err(Exception::new(self, Condition::Type, source, fp_arg));
                        }
                    }
                    _ => return Err(Exception::new(self, Condition::Type, source, fp_arg)),
                },
                Type::List => match fp_arg_type {
                    Type::Cons | Type::Null => (),
                    _ => return Err(Exception::new(self, Condition::Type, source, fp_arg)),
                },
                Type::String => match fp_arg_type {
                    Type::Vector => {
                        if Vector::type_of(self, fp.argv[index]) != Type::Char {
                            return Err(Exception::new(self, Condition::Type, source, fp_arg));
                        }
                    }
                    _ => return Err(Exception::new(self, Condition::Type, source, fp_arg)),
                },
                Type::T => (),
                _ => {
                    if fp_arg_type != *arg_type {
                        return Err(Exception::new(self, Condition::Type, source, fp_arg));
                    }
                }
            }
        }

        Ok(())
    }
}

pub trait CoreFunction {
    fn crux_apply(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_eval(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_fix(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn crux_eval(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = env.eval(fp.argv[0])?;

        Ok(())
    }

    fn crux_apply(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        env.fp_argv_check("crux:apply", &[Type::Function, Type::List], fp)?;
        fp.value = Frame {
            func,
            argv: Cons::iter(env, args)
                .map(|cons| Cons::car(env, cons))
                .collect::<Vec<Tag>>(),
            value: Tag::nil(),
        }
        .apply(env, func)?;

        Ok(())
    }

    fn crux_fix(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let mut value = fp.argv[1];

        env.fp_argv_check("crux:fix", &[Type::Function, Type::T], fp)?;

        fp.value = loop {
            let last_value = value;
            let argv = vec![value];

            value = Frame { func, argv, value }.apply(env, func)?;
            if last_value.eq_(&value) {
                break value;
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn crux_functions() {
        assert_eq!(2 + 2, 4);
    }
}
