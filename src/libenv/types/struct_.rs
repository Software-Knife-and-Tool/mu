//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env struct type
use crate::{
    core::{
        apply::Core as _,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        gc::Core as _,
        indirect::IndirectTag,
        types::{Tag, TagType, Type},
    },
    streams::write::Core as _,
    types::{
        cons::{Cons, Core as _},
        stream::{Core as _, Stream},
        symbol::{Core as _, Symbol},
        vector::{TypedVec, VecType, VectorIter},
        vectors::{Core as _, Vector},
    },
};

use futures::executor::block_on;

// a struct is a vector with an arbitrary type keyword
pub struct Struct {
    pub stype: Tag,
    pub vector: Tag,
}

impl Struct {
    pub fn to_image(env: &Env, tag: Tag) -> Self {
        match tag.type_of() {
            Type::Struct => match tag {
                Tag::Indirect(image) => {
                    let heap_ref = block_on(env.heap.read());

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

    pub fn stype(env: &Env, tag: Tag) -> Tag {
        Self::to_image(env, tag).stype
    }

    pub fn vector(env: &Env, tag: Tag) -> Tag {
        Self::to_image(env, tag).vector
    }

    pub fn to_tag(env: &Env, stype: Tag, vec: Vec<Tag>) -> Tag {
        match stype.type_of() {
            Type::Keyword => {
                let vector = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(env);
                Struct { stype, vector }.evict(env)
            }
            _ => panic!(),
        }
    }
}

// core
pub trait Core<'a> {
    fn new(_: &Env, _: &str, _: Vec<Tag>) -> Self;
    fn read(_: &Env, _: Tag) -> exception::Result<Tag>;
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn evict(&self, _: &Env) -> Tag;
    fn mark(_: &Env, _: Tag);
    fn view(_: &Env, _: Tag) -> Tag;
    fn heap_size(_: &Env, _: Tag) -> usize;
}

impl<'a> Core<'a> for Struct {
    fn new(env: &Env, key: &str, vec: Vec<Tag>) -> Self {
        Struct {
            stype: Symbol::keyword(key),
            vector: TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(env),
        }
    }

    fn mark(env: &Env, struct_: Tag) {
        let mark = env.mark_image(struct_).unwrap();

        if !mark {
            Env::mark(env, Self::vector(env, struct_))
        }
    }

    fn view(env: &Env, tag: Tag) -> Tag {
        let image = Self::to_image(env, tag);
        let vec = vec![image.stype, image.vector];

        TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
    }

    fn heap_size(env: &Env, struct_: Tag) -> usize {
        std::mem::size_of::<Struct>() + Vector::heap_size(env, Self::vector(env, struct_))
    }

    fn write(env: &Env, tag: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        match tag {
            Tag::Indirect(_) => {
                match env.write_string("#s(", stream) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }

                match env.write_stream(Self::to_image(env, tag).stype, true, stream) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }

                for tag in VectorIter::new(env, Self::to_image(env, tag).vector) {
                    match env.write_string(" ", stream) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }

                    match env.write_stream(tag, true, stream) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }

                env.write_string(")", stream)
            }
            _ => panic!(),
        }
    }

    fn read(env: &Env, stream: Tag) -> exception::Result<Tag> {
        match Stream::read_char(env, stream) {
            Ok(Some(ch)) => match ch {
                '(' => {
                    let vec_list = match Cons::read(env, stream) {
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

                    let stype = Cons::car(env, vec_list);
                    match stype.type_of() {
                        Type::Keyword => Ok(Self::to_tag(
                            env,
                            stype,
                            Cons::iter(env, Cons::cdr(env, vec_list))
                                .map(|cons| Cons::car(env, cons))
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

    fn evict(&self, env: &Env) -> Tag {
        let image: &[[u8; 8]] = &[self.stype.as_slice(), self.vector.as_slice()];
        let mut heap_ref = block_on(env.heap.write());

        Tag::Indirect(
            IndirectTag::new()
                .with_image_id(heap_ref.alloc(image, None, Type::Struct as u8).unwrap() as u64)
                .with_heap_id(1)
                .with_tag(TagType::Struct),
        )
    }
}

// env functions
pub trait LibFunction {
    fn lib_struct_type(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_struct_vector(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_make_struct(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Struct {
    fn lib_struct_type(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        fp.value = match env.fp_argv_check("st-type", &[Type::Struct], fp) {
            Ok(_) => Self::to_image(env, tag).stype,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_struct_vector(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        fp.value = match env.fp_argv_check("st-vec", &[Type::Struct], fp) {
            Ok(_) => Self::to_image(env, tag).vector,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_make_struct(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let stype = fp.argv[0];
        let list = fp.argv[1];

        fp.value = match env.fp_argv_check("struct", &[Type::Keyword, Type::List], fp) {
            Ok(_) => {
                let vec = Cons::iter(env, list)
                    .map(|cons| Cons::car(env, cons))
                    .collect::<Vec<Tag>>();
                let vector = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(env);

                Struct { stype, vector }.evict(env)
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
