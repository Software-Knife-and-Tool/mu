//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// function type
#[rustfmt::skip]
use {
    crate::{
        core_::{
            core::CoreFnDef,
            direct::DirectTag,
            env::Env,
            exception,
            indirect::IndirectTag,
            namespace::Namespace,
            tag::{Tag, TagType},
            type_::Type,
        },
        spaces::{
            gc::{Gc as _, GcContext},
            heap::HeapRequest,
        },
        streams::writer::StreamWriter,
        types::{
            cons::Cons,
            fixnum::Fixnum,
            symbol::Symbol,
            vector::Vector
        },
    },
    futures_lite::future::block_on,
};

#[derive(Copy, Clone)]
pub struct Function {
    pub arity: Tag, // fixnum # of required arguments
    pub form: Tag,  // list
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
            Tag::Direct(_) => {
                let (arity, form) = Self::destruct(env, tag);

                Self::new(arity, form)
            }
        }
    }

    pub fn staticns_deref(env: &Env, func: Tag) -> (String, Namespace, CoreFnDef) {
        assert!(func.type_of() == Type::Function);

        let (ns_id, offset) = DirectTag::function_id(func);

        let ns_ref = block_on(env.ns_map.read());
        let (_, ref name, ref namespace) = ns_ref[ns_id as usize];

        if let Namespace::Static(static_) = namespace {
            (
                name.to_string(),
                namespace.clone(),
                static_.functions.unwrap()[offset as usize],
            )
        } else {
            panic!()
        }
    }

    pub fn destruct(env: &Env, func: Tag) -> (Tag, Tag) {
        assert!(func.type_of() == Type::Function);

        match func {
            Tag::Direct(_) => {
                let (_, offset) = DirectTag::function_id(func);
                let desc = Self::staticns_deref(env, func);

                let arity: Tag = desc.2 .1.into();
                let index: Tag = offset.into();

                (arity, index)
            }
            Tag::Indirect(fn_) => {
                let heap_ref = block_on(env.heap.read());

                (
                    Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize).unwrap()),
                    Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize + 1).unwrap()),
                )
            }
        }
    }

    pub fn with_heap(&self, env: &Env) -> Tag {
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

    pub fn update(env: &Env, image: &Function, func: Tag) {
        match func {
            Tag::Indirect(heap) => {
                let slices: &[[u8; 8]] = &[image.arity.as_slice(), image.form.as_slice()];
                let mut heap_ref = block_on(env.heap.write());

                heap_ref.write_image(slices, heap.image_id() as usize)
            }
            Tag::Direct(_tag) => panic!(),
        }
    }

    pub fn view(env: &Env, func: Tag) -> Tag {
        let (arity, form) = Self::destruct(env, func);
        let vec = vec![arity, form];

        Vector::from(vec).evict(env)
    }

    pub fn heap_size(env: &Env, func: Tag) -> usize {
        match Function::destruct(env, func).1.type_of() {
            Type::Null | Type::Cons => std::mem::size_of::<Function>(),
            Type::Vector => std::mem::size_of::<Function>(),
            Type::Symbol => {
                std::mem::size_of::<Fixnum>() + Symbol::heap_size(env, Self::destruct(env, func).1)
            }
            _ => panic!(),
        }
    }

    pub fn write(env: &Env, func: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        assert_eq!(func.type_of(), Type::Function);

        let (arity, form) = Function::destruct(env, func);
        let nreq = Fixnum::as_i64(arity);

        let desc = match form.type_of() {
            Type::Null => (
                "null".to_string(),
                "lambda".to_string(),
                format!("{:x}", form.as_u64()),
            ),
            Type::Cons => match Cons::destruct(env, form).1.type_of() {
                Type::Null | Type::Cons => (
                    "null".to_string(),
                    "lambda".to_string(),
                    format!("{:x}", form.as_u64()),
                ),
                _ => panic!(),
            },
            Type::Fixnum => {
                let (name, _ns, core_def) = Self::staticns_deref(env, func);

                (name, "core".into(), core_def.0.to_string())
            }
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
