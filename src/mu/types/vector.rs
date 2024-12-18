//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env vector type
use {
    crate::{
        core::{
            apply::Core as _,
            direct::DirectType,
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            gc::Gc,
            types::{Tag, Type},
        },
        types::{
            char::Char,
            cons::{Cons, Core as _},
            fixnum::{Core as _, Fixnum},
            float::Float,
            symbol::{Core as _, Symbol},
        },
        vectors::{
            core::Core as _,
            image::{VecImage, VecImageType, VectorImage, VectorImageType},
        },
    },
    futures::executor::block_on,
    std::str,
};

lazy_static! {
    pub static ref VTYPEMAP: Vec<(Tag, Type)> = vec![
        (Symbol::keyword("bit"), Type::Bit),
        (Symbol::keyword("byte"), Type::Byte),
        (Symbol::keyword("char"), Type::Char),
        (Symbol::keyword("fixnum"), Type::Fixnum),
        (Symbol::keyword("float"), Type::Float),
        (Symbol::keyword("t"), Type::T),
    ];
}

#[derive(Clone)]
pub enum Vector {
    Direct(Tag),
    Indirect(VectorImage, VectorImageType),
}

impl Vector {
    const IMAGE_LEN: usize = 2; // heap words in image

    pub fn to_type(keyword: Tag) -> Option<Type> {
        VTYPEMAP
            .iter()
            .copied()
            .find(|tab| keyword.eq_(&tab.0))
            .map(|tab| tab.1)
    }

