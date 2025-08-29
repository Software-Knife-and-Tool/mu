//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// Apply trait
use crate::{
    core::{
        compile::Compile,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        tag::Tag,
        type_::Type,
    },
    types::{cons::Cons, fixnum::Fixnum, symbol::Symbol, vector::Vector},
};

pub trait Apply {
    fn argv_check(&self, _: &str, _: &[Type], _: &Frame) -> exception::Result<()>;
    fn apply(&self, _: Tag, _: Tag) -> exception::Result<Tag>;
    fn apply_(&self, _: Tag, _: Vec<Tag>) -> exception::Result<Tag>;
    fn eval(&self, _: Tag) -> exception::Result<Tag>;
}

impl Apply for Env {
    fn argv_check(&self, source: &str, types: &[Type], fp: &Frame) -> exception::Result<()> {
        for (index, arg_type) in types.iter().enumerate() {
            let fp_arg = fp.argv[index];
            let fp_arg_type = fp_arg.type_of();

            match *arg_type {
                Type::Byte => match fp_arg_type {
                    Type::Fixnum => {
                        let n = Fixnum::as_i64(fp_arg);

                        if !(0..=255).contains(&n) {
                            Err(Exception::new(self, Condition::Type, source, fp_arg))?
                        }
                    }
                    _ => Err(Exception::new(self, Condition::Type, source, fp_arg))?,
                },
                Type::List => match fp_arg_type {
                    Type::Cons | Type::Null => (),
                    _ => Err(Exception::new(self, Condition::Type, source, fp_arg))?,
                },
                Type::String => match fp_arg_type {
                    Type::Vector => {
                        if Vector::type_of(self, fp.argv[index]) != Type::Char {
                            Err(Exception::new(self, Condition::Type, source, fp_arg))?;
                        }
                    }
                    _ => Err(Exception::new(self, Condition::Type, source, fp_arg))?,
                },
                Type::T => (),
                _ => {
                    if fp_arg_type != *arg_type {
                        Err(Exception::new(self, Condition::Type, source, fp_arg))?
                    }
                }
            }
        }

        Ok(())
    }

    fn apply_(&self, func: Tag, argv: Vec<Tag>) -> exception::Result<Tag> {
        Frame {
            func,
            argv,
            value: Tag::nil(),
        }
        .apply(self, func)
    }

    fn apply(&self, func: Tag, args: Tag) -> exception::Result<Tag> {
        let eval_results: exception::Result<Vec<Tag>> = Cons::iter(self, args)
            .map(|cons| self.eval(Cons::car(self, cons)))
            .collect();

        self.apply_(func, eval_results?)
    }

    fn eval(&self, expr: Tag) -> exception::Result<Tag> {
        if self.is_quoted(&expr) {
            return Ok(self.unquote(&expr));
        }

        match expr.type_of() {
            Type::Cons => {
                let func = Cons::car(self, expr);
                let args = Cons::cdr(self, expr);

                match func.type_of() {
                    Type::Symbol => {
                        if Symbol::is_bound(self, func) {
                            let fn_ = Symbol::value(self, func);
                            match fn_.type_of() {
                                Type::Function => self.apply(fn_, args),
                                _ => Err(Exception::new(self, Condition::Type, "mu:eval", func))?,
                            }
                        } else {
                            Err(Exception::new(self, Condition::Unbound, "mu:eval", func))?
                        }
                    }
                    Type::Function => self.apply(func, args),
                    _ => Err(Exception::new(self, Condition::Type, "mu:eval", func))?,
                }
            }
            Type::Symbol => {
                if Symbol::is_bound(self, expr) {
                    Ok(Symbol::value(self, expr))
                } else {
                    Err(Exception::new(self, Condition::Unbound, "mu:eval", expr))?
                }
            }
            _ => Ok(expr),
        }
    }
}

pub trait CoreFunction {
    fn mu_apply(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_eval(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fix(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn mu_eval(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = env.eval(fp.argv[0])?;

        Ok(())
    }

    fn mu_apply(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:apply", &[Type::Function, Type::List], fp)?;

        let func = fp.argv[0];
        let args = fp.argv[1];

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

    fn mu_fix(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:fix", &[Type::Function, Type::T], fp)?;

        let func = fp.argv[0];
        let mut value = fp.argv[1];

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
    fn mu_functions() {
        assert!(true)
    }
}
