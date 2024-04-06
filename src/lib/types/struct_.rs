//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu struct type
use crate::{
    core::{
        apply::Core as _,
        exception::{self, Condition, Exception},
        frame::Frame,
        gc::Core as _,
        indirect::IndirectTag,
        lib::Core as _,
        mu::Mu,
        types::{Tag, TagType, Type},
    },
    types::{
        cons::{Cons, Core as _},
        stream::{Core as _, Stream},
        symbol::{Core as _, Symbol},
        vecimage::{TypedVec, VecType, VectorIter},
        vector::{Core as _, Vector},
    },
};

use futures::executor::block_on;

// a struct is a vector with an arbitrary type keyword
pub struct Struct {
    pub stype: Tag,
    pub vector: Tag,
}

impl Struct {
    pub fn to_image(mu: &Mu, tag: Tag) -> Self {
        match tag.type_of() {
            Type::Struct => match tag {
                Tag::Indirect(image) => {
                    let heap_ref = block_on(mu.heap.read());

                    Struct {
                        stype: Tag::from_slice(
                            heap_ref.image_slice(image.image_id() as usize).unwrap(),
                        ),
                        vector: Tag::from_slice(
                            heap_ref.image_slice(image.image_id() as usize + 1).unwrap(),
                        ),
                    }
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn stype(mu: &Mu, tag: Tag) -> Tag {
        Self::to_image(mu, tag).stype
    }

    pub fn vector(mu: &Mu, tag: Tag) -> Tag {
        Self::to_image(mu, tag).vector
    }

    pub fn to_tag(mu: &Mu, stype: Tag, vec: Vec<Tag>) -> Tag {
        match stype.type_of() {
            Type::Keyword => {
                let vector = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu);
                Struct { stype, vector }.evict(mu)
            }
            _ => panic!(),
        }
    }
}

// core
pub trait Core<'a> {
    fn new(_: &Mu, _: &str, _: Vec<Tag>) -> Self;
    fn read(_: &Mu, _: Tag) -> exception::Result<Tag>;
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn evict(&self, _: &Mu) -> Tag;
    fn mark(_: &Mu, _: Tag);
    fn view(_: &Mu, _: Tag) -> Tag;
    fn heap_size(_: &Mu, _: Tag) -> usize;
}

impl<'a> Core<'a> for Struct {
    fn new(mu: &Mu, key: &str, vec: Vec<Tag>) -> Self {
        Struct {
            stype: Symbol::keyword(key),
            vector: TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu),
        }
    }

    fn mark(mu: &Mu, struct_: Tag) {
        let mark = mu.mark_image(struct_).unwrap();

        if !mark {
            Mu::mark(mu, Self::vector(mu, struct_))
        }
    }

    fn view(mu: &Mu, tag: Tag) -> Tag {
        let image = Self::to_image(mu, tag);
        let vec = vec![image.stype, image.vector];

        TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu)
    }

    fn heap_size(mu: &Mu, struct_: Tag) -> usize {
        std::mem::size_of::<Struct>() + Vector::heap_size(mu, Self::vector(mu, struct_))
    }

    fn write(mu: &Mu, tag: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        match tag {
            Tag::Indirect(_) => {
                match mu.write_string("#s(", stream) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }

                match mu.write_stream(Self::to_image(mu, tag).stype, true, stream) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }

                for tag in VectorIter::new(mu, Self::to_image(mu, tag).vector) {
                    match mu.write_string(" ", stream) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }

                    match mu.write_stream(tag, true, stream) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }

                mu.write_string(")", stream)
            }
            _ => panic!(),
        }
    }

    fn read(mu: &Mu, stream: Tag) -> exception::Result<Tag> {
        match Stream::read_char(mu, stream) {
            Ok(Some(ch)) => match ch {
                '(' => {
                    let vec_list = match Cons::read(mu, stream) {
                        Ok(list) => {
                            if list.null_() {
                                return Err(Exception::new(Condition::Type, "read:st", Tag::nil()));
                            }
                            list
                        }
                        Err(_) => {
                            return Err(Exception::new(Condition::Syntax, "read:st", stream));
                        }
                    };

                    let stype = Cons::car(mu, vec_list);
                    match stype.type_of() {
                        Type::Keyword => Ok(Self::to_tag(
                            mu,
                            stype,
                            Cons::iter(mu, Cons::cdr(mu, vec_list))
                                .map(|cons| Cons::car(mu, cons))
                                .collect::<Vec<Tag>>(),
                        )),
                        _ => Err(Exception::new(Condition::Type, "read:st", stype)),
                    }
                }
                _ => Err(Exception::new(Condition::Eof, "read:st", stream)),
            },
            Ok(None) => Err(Exception::new(Condition::Eof, "read:st", stream)),
            Err(e) => Err(e),
        }
    }

    fn evict(&self, mu: &Mu) -> Tag {
        let image: &[[u8; 8]] = &[self.stype.as_slice(), self.vector.as_slice()];
        let mut heap_ref = block_on(mu.heap.write());

        Tag::Indirect(
            IndirectTag::new()
                .with_image_id(heap_ref.alloc(image, None, Type::Struct as u8).unwrap() as u64)
                .with_heap_id(1)
                .with_tag(TagType::Struct),
        )
    }
}

// mu functions
pub trait MuFunction {
    fn lib_struct_type(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_struct_vector(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_make_struct(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Struct {
    fn lib_struct_type(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        fp.value = match mu.fp_argv_check("st-type", &[Type::Struct], fp) {
            Ok(_) => Self::to_image(mu, tag).stype,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_struct_vector(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        fp.value = match mu.fp_argv_check("st-vec", &[Type::Struct], fp) {
            Ok(_) => Self::to_image(mu, tag).vector,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_make_struct(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stype = fp.argv[0];
        let list = fp.argv[1];

        fp.value = match mu.fp_argv_check("struct", &[Type::Keyword, Type::List], fp) {
            Ok(_) => {
                let vec = Cons::iter(mu, list)
                    .map(|cons| Cons::car(mu, cons))
                    .collect::<Vec<Tag>>();
                let vector = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu);

                Struct { stype, vector }.evict(mu)
            }
            Err(e) => return Err(e),
        };

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
