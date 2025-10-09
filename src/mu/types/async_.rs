//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// async function type
use {
    crate::{
        core_::{
            env::Env,
            exception,
            indirect::IndirectTag,
            tag::{Tag, TagType},
            type_::Type,
        },
        spaces::{
            gc::{Gc as _, GcContext},
            heap::HeapRequest,
        },
        streams::writer::StreamWriter,
        types::{cons::Cons, fixnum::Fixnum, symbol::Symbol, vector::Vector},
    },
    futures_lite::future::block_on,
};

#[derive(Copy, Clone)]
pub struct Async {
    pub arity: Tag,
    pub form: Tag,
}

pub trait Gc {
    fn ref_form(_: &mut GcContext, _: Tag) -> Tag;
    fn mark(_: &mut GcContext, _: &Env, _: Tag);
    fn gc_ref_image(_: &mut GcContext, _: Tag) -> Self;
}

impl Gc for Async {
    fn ref_form(context: &mut GcContext, func: Tag) -> Tag {
        Self::gc_ref_image(context, func).form
    }

    fn mark(context: &mut GcContext, env: &Env, function: Tag) {
        let mark = context.mark_image(function).unwrap();

        if !mark {
            let form = Self::ref_form(context, function);

            context.mark(env, form)
        }
    }

    fn gc_ref_image(context: &mut GcContext, tag: Tag) -> Self {
        let heap_ref = &context.heap_ref;

        assert_eq!(tag.type_of(), Type::Async);
        match tag {
            Tag::Indirect(fn_) => Async {
                arity: Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize).unwrap()),
                form: Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize + 1).unwrap()),
            },
            _ => panic!(),
        }
    }
}

impl Async {
    pub fn new(arity: Tag, form: Tag) -> Self {
        Async { arity, form }
    }

    pub fn to_image(env: &Env, tag: Tag) -> Self {
        let heap_ref = block_on(env.heap.read());

        assert_eq!(tag.type_of(), Type::Async);
        match tag {
            Tag::Indirect(fn_) => Async {
                arity: Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize).unwrap()),
                form: Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize + 1).unwrap()),
            },
            _ => panic!(),
        }
    }

    pub fn destruct(env: &Env, func: Tag) -> (Tag, Tag) {
        assert!(func.type_of() == Type::Async);

        match func {
            Tag::Indirect(fn_) => {
                let heap_ref = block_on(env.heap.read());

                (
                    Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize).unwrap()),
                    Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize + 1).unwrap()),
                )
            }
            _ => panic!(),
        }
    }

    pub fn with_heap(&self, env: &Env) -> Tag {
        let image: &[[u8; 8]] = &[self.arity.as_slice(), self.form.as_slice()];
        let mut heap_ref = block_on(env.heap.write());
        let ha = HeapRequest {
            env,
            image,
            vdata: None,
            type_id: Type::Async as u8,
        };

        match heap_ref.alloc(&ha) {
            Some(image_id) => {
                let ind = IndirectTag::new()
                    .with_image_id(image_id as u64)
                    .with_heap_id(1)
                    .with_tag(TagType::Async);

                Tag::Indirect(ind)
            }
            None => panic!(),
        }
    }

    pub fn update(env: &Env, image: &Async, func: Tag) {
        let slices: &[[u8; 8]] = &[image.arity.as_slice(), image.form.as_slice()];

        let offset = match func {
            Tag::Indirect(heap) => heap.image_id(),
            _ => panic!(),
        } as usize;

        let mut heap_ref = block_on(env.heap.write());

        heap_ref.write_image(slices, offset);
    }

    pub fn view(env: &Env, func: Tag) -> Tag {
        let (arity, form) = Self::destruct(env, func);
        let vec = vec![arity, form];

        Vector::from(vec).with_heap(env)
    }

    pub fn image_size(env: &Env, func: Tag) -> usize {
        let form = Self::destruct(env, func).1;
        match form.type_of() {
            Type::Null | Type::Cons => std::mem::size_of::<Async>(),
            Type::Vector => std::mem::size_of::<Async>(),
            Type::Symbol => std::mem::size_of::<Fixnum>() + Symbol::image_size(env, form),
            _ => panic!(),
        }
    }

    pub fn write(env: &Env, func: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        assert_eq!(func.type_of(), Type::Async);

        let (arity, form) = Async::destruct(env, func);
        let nreq = Fixnum::as_i64(arity);

        let desc = match form.type_of() {
            Type::Null => (
                "null".to_string(),
                "alambda".to_string(),
                format!("{:x}", form.as_u64()),
            ),
            Type::Cons => match Cons::destruct(env, form).1.type_of() {
                Type::Null | Type::Cons => (
                    "null".to_string(),
                    "alambda".to_string(),
                    format!("{:x}", form.as_u64()),
                ),
                _ => panic!(),
            },
            _ => panic!(),
        };

        StreamWriter::write_str(
            env,
            format!(
                "#<:function :{} [type:{}, req:{nreq}, form:{}]>",
                desc.0, desc.1, desc.2
            )
            .as_str(),
            stream,
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn as_tag() {
        assert_eq!(true, true)
    }
}
