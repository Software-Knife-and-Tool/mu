//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// struct type
use crate::{
    core::{env::Env, tag::Tag, type_::Type},
    gc::gc_::{Gc as _, GcContext},
    types::struct_::Struct,
};

pub trait Gc {
    fn gc_ref_image(_: &mut GcContext, tag: Tag) -> Self;
    fn mark(_: &mut GcContext, env: &Env, struct_: Tag);
}

impl Gc for Struct {
    fn gc_ref_image(context: &mut GcContext, tag: Tag) -> Self {
        assert_eq!(tag.type_of(), Type::Struct);

        match tag {
            Tag::Indirect(image) => {
                let heap_ref = &context.heap_ref;
                let slice = usize::try_from(image.image_id()).unwrap();

                Struct {
                    stype: Tag::from_slice(heap_ref.image_slice(slice).unwrap()),
                    vector: Tag::from_slice(heap_ref.image_slice(slice + 1).unwrap()),
                }
            }
            Tag::Direct(_) => panic!(),
        }
    }

    fn mark(context: &mut GcContext, env: &Env, struct_: Tag) {
        let mark = context.mark_image(struct_).unwrap();

        if !mark {
            let vector = Self::gc_ref_image(context, struct_).vector;

            context.mark(env, vector);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn struct_test() {
        assert!(true);
    }
}
