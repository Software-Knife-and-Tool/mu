//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env function type
use crate::{
    core::{
        env::Env,
        exception,
        gc::{Gc, HeapGcRef},
        indirect::IndirectTag,
        namespace::Namespace,
        types::{Tag, TagType, Type},
    },
    streams::write::Write as _,
    types::{fixnum::Fixnum, symbol::Symbol, vector::Vector},
};

use futures::executor::block_on;

#[derive(Copy, Clone)]
pub struct Function {
    pub arity: Tag, // fixnum # of required arguments
    pub form: Tag,  // list or vector
}

pub trait GC {
    fn ref_form(_: &mut Gc, _: Tag) -> Tag;
    fn mark(_: &mut Gc, _: &Env, _: Tag);
    fn gc_ref_image(_: &mut HeapGcRef, _: Tag) -> Self;
}

impl GC for Function {
    fn ref_form(gc: &mut Gc, func: Tag) -> Tag {
        Self::gc_ref_image(&mut gc.lock, func).form
    }

    fn mark(gc: &mut Gc, env: &Env, function: Tag) {
        let mark = gc.mark_image(function).unwrap();

        if !mark {
            let form = Self::ref_form(gc, function);

            gc.mark(env, form)
        }
    }

    fn gc_ref_image(heap_ref: &mut HeapGcRef, tag: Tag) -> Self {
        match tag.type_of() {
            Type::Function => match tag {
                Tag::Indirect(fn_) => Function {
                    arity: Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize).unwrap()),
                    form: Tag::from_slice(
                        heap_ref.image_slice(fn_.image_id() as usize + 1).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

impl Function {
    pub fn new(arity: Tag, form: Tag) -> Self {
        Function { arity, form }
    }

    pub fn evict(&self, env: &Env) -> Tag {
        let image: &[[u8; 8]] = &[self.arity.as_slice(), self.form.as_slice()];

        let mut heap_ref = block_on(env.heap.write());
        let ind = IndirectTag::new()
            .with_image_id(heap_ref.alloc(image, None, Type::Function as u8).unwrap() as u64)
            .with_heap_id(1)
            .with_tag(TagType::Function);

        Tag::Indirect(ind)
    }

    pub fn to_image(env: &Env, tag: Tag) -> Self {
        let heap_ref = block_on(env.heap.read());

        match tag.type_of() {
            Type::Function => match tag {
                Tag::Indirect(fn_) => Function {
                    arity: Tag::from_slice(heap_ref.image_slice(fn_.image_id() as usize).unwrap()),
                    form: Tag::from_slice(
                        heap_ref.image_slice(fn_.image_id() as usize + 1).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn update(env: &Env, image: &Function, func: Tag) {
        let slices: &[[u8; 8]] = &[image.arity.as_slice(), image.form.as_slice()];

        let offset = match func {
            Tag::Indirect(heap) => heap.image_id(),
            _ => panic!(),
        } as usize;

        let mut heap_ref = block_on(env.heap.write());

        heap_ref.write_image(slices, offset);
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
        match func.type_of() {
            Type::Function => {
                let nreq = Fixnum::as_i64(Function::arity(env, func));
                let form = Function::form(env, func);

                let desc = match form.type_of() {
                    Type::Cons | Type::Null => ("lambda".into(), format!("{:x}", form.as_u64())),
                    Type::Vector => {
                        let ns = Vector::ref_(env, form, 0).unwrap();
                        let offset = Vector::ref_(env, form, 1).unwrap();

                        let ns_ref = block_on(env.ns_map.read());
                        let (_, _, ref namespace) = ns_ref[Namespace::index_of(env, ns)];

                        let fn_name = match namespace {
                            Namespace::Static(static_) => match static_.functions {
                                Some(functions) => {
                                    functions[Fixnum::as_i64(offset) as usize].1.to_string()
                                }
                                None => "<no functions>".to_string(),
                            },
                            _ => panic!(),
                        };

                        (Namespace::name(env, ns).unwrap(), fn_name)
                    }
                    _ => panic!(),
                };

                env.write_string(
                    format!("#<:function :{} [req:{nreq}, form:{}]>", desc.0, desc.1).as_str(),
                    stream,
                )
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn as_tag() {
        assert_eq!(true, true)
    }
}
