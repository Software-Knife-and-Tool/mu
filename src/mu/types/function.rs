//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// function type
use {
    crate::{
        core::{
            env::Env,
            exception,
            gc::{Gc as _, GcContext},
            heap::HeapRequest,
            image::Image,
            image_cache::ImageCache,
            indirect::IndirectTag,
            namespace::Namespace,
            tag::{Tag, TagType},
            type_::Type,
        },
        streams::writer::StreamWriter,
        types::{cons::Cons, fixnum::Fixnum, symbol::Symbol, vector::Vector},
    },
    futures_lite::future::block_on,
};

#[derive(Copy, Clone)]
pub struct Function {
    pub arity: Tag, // fixnum # of required arguments
    pub form: Tag,  // dotted pair or list
}

pub trait Gc {
    fn ref_form(_: &mut GcContext, _: Tag) -> Tag;
    fn mark(_: &mut GcContext, _: &Env, _: Tag);
    fn gc_ref_image(_: &mut GcContext, _: Tag) -> Self;
}

impl Gc for Function {
    fn ref_form(context: &mut GcContext, func: Tag) -> Tag {
        Self::gc_ref_image(context, func).form
    }

    fn mark(context: &mut GcContext, env: &Env, function: Tag) {
        let mark = context.mark_image(function).unwrap();

        if !mark {
            let form = Self::ref_form(context, function);

            context.mark(env, form);
        }
    }

    fn gc_ref_image(context: &mut GcContext, tag: Tag) -> Self {
        assert_eq!(tag.type_of(), Type::Function);

        let heap_ref = &context.heap_ref;

        match tag {
            Tag::Indirect(fn_) => Self::new(
                Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize).unwrap()),
                Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize + 1).unwrap()),
            ),
            _ => panic!(),
        }
    }
}

impl Function {
    pub fn new(arity: Tag, form: Tag) -> Self {
        Function { arity, form }
    }

    pub fn to_image(env: &Env, tag: Tag) -> Self {
        assert_eq!(tag.type_of(), Type::Function);

        let heap_ref = block_on(env.heap.read());

        match tag {
            Tag::Indirect(fn_) => Self::new(
                Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize).unwrap()),
                Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize + 1).unwrap()),
            ),
            Tag::Direct(_) | Tag::Image(_) => {
                let (index, _) = Image::detag(tag);

                match ImageCache::ref_(env, index) {
                    Image::Function(fn_) => fn_,
                    _ => panic!(),
                }
            }
        }
    }

    pub fn to_image_tag(self, env: &Env) -> Tag {
        let image = Image::Function(self);

        Image::to_tag(&image, env, Type::Function as u8)
    }

    pub fn evict(&self, env: &Env) -> Tag {
        let image: &[[u8; 8]] = &[self.arity.as_slice(), self.form.as_slice()];
        let mut heap_ref = block_on(env.heap.write());
        let type_id = Type::Function as u8;

        let ha = HeapRequest {
            env,
            image,
            vdata: None,
            type_id,
        };

        match heap_ref.alloc(&ha) {
            Some(image_id) => {
                let ind = IndirectTag::new()
                    .with_image_id(image_id as u64)
                    .with_heap_id(1)
                    .with_tag(TagType::Function);

                Tag::Indirect(ind)
            }
            None => panic!(),
        }
    }

    pub fn evict_image(tag: Tag, env: &Env) -> Tag {
        Self::to_image(env, tag).evict(env)
    }

    pub fn update(env: &Env, image: &Function, func: Tag) {
        match func {
            Tag::Indirect(heap) => {
                let slices: &[[u8; 8]] = &[image.arity.as_slice(), image.form.as_slice()];
                let mut heap_ref = block_on(env.heap.write());

                heap_ref.write_image(slices, heap.image_id() as usize)
            }
            Tag::Image(_tag) => ImageCache::update(env, Image::Function(*image), func),
            Tag::Direct(_tag) => panic!(),
        }
    }

    pub fn arity(env: &Env, func: Tag) -> Tag {
        Self::to_image(env, func).arity
    }

    pub fn form(env: &Env, func: Tag) -> Tag {
        Self::to_image(env, func).form
    }

    pub fn view(env: &Env, func: Tag) -> Tag {
        let vec = vec![Self::arity(env, func), Self::form(env, func)];

        Vector::from(vec).evict(env)
    }

    pub fn heap_size(env: &Env, fn_: Tag) -> usize {
        match Self::form(env, fn_).type_of() {
            Type::Null | Type::Cons => std::mem::size_of::<Function>(),
            Type::Vector => std::mem::size_of::<Function>(),
            Type::Symbol => {
                std::mem::size_of::<Fixnum>() + Symbol::heap_size(env, Self::form(env, fn_))
            }
            _ => panic!(),
        }
    }

    pub fn write(env: &Env, func: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        assert_eq!(func.type_of(), Type::Function);

        let nreq = Fixnum::as_i64(Function::arity(env, func));
        let form = Function::form(env, func);

        let desc = match form.type_of() {
            Type::Null => (
                "null".to_string(),
                "lambda".to_string(),
                format!("{:x}", form.as_u64()),
            ),
            Type::Cons => match Cons::cdr(env, form).type_of() {
                Type::Null | Type::Cons => (
                    "null".to_string(),
                    "lambda".to_string(),
                    format!("{:x}", form.as_u64()),
                ),
                Type::Fixnum => {
                    let ns = Cons::car(env, form);
                    let offset = Cons::cdr(env, form);

                    let ns_ref = block_on(env.ns_map.read());
                    let (_, _, ref namespace) = ns_ref[Namespace::index_of(env, ns)];

                    let fn_name = match namespace {
                        Namespace::Static(static_) => match static_.functions {
                            Some(functions) => {
                                functions[Fixnum::as_i64(offset) as usize].0.to_string()
                            }
                            None => "<undef>".to_string(),
                        },
                        _ => panic!(),
                    };

                    (Namespace::name(env, ns).unwrap(), "native".into(), fn_name)
                }
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
        assert!(true)
    }
}
