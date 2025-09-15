//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! typed vectors
use crate::{
    core::{
        apply::Apply,
        direct::DirectType,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        gc::{Gc as _, GcContext},
        tag::Tag,
        type_::Type,
    },
    types::{
        char::Char,
        cons::Cons,
        fixnum::Fixnum,
        float::Float,
        vector::{Vector, VTYPEMAP},
    },
    vectors::image::{VecImage, VecImageType, VectorImage},
};

pub trait Gc {
    fn gc_ref_image(_: &mut GcContext, _: Tag) -> VectorImage;
    fn gc_ref(_: &mut GcContext, _: &Env, _: Tag, _: usize) -> Option<Tag>;
    fn ref_type_of(_: &mut GcContext, _: Tag) -> Type;
    fn ref_length(_: &mut GcContext, _: Tag) -> usize;
    fn mark(_: &mut GcContext, _: &Env, _: Tag);
}

impl Gc for Vector {
    fn gc_ref_image(context: &mut GcContext, tag: Tag) -> VectorImage {
        let heap_ref = &context.heap_ref;

        match tag.type_of() {
            Type::Vector => match tag {
                Tag::Indirect(image) => VectorImage {
                    type_: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize).unwrap(),
                    ),
                    length: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize + 1).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn gc_ref(context: &mut GcContext, env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        match vector.type_of() {
            Type::Vector => match vector {
                Tag::Image(_) => panic!(),
                Tag::Direct(_direct) => {
                    let ch: char = vector.data(env).to_le_bytes()[index].into();

                    Some(ch.into())
                }
                Tag::Indirect(_) => VecImageType::gc_ref(context, vector, index),
            },
            _ => panic!(),
        }
    }

    fn ref_type_of(context: &mut GcContext, vector: Tag) -> Type {
        match vector {
            Tag::Image(_) => panic!(),
            Tag::Direct(direct) => match direct.dtype() {
                DirectType::String => Type::Char,
                DirectType::ByteVec => Type::Byte,
                _ => panic!(),
            },
            Tag::Indirect(_) => {
                let image = Self::gc_ref_image(context, vector);

                match VTYPEMAP
                    .iter()
                    .copied()
                    .find(|desc| image.type_.eq_(&desc.0))
                {
                    Some(desc) => desc.1,
                    None => panic!(),
                }
            }
        }
    }

    fn ref_length(context: &mut GcContext, vector: Tag) -> usize {
        match vector {
            Tag::Image(_) => panic!(),
            Tag::Direct(direct) => direct.ext() as usize,
            Tag::Indirect(_) => {
                let image = Self::gc_ref_image(context, vector);
                Fixnum::as_i64(image.length) as usize
            }
        }
    }

    fn mark(context: &mut GcContext, env: &Env, vector: Tag) {
        match vector {
            Tag::Image(_) => panic!(),
            Tag::Direct(_) => (),
            Tag::Indirect(_) => {
                let marked = context.mark_image(vector).unwrap();

                if !marked && Self::ref_type_of(context, vector) == Type::T {
                    for index in 0..Self::ref_length(context, vector) {
                        let value = Self::gc_ref(context, env, vector, index).unwrap();

                        context.mark(env, value)
                    }
                }
            }
        }
    }
}

// env functions
pub trait CoreFn {
    fn mu_type(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_length(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_make_vector(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_svref(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Vector {
    fn mu_make_vector(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:make-vector", &[Type::Keyword, Type::List], fp)?;

        let type_sym = fp.argv[0];
        let list = fp.argv[1];

        fp.value = match Self::to_type(type_sym) {
            Some(vtype) => match vtype {
                Type::Null => Err(Exception::new(
                    env,
                    Condition::Type,
                    "mu:make-vector",
                    type_sym,
                ))?,
                Type::T => {
                    let vec = Cons::list_iter(env, list).collect::<Vec<Tag>>();

                    Vector::from(vec).evict(env)
                }
                Type::Char => {
                    let vec: exception::Result<String> = Cons::list_iter(env, list)
                        .map(|ch| {
                            if ch.type_of() == Type::Char {
                                Ok(Char::as_char(env, ch))
                            } else {
                                Err(Exception::new(env, Condition::Type, "mu:make-vector", ch))?
                            }
                        })
                        .collect();

                    Vector::from(vec?).evict(env)
                }
                Type::Bit => {
                    let mut vec = vec![0; Cons::length(env, list).unwrap().div_ceil(8)];
                    let bvec = &mut vec;

                    for (i, fx) in Cons::list_iter(env, list).enumerate() {
                        if fx.type_of() == Type::Fixnum {
                            let bit = Fixnum::as_i64(fx);
                            if !(0..1).contains(&bit) {
                                Err(Exception::new(env, Condition::Range, "mu:make-vector", fx))?
                            } else {
                                bvec[i / 8] |= (bit as u8) << (7 - i % 8)
                            }
                        } else {
                            Err(Exception::new(env, Condition::Type, "mu:make-vector", fx))?
                        }
                    }

                    Vector::from(vec).evict(env)
                }
                Type::Byte => {
                    let vec: exception::Result<Vec<u8>> = Cons::list_iter(env, list)
                        .map(|fx| {
                            if fx.type_of() == Type::Fixnum {
                                let byte = Fixnum::as_i64(fx);
                                if !(0..=255).contains(&byte) {
                                    Err(Exception::new(
                                        env,
                                        Condition::Range,
                                        "mu:make-vector",
                                        fx,
                                    ))?
                                } else {
                                    Ok(byte as u8)
                                }
                            } else {
                                Err(Exception::new(env, Condition::Type, "mu:make-vector", fx))
                            }
                        })
                        .collect();

                    Vector::from(vec?).evict(env)
                }
                Type::Fixnum => {
                    let vec: exception::Result<Vec<i64>> = Cons::list_iter(env, list)
                        .map(|fx| {
                            if fx.type_of() == Type::Fixnum {
                                Ok(Fixnum::as_i64(fx))
                            } else {
                                Err(Exception::new(env, Condition::Type, "mu:make-vector", fx))?
                            }
                        })
                        .collect();

                    Vector::from(vec?).evict(env)
                }
                Type::Float => {
                    let vec: exception::Result<Vec<f32>> = Cons::list_iter(env, list)
                        .map(|fl| {
                            if fl.type_of() == Type::Float {
                                Ok(Float::as_f32(env, fl))
                            } else {
                                Err(Exception::new(env, Condition::Type, "mu:make-vector", fl))?
                            }
                        })
                        .collect();

                    Vector::from(vec?).evict(env)
                }
                _ => Err(Exception::new(
                    env,
                    Condition::Type,
                    "mu:make-vector",
                    type_sym,
                ))?,
            },
            None => Err(Exception::new(
                env,
                Condition::Type,
                "mu:make-vector",
                type_sym,
            ))?,
        };

        Ok(())
    }

    fn mu_svref(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:svref", &[Type::Vector, Type::Fixnum], fp)?;

        let vector = fp.argv[0];
        let index = fp.argv[1];

        let nth = Fixnum::as_i64(index);

        if nth < 0 || nth as usize >= Self::length(env, vector) {
            Err(Exception::new(env, Condition::Range, "mu:svref", index))?
        }

        fp.value = Self::ref_(env, vector, nth as usize).unwrap();

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

        fp.value = Fixnum::with_or_panic(Self::length(env, vector));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
