//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu function type
use crate::{
    core::{
        exception,
        gc::Core as _,
        indirect::IndirectTag,
        mu::Mu,
        system::Core as _,
        types::{Tag, TagType, Type},
    },
    types::{
        fixnum::Fixnum,
        symbol::Symbol,
        vecimage::{TypedVec, VecType},
        vector::Core as _,
        vector::Vector,
    },
};

use futures::executor::block_on;

#[derive(Copy, Clone)]
pub struct Function {
    pub arity: Tag, // fixnum # of required arguments
    pub form: Tag,  // list or native keyword
}

impl Function {
    pub fn new(arity: Tag, form: Tag) -> Self {
        Function { arity, form }
    }

    pub fn evict(&self, mu: &Mu) -> Tag {
        let image: &[[u8; 8]] = &[self.arity.as_slice(), self.form.as_slice()];

        let mut heap_ref = block_on(mu.heap.write());
        let ind = IndirectTag::new()
            .with_image_id(heap_ref.alloc(image, None, Type::Function as u8).unwrap() as u64)
            .with_heap_id(1)
            .with_tag(TagType::Function);

        Tag::Indirect(ind)
    }

    pub fn to_image(mu: &Mu, tag: Tag) -> Self {
        match tag.type_of() {
            Type::Function => match tag {
                Tag::Indirect(fn_) => {
                    let heap_ref = block_on(mu.heap.read());

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

    pub fn update(mu: &Mu, image: &Function, func: Tag) {
        let slices: &[[u8; 8]] = &[image.arity.as_slice(), image.form.as_slice()];

        let offset = match func {
            Tag::Indirect(heap) => heap.image_id(),
            _ => panic!(),
        } as usize;

        let mut heap_ref = block_on(mu.heap.write());

        heap_ref.write_image(slices, offset);
    }

    pub fn arity(mu: &Mu, func: Tag) -> Tag {
        Self::to_image(mu, func).arity
    }

    pub fn form(mu: &Mu, func: Tag) -> Tag {
        Self::to_image(mu, func).form
    }
}

pub trait Core {
    fn mark(_: &Mu, _: Tag);
    fn heap_size(_: &Mu, _: Tag) -> usize;
    fn view(_: &Mu, _: Tag) -> Tag;
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl Core for Function {
    fn mark(mu: &Mu, function: Tag) {
        let mark = mu.mark_image(function).unwrap();

        if !mark {
            mu.mark(Self::form(mu, function))
        }
    }

    fn view(mu: &Mu, func: Tag) -> Tag {
        let vec = vec![Self::arity(mu, func), Self::form(mu, func)];

        TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu)
    }

    fn heap_size(mu: &Mu, fn_: Tag) -> usize {
        match Self::form(mu, fn_).type_of() {
            Type::Null | Type::Cons => std::mem::size_of::<Function>(),
            Type::Keyword => std::mem::size_of::<Function>(),
            _ => panic!(),
        }
    }

    fn write(mu: &Mu, func: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        match func.type_of() {
            Type::Function => {
                let nreq = Fixnum::as_i64(Function::arity(mu, func));
                let form = Function::form(mu, func);

                let desc = match form.type_of() {
                    Type::Cons | Type::Null => {
                        (":lambda".to_string(), format!("{:x}", form.as_u64()))
                    }
                    Type::Keyword => (
                        ":core".to_string(),
                        Vector::as_string(mu, Symbol::name(mu, form)).to_string(),
                    ),
                    Type::Vector if Vector::type_of(mu, form) == Type::Char => (
                        ":namespace".to_string(),
                        Vector::as_string(mu, Symbol::name(mu, form)).to_string(),
                    ),
                    _ => panic!(),
                };

                mu.write_string(
                    format!("#<:function {} [req:{nreq}, form:{}]>", desc.0, desc.1).as_str(),
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
