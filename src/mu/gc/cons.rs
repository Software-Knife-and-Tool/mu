//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! cons type
use crate::{
    core::{direct::DirectTag, env::Env, tag::Tag, type_::Type},
    gc::gc_::{Gc as _, GcContext},
    types::cons::Cons,
};

pub trait Gc {
    fn gc_ref_image(_: &GcContext, tag: Tag) -> Self;
    fn ref_car(_: &GcContext, _: Tag) -> Tag;
    fn ref_cdr(_: &GcContext, _: Tag) -> Tag;
    fn mark(_: &mut GcContext, _: &Env, _: Tag);
}

impl Gc for Cons {
    fn gc_ref_image(context: &GcContext, tag: Tag) -> Self {
        assert_eq!(tag.type_of(), Type::Cons);

        match tag {
            Tag::Indirect(main) => {
                let heap_ref = &context.heap_ref;
                let slice = usize::try_from(main.image_id()).unwrap();
                Cons {
                    car: Tag::from_slice(heap_ref.image_slice(slice).unwrap()),
                    cdr: Tag::from_slice(heap_ref.image_slice(slice + 1).unwrap()),
                }
            }
            Tag::Direct(_) => panic!(),
        }
    }

    fn ref_car(context: &GcContext, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Direct(_) => DirectTag::cons_destruct(cons).0,
                Tag::Indirect(_) => Self::gc_ref_image(context, cons).car,
            },
            _ => panic!(),
        }
    }

    fn ref_cdr(context: &GcContext, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Indirect(_) => Self::gc_ref_image(context, cons).cdr,
                Tag::Direct(_) => DirectTag::cons_destruct(cons).1,
            },
            _ => panic!(),
        }
    }

    fn mark(context: &mut GcContext, env: &Env, cons: Tag) {
        match cons {
            Tag::Direct(_) => {
                let car = Self::ref_car(context, cons);
                let cdr = Self::ref_cdr(context, cons);

                context.mark(env, car);
                context.mark(env, cdr);
            }
            Tag::Indirect(_) => {
                let mark = context.mark_image(cons).unwrap();
                if !mark {
                    context.mark(env, Self::ref_car(context, cons));
                    context.mark(env, Self::ref_cdr(context, cons));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::tag::Tag, types::cons::Cons};

    #[test]
    fn cons_test() {
        match Cons::new(Tag::nil(), Tag::nil()) {
            _ => assert!(true),
        }
    }
}
