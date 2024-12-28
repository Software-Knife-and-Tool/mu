//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env struct type
use crate::{
    core::{
        apply::Apply as _,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        gc::{Gc, HeapGcRef},
        indirect::IndirectTag,
        types::{Tag, TagType, Type},
    },
    streams::write::Write as _,
    types::{cons::Cons, stream::Read, symbol::Symbol, vector::Vector},
};

use futures::executor::block_on;

// a struct is a vector with an arbitrary type keyword
#[derive(Copy, Clone)]
pub struct Struct {
    pub stype: Tag,
    pub vector: Tag,
}

pub trait GC {
    fn gc_ref_image(heap_ref: &mut HeapGcRef, tag: Tag) -> Self;
    fn mark(gc: &mut Gc, env: &Env, struct_: Tag);
}

impl GC for Struct {
    fn gc_ref_image(heap_ref: &mut HeapGcRef, tag: Tag) -> Self {
        match tag.type_of() {
            Type::Struct => match tag {
                Tag::Indirect(image) => Struct {
                    stype: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize).unwrap(),
                    ),
                    vector: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize + 1).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn mark(gc: &mut Gc, env: &Env, struct_: Tag) {
        let mark = gc.mark_image(struct_).unwrap();

        if !mark {
            let vector = Self::gc_ref_image(&mut gc.lock, struct_).vector;

            gc.mark(env, vector)
        }
    }
}

impl Struct {
    pub fn to_image(env: &Env, tag: Tag) -> Self {
        let heap_ref = block_on(env.heap.read());

        match tag.type_of() {
            Type::Struct => match tag {
                Tag::Indirect(image) => Struct {
                    stype: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize).unwrap(),
                    ),
                    vector: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize + 1).unwrap(),
                    ),
                },
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
                let vector = Vector::from(vec).evict(env);

                Struct { stype, vector }.evict(env)
            }
            _ => panic!(),
        }
    }

    pub fn new(env: &Env, key: &str, vec: Vec<Tag>) -> Self {
        Struct {
            stype: Symbol::keyword(key),
            vector: Vector::from(vec).evict(env),
        }
    }

    pub fn view(env: &Env, tag: Tag) -> Tag {
        let image = Self::to_image(env, tag);

        Vector::from(vec![image.stype, image.vector]).evict(env)
    }

    pub fn heap_size(env: &Env, struct_: Tag) -> usize {
        std::mem::size_of::<Struct>() + Vector::heap_size(env, Self::vector(env, struct_))
    }

    pub fn write(env: &Env, tag: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        match tag {
            Tag::Indirect(_) => {
                env.write_string("#s(", stream)?;
                env.write_stream(Self::to_image(env, tag).stype, true, stream)?;
                env.write_string(" ", stream)?;
                env.write_stream(Self::to_image(env, tag).vector, true, stream)?;
                env.write_string(")", stream)
            }
            _ => panic!(),
        }
    }

    pub fn read(env: &Env, stream: Tag) -> exception::Result<Tag> {
        match env.read_char(stream)? {
            Some('(') => {
                let vec_list = match Cons::read(env, stream) {
                    Ok(list) => {
                        if list.null_() {
                            return Err(Exception::new(
                                env,
                                Condition::Type,
                                "mu:read",
                                Tag::nil(),
                            ));
                        }
                        list
                    }
                    Err(_) => {
                        return Err(Exception::new(env, Condition::Syntax, "mu:read", stream));
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
                    _ => Err(Exception::new(env, Condition::Type, "mu:read", stype)),
                }
            }
            _ => Err(Exception::new(env, Condition::Eof, "mu:read", stream)),
        }
    }

    pub fn evict(&self, env: &Env) -> Tag {
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
pub trait CoreFunction {
    fn mu_struct_type(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_struct_vector(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_make_struct(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Struct {
    fn mu_struct_type(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        env.fp_argv_check("mu:struct-type", &[Type::Struct], fp)?;
        fp.value = Self::to_image(env, tag).stype;

        Ok(())
    }

    fn mu_struct_vector(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        env.fp_argv_check("mu:struct-vec", &[Type::Struct], fp)?;
        fp.value = Self::to_image(env, tag).vector;

        Ok(())
    }

    fn mu_make_struct(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let type_ = fp.argv[0];
        let list = fp.argv[1];

        env.fp_argv_check("mu:make-struct", &[Type::Keyword, Type::List], fp)?;

        let vec = Cons::iter(env, list)
            .map(|cons| Cons::car(env, cons))
            .collect::<Vec<Tag>>();

        fp.value = Struct {
            stype: type_,
            vector: Vector::from(vec).evict(env),
        }
        .evict(env);

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
