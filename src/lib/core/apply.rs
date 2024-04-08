//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu functions
use crate::{
    core::{
        exception::{self, Condition, Exception},
        frame::Frame,
        mu::{Core as _, Mu},
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

impl Core for Mu {
    fn fp_argv_check(&self, fn_name: &str, types: &[Type], fp: &Frame) -> exception::Result<()> {
        for (index, arg_type) in types.iter().enumerate() {
            let fp_arg = fp.argv[index];
            let fp_arg_type = fp_arg.type_of();

            match *arg_type {
                Type::Byte => match fp_arg_type {
                    Type::Fixnum => {
                        let n = Fixnum::as_i64(fp_arg);

                        if !(0..=255).contains(&n) {
                            return Err(Exception::new(Condition::Type, fn_name, fp_arg));
                        }
                    }
                    _ => return Err(Exception::new(Condition::Type, fn_name, fp_arg)),
                },
                Type::List => match fp_arg_type {
                    Type::Cons | Type::Null => (),
                    _ => return Err(Exception::new(Condition::Type, fn_name, fp_arg)),
                },
                Type::String => match fp_arg_type {
                    Type::Vector => {
                        if Vector::type_of(self, fp.argv[index]) != Type::Char {
                            return Err(Exception::new(Condition::Type, fn_name, fp_arg));
                        }
                    }
                    _ => return Err(Exception::new(Condition::Type, fn_name, fp_arg)),
                },
                Type::T => (),
                _ => {
                    if fp_arg_type != *arg_type {
                        return Err(Exception::new(Condition::Type, fn_name, fp_arg));
                    }
                }
            }
        }

        Ok(())
    }
}

pub trait MuFunction {
    fn lib_apply(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_eval(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_fix(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Mu {
    fn lib_eval(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match mu.eval(fp.argv[0]) {
            Ok(tag) => tag,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_apply(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        fp.value = match mu.fp_argv_check("apply", &[Type::Function, Type::List], fp) {
            Ok(_) => {
                match (Frame {
                    func,
                    argv: Cons::iter(mu, args)
                        .map(|cons| Cons::car(mu, cons))
                        .collect::<Vec<Tag>>(),
                    value: Tag::nil(),
                })
                .apply(mu, func)
                {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_fix(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];

        fp.value = fp.argv[1];

        match func.type_of() {
            Type::Function => {
                loop {
                    let value = Tag::nil();
                    let argv = vec![fp.value];
                    let result = Frame { func, argv, value }.apply(mu, func);

                    fp.value = match result {
                        Ok(value) => {
                            if value.eq_(&fp.value) {
                                break;
                            }

                            value
                        }
                        Err(e) => return Err(e),
                    };
                }

                Ok(())
            }
            _ => Err(Exception::new(Condition::Type, "fix", func)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lib_functions() {
        assert_eq!(2 + 2, 4);
    }
}
