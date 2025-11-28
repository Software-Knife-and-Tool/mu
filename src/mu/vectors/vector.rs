//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! typed vectors
use crate::{
    core::{
        apply::Apply,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        tag::Tag,
        type_::Type,
    },
    types::{char::Char, cons::Cons, fixnum::Fixnum, float::Float, vector::Vector},
};

// env functions
pub trait CoreFn {
    fn mu_type(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_length(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_make_vector(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_svref(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Vector {
    #[allow(clippy::too_many_lines)]
    fn mu_make_vector(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:make-vector", &[Type::Keyword, Type::List], fp)?;

        let type_sym = fp.argv[0];
        let list = fp.argv[1];

        fp.value = match Self::to_type(type_sym) {
            Some(vtype) => match vtype {
                Type::T => {
                    let vec = Cons::list_iter(env, list).collect::<Vec<Tag>>();

                    Vector::from(vec).with_heap(env)
                }
                Type::Char => {
                    let vec: exception::Result<String> = Cons::list_iter(env, list)
                        .map(|ch| {
                            if ch.type_of() == Type::Char {
                                Ok(Char::as_char(env, ch))
                            } else {
                                Err(Exception::err(env, ch, Condition::Type, "mu:make-vector"))?
                            }
                        })
                        .collect();

                    Vector::from(vec?).with_heap(env)
                }
                Type::Bit => {
                    let mut vec = vec![0; Cons::length(env, list).unwrap().div_ceil(8)];
                    let bvec = &mut vec;

                    for (i, fx) in Cons::list_iter(env, list).enumerate() {
                        if fx.type_of() == Type::Fixnum {
                            let bit = Fixnum::as_i64(fx);
                            if (0..1).contains(&bit) {
                                bvec[i / 8] |= u8::try_from(bit).unwrap() << (7 - i % 8);
                            } else {
                                Err(Exception::err(env, fx, Condition::Range, "mu:make-vector"))?;
                            }
                        } else {
                            Err(Exception::err(env, fx, Condition::Type, "mu:make-vector"))?;
                        }
                    }

                    Vector::from(vec).with_heap(env)
                }
                Type::Byte => {
                    let vec: exception::Result<Vec<u8>> = Cons::list_iter(env, list)
                        .map(|fx| {
                            if fx.type_of() == Type::Fixnum {
                                let byte = Fixnum::as_i64(fx);
                                if (0..=255).contains(&byte) {
                                    Ok(u8::try_from(byte).unwrap())
                                } else {
                                    Err(Exception::err(
                                        env,
                                        fx,
                                        Condition::Range,
                                        "mu:make-vector",
                                    ))?
                                }
                            } else {
                                Err(Exception::err(env, fx, Condition::Type, "mu:make-vector"))
                            }
                        })
                        .collect();

                    Vector::from(vec?).with_heap(env)
                }
                Type::Fixnum => {
                    let vec: exception::Result<Vec<i64>> = Cons::list_iter(env, list)
                        .map(|fx| {
                            if fx.type_of() == Type::Fixnum {
                                Ok(Fixnum::as_i64(fx))
                            } else {
                                Err(Exception::err(env, fx, Condition::Type, "mu:make-vector"))?
                            }
                        })
                        .collect();

                    Vector::from(vec?).with_heap(env)
                }
                Type::Float => {
                    let vec: exception::Result<Vec<f32>> = Cons::list_iter(env, list)
                        .map(|fl| {
                            if fl.type_of() == Type::Float {
                                Ok(Float::as_f32(env, fl))
                            } else {
                                Err(Exception::err(env, fl, Condition::Type, "mu:make-vector"))?
                            }
                        })
                        .collect();

                    Vector::from(vec?).with_heap(env)
                }
                _ => Err(Exception::err(
                    env,
                    type_sym,
                    Condition::Type,
                    "mu:make-vector",
                ))?,
            },
            None => Err(Exception::err(
                env,
                type_sym,
                Condition::Type,
                "mu:make-vector",
            ))?,
        };

        Ok(())
    }

    fn mu_svref(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:svref", &[Type::Vector, Type::Fixnum], fp)?;

        let vector = fp.argv[0];
        let index = fp.argv[1];

        let nth = Fixnum::as_i64(index);

        if nth < 0 || usize::try_from(nth).unwrap() >= Self::length(env, vector) {
            Err(Exception::err(env, index, Condition::Range, "mu:svref"))?;
        }

        fp.value = Self::ref_(env, vector, usize::try_from(nth).unwrap()).unwrap();

        Ok(())
    }

    fn mu_type(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:vector-type", &[Type::Vector], fp)?;

        let vector = fp.argv[0];

        fp.value = Self::type_of(env, vector).map_typesym();

        Ok(())
    }

    fn mu_length(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:vector-length", &[Type::Vector], fp)?;

        let vector = fp.argv[0];

        fp.value = Fixnum::with_usize_or_panic(Self::length(env, vector));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn vector_test() {
        assert!(true);
    }
}
