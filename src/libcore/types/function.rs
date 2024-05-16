//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env function type
use crate::{
    core::{
        env::Env,
        exception,
        gc::Gc,
        indirect::IndirectTag,
        types::{Tag, TagType, Type},
    },
    streams::write::Core as _,
    types::{
        fixnum::Fixnum,
        indirect_vector::{TypedVector, VecType},
        namespace::Namespace,
        vector::{Core as _, Vector},
    },
};

use futures::executor::block_on;

#[derive(Copy, Clone)]
pub struct Function {
    pub arity: Tag, // fixnum # of required arguments
    pub form: Tag,  // list or symbol
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
        match tag.type_of() {
            Type::Function => match tag {
                Tag::Indirect(fn_) => {
                    let heap_ref = block_on(env.heap.read());

                    Function {
                        arity: Tag::from_slice(
                            heap_ref.image_slice(fn_.image_id() as usize).unwrap(),
                        ),
                        form: Tag::from_slice(
                            heap_ref.image_slice(fn_.image_id() as usize + 1).unwrap(),
                        ),
                    }
                }
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

    pub fn mark(env: &Env, function: Tag) {
        let mark = Gc::mark_image(env, function).unwrap();

        if !mark {
            Gc::mark(env, Self::form(env, function))
        }
    }
}

pub trait Core {
    fn heap_size(_: &Env, _: Tag) -> usize;
    fn view(_: &Env, _: Tag) -> Tag;
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl Core for Function {
    fn view(env: &Env, func: Tag) -> Tag {
        let vec = vec![Self::arity(env, func), Self::form(env, func)];

        TypedVector::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
    }

    fn heap_size(env: &Env, fn_: Tag) -> usize {
        match Self::form(env, fn_).type_of() {
            Type::Null | Type::Cons => std::mem::size_of::<Function>(),
            Type::Vector => std::mem::size_of::<Function>(),
            Type::Symbol => std::mem::size_of::<Function>(),
            _ => panic!(),
        }
    }

    fn write(env: &Env, func: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        match func.type_of() {
            Type::Function => {
                let nreq = Fixnum::as_i64(Function::arity(env, func));
                let form = Function::form(env, func);

                let desc = match form.type_of() {
                    Type::Cons | Type::Null => {
                        ("lambda".to_string(), format!("{:x}", form.as_u64()))
                    }
                    Type::Vector => {
                        let ns = Vector::ref_(env, form, 0).unwrap();
                        let name = Vector::ref_(env, form, 1).unwrap();

                        (
                            Namespace::ns_name(env, ns).unwrap(),
                            Vector::as_string(env, name).to_string(),
                        )
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
    use crate::core::types::Tag;
    use crate::types::function::Function;

    #[test]
    fn as_tag() {
        match Function::new(Tag::from(0i64), Tag::nil()) {
            _ => assert_eq!(true, true),
        }
    }
}