    pub fn type_of(env: &Env, vector: Tag) -> Type {
        match vector {
            Tag::Direct(direct) => match direct.dtype() {
                DirectType::String => Type::Char,
                DirectType::ByteVec => Type::Byte,
                _ => panic!(),
            },
            Tag::Indirect(_) => {
                let image = Self::to_image(env, vector);

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

    pub fn length(env: &Env, vector: Tag) -> usize {
        match vector {
            Tag::Direct(direct) => direct.ext() as usize,
            Tag::Indirect(_) => {
                let image = Self::to_image(env, vector);
                Fixnum::as_i64(image.length) as usize
            }
        }
    }

    pub fn ref_type_of(gc: &mut Gc, vector: Tag) -> Type {
        match vector {
            Tag::Direct(direct) => match direct.dtype() {
                DirectType::String => Type::Char,
                DirectType::ByteVec => Type::Byte,
                _ => panic!(),
            },
            Tag::Indirect(_) => {
                let image = Self::gc_ref_image(&mut gc.lock, vector);

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

    pub fn ref_length(gc: &mut Gc, vector: Tag) -> usize {
        match vector {
            Tag::Direct(direct) => direct.ext() as usize,
            Tag::Indirect(_) => {
                let image = Self::gc_ref_image(&mut gc.lock, vector);
                Fixnum::as_i64(image.length) as usize
            }
        }
    }

    pub fn mark(gc: &mut Gc, env: &Env, vector: Tag) {
        match vector {
            Tag::Direct(_) => (),
            Tag::Indirect(_) => {
                let marked = gc.mark_image(vector).unwrap();

                if !marked && Self::ref_type_of(gc, vector) == Type::T {
                    for index in 0..Self::ref_length(gc, vector) {
                        let value = Self::gc_ref(gc, env, vector, index).unwrap();

                        gc.mark(env, value)
                    }
                }
            }
        }
    }
}

pub trait Core<'a> {
    fn as_string(_: &Env, _: Tag) -> String;
    fn gc_ref(_: &mut Gc, _: &Env, _: Tag, _: usize) -> Option<Tag>;
    fn iter(_: &Env, _: Tag) -> VectorIter;
    fn ref_(_: &Env, _: Tag, _: usize) -> Option<Tag>;
    fn view(_: &Env, _: Tag) -> Tag;
}

impl Core<'_> for Vector {
    fn view(env: &Env, vector: Tag) -> Tag {
        let vec = vec![
            Fixnum::with_or_panic(Self::length(env, vector)),
            match Tag::type_key(Self::type_of(env, vector)) {
                Some(key) => key,
                None => panic!(),
            },
        ];

        Vector::from(vec).evict(env)
    }

    fn as_string(env: &Env, tag: Tag) -> String {
        match tag.type_of() {
            Type::Vector => match tag {
                Tag::Direct(dir) => match dir.dtype() {
                    DirectType::String => str::from_utf8(&dir.data().to_le_bytes()).unwrap()
                        [..dir.ext() as usize]
                        .into(),
                    _ => panic!(),
                },
                Tag::Indirect(image) => {
                    let heap_ref = block_on(env.heap.read());
                    let vec: VectorImage = Self::to_image(env, tag);

                    str::from_utf8(
                        heap_ref
                            .image_data_slice(
                                image.image_id() as usize + Self::IMAGE_LEN,
                                0,
                                Fixnum::as_i64(vec.length) as usize,
                            )
                            .unwrap(),
                    )
                    .unwrap()
                    .into()
                }
            },
            _ => panic!(),
        }
    }

    fn gc_ref(gc: &mut Gc, env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        match vector.type_of() {
            Type::Vector => match vector {
                Tag::Direct(_direct) => {
                    let ch: char = vector.data(env).to_le_bytes()[index].into();

                    Some(ch.into())
                }
                Tag::Indirect(_) => VecImageType::gc_ref(gc, vector, index),
            },
            _ => panic!(),
        }
    }

    fn iter(env: &Env, vec: Tag) -> VectorIter {
        VectorIter { env, vec, index: 0 }
    }

    fn ref_(env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        match vector.type_of() {
            Type::Vector => match vector {
                Tag::Direct(direct) => match direct.dtype() {
                    DirectType::String => {
                        let ch: char = vector.data(env).to_le_bytes()[index].into();

                        Some(ch.into())
                    }
                    DirectType::ByteVec => {
                        let byte: u8 = vector.data(env).to_le_bytes()[index];

                        Some(byte.into())
                    }
                    _ => panic!(),
                },
                Tag::Indirect(_) => VecImageType::ref_(env, vector, index),
            },
            _ => {
                panic!()
            }
        }
    }
}

// env functions
pub trait CoreFunction {
    fn mu_type(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_length(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_make_vector(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_svref(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Vector {
    fn mu_make_vector(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let type_sym = fp.argv[0];
        let list = fp.argv[1];

        env.fp_argv_check("mu:make-vector", &[Type::Keyword, Type::List], fp)?;

        fp.value = match Self::to_type(type_sym) {
            Some(vtype) => match vtype {
                Type::Null => {
                    return Err(Exception::new(
                        env,
                        Condition::Type,
                        "mu:make-vector",
                        type_sym,
                    ))
                }
                Type::T => {
                    let vec = Cons::iter(env, list)
                        .map(|cons| Cons::car(env, cons))
                        .collect::<Vec<Tag>>();

                    Vector::from(vec).evict(env)
                }
                Type::Char => {
                    let vec: exception::Result<String> = Cons::iter(env, list)
                        .map(|cons| {
                            let ch = Cons::car(env, cons);
                            if ch.type_of() == Type::Char {
                                Ok(Char::as_char(env, ch))
                            } else {
                                Err(Exception::new(env, Condition::Type, "mu:make-vector", ch))
                            }
                        })
                        .collect();

                    Vector::from(vec?).evict(env)
                }
                Type::Bit => {
                    let mut vec = vec![0; (Cons::length(env, list).unwrap() + 7) / 8];
                    let bvec = &mut vec;

                    for (i, cons) in Cons::iter(env, list).enumerate() {
                        let fx = Cons::car(env, cons);
                        if fx.type_of() == Type::Fixnum {
                            let bit = Fixnum::as_i64(fx);
                            if !(0..1).contains(&bit) {
                                return Err(Exception::new(
                                    env,
                                    Condition::Range,
                                    "mu:make-vector",
                                    fx,
                                ));
                            } else {
                                bvec[i / 8] |= (bit as u8) << (7 - i % 8)
                            }
                        } else {
                            return Err(Exception::new(env, Condition::Type, "mu:make-vector", fx));
                        }
                    }

                    Vector::from(vec).evict(env)
                }
                Type::Byte => {
                    let vec: exception::Result<Vec<u8>> = Cons::iter(env, list)
                        .map(|cons| {
                            let fx = Cons::car(env, cons);
                            if fx.type_of() == Type::Fixnum {
                                let byte = Fixnum::as_i64(fx);
                                if !(0..255).contains(&byte) {
                                    Err(Exception::new(env, Condition::Range, "mu:make-vector", fx))
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
                    let vec: exception::Result<Vec<i64>> = Cons::iter(env, list)
                        .map(|cons| {
                            let fx = Cons::car(env, cons);
                            if fx.type_of() == Type::Fixnum {
                                Ok(Fixnum::as_i64(fx))
                            } else {
                                Err(Exception::new(env, Condition::Type, "mu:make-vector", fx))
                            }
                        })
                        .collect();

                    Vector::from(vec?).evict(env)
                }
                Type::Float => {
                    let vec: exception::Result<Vec<f32>> = Cons::iter(env, list)
                        .map(|cons| {
                            let fl = Cons::car(env, cons);
                            if fl.type_of() == Type::Float {
                                Ok(Float::as_f32(env, fl))
                            } else {
                                Err(Exception::new(env, Condition::Type, "mu:make-vector", fl))
                            }
                        })
                        .collect();

                    Vector::from(vec?).evict(env)
                }
                _ => {
                    return Err(Exception::new(
                        env,
                        Condition::Type,
                        "mu:make-vector",
                        type_sym,
                    ));
                }
            },
            None => {
                return Err(Exception::new(
                    env,
                    Condition::Type,
                    "mu:make-vector",
                    type_sym,
                ));
            }
        };

        Ok(())
    }

    fn mu_svref(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vector = fp.argv[0];
        let index = fp.argv[1];

        env.fp_argv_check("mu:svref", &[Type::Vector, Type::Fixnum], fp)?;

        let nth = Fixnum::as_i64(index);

        if nth < 0 || nth as usize >= Self::length(env, vector) {
            return Err(Exception::new(env, Condition::Range, "mu:svref", index));
        }

        fp.value = match Self::ref_(env, vector, nth as usize) {
            Some(nth) => nth,
            None => panic!(),
        };

        Ok(())
    }

    fn mu_type(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vector = fp.argv[0];

        env.fp_argv_check("mu:vector-type", &[Type::Vector], fp)?;
        fp.value = match Tag::type_key(Self::type_of(env, vector)) {
            Some(key) => key,
            None => panic!(),
        };

        Ok(())
    }

    fn mu_length(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vector = fp.argv[0];

        env.fp_argv_check("mu:vector-length", &[Type::Vector], fp)?;
        fp.value = Fixnum::with_or_panic(Self::length(env, vector));

        Ok(())
    }
}

// iterator
pub struct VectorIter<'a> {
    env: &'a Env,
    pub vec: Tag,
    pub index: usize,
}

impl<'a> VectorIter<'a> {
    pub fn new(env: &'a Env, vec: Tag) -> Self {
        Self { env, vec, index: 0 }
    }
}

impl Iterator for VectorIter<'_> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= Vector::length(self.env, self.vec) {
            None
        } else {
            let el = Vector::ref_(self.env, self.vec, self.index).unwrap();
            self.index += 1;

            Some(el)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
