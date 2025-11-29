//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// async function type
use crate::{
    core::{env::Env, tag::Tag, type_::Type},
    gc::gc_::{Gc as _, GcContext},
    types::async_::Async,
};

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

            context.mark(env, form);
        }
    }

    fn gc_ref_image(context: &mut GcContext, tag: Tag) -> Self {
        assert_eq!(tag.type_of(), Type::Async);

        match tag {
            Tag::Indirect(fn_) => {
                let heap_ref = &context.heap_ref;
                let slice = usize::try_from(fn_.image_id()).unwrap();

                Async {
                    arity: Tag::from_slice(heap_ref.image_slice(slice).unwrap()),
                    form: Tag::from_slice(heap_ref.image_slice(slice + 1).unwrap()),
                }
            }
            Tag::Direct(_) => panic!(),
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
