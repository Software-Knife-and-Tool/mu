//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! typed vectors
use crate::{
    core::{direct::DirectType, env::Env, tag::Tag, type_::Type},
    gc::gc_::{Gc as _, GcContext},
    types::{
        fixnum::Fixnum,
        vector::{Vector, VTYPEMAP},
    },
    vectors::image::{VecImage, VecImageType, VectorImage},
};

pub trait Gc {
    fn gc_image_ref(_: &mut GcContext, _: Tag, _: usize) -> Option<Tag>;
    fn gc_ref_image(_: &mut GcContext, _: Tag) -> VectorImage;
    fn gc_ref(_: &mut GcContext, _: &Env, _: Tag, _: usize) -> Option<Tag>;
    fn ref_type_of(_: &mut GcContext, _: Tag) -> Type;
    fn ref_length(_: &mut GcContext, _: Tag) -> usize;
    fn mark(_: &mut GcContext, _: &Env, _: Tag);
}

impl Gc for Vector {
    fn gc_image_ref(context: &mut GcContext, vector: Tag, index: usize) -> Option<Tag> {
        let image = Vector::gc_ref_image(context, vector);

        if index >= usize::try_from(Fixnum::as_i64(image.length)).unwrap() {
            None?;
        }

        let Tag::Indirect(vimage) = vector else {
            panic!()
        };

        let slice =
            usize::try_from(vimage.image_id()).unwrap() + <VecImageType<'_> as VecImage>::IMAGE_LEN;

        match Vector::to_type(image.type_).unwrap() {
            Type::Byte => {
                let slice = context.heap_ref.image_data_slice(slice, index, 1).unwrap();

                Some(slice[0].into())
            }
            Type::Char => {
                let slice = context.heap_ref.image_data_slice(slice, index, 1).unwrap();
                let ch: char = slice[0].into();

                Some(ch.into())
            }
            Type::T => Some(Tag::from_slice(
                context
                    .heap_ref
                    .image_data_slice(slice, index * 8, 8)
                    .unwrap(),
            )),
            Type::Fixnum => {
                let slice = context
                    .heap_ref
                    .image_data_slice(slice, index * 8, 8)
                    .unwrap();

                Some(Fixnum::with_i64_or_panic(i64::from_le_bytes(
                    slice[0..8].try_into().unwrap(),
                )))
            }
            Type::Float => {
                let slice = context
                    .heap_ref
                    .image_data_slice(slice, index * 4, 4)
                    .unwrap();

                Some(f32::from_le_bytes(slice[0..4].try_into().unwrap()).into())
            }
            _ => panic!(),
        }
    }

    fn gc_ref_image(context: &mut GcContext, tag: Tag) -> VectorImage {
        match tag.type_of() {
            Type::Vector => match tag {
                Tag::Indirect(image) => {
                    let heap_ref = &context.heap_ref;
                    let slice = usize::try_from(image.image_id()).unwrap();

                    VectorImage {
                        type_: Tag::from_slice(heap_ref.image_slice(slice).unwrap()),
                        length: Tag::from_slice(heap_ref.image_slice(slice + 1).unwrap()),
                    }
                }
                Tag::Direct(_) => panic!(),
            },
            _ => panic!(),
        }
    }

    fn gc_ref(context: &mut GcContext, env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        match vector.type_of() {
            Type::Vector => match vector {
                Tag::Direct(_direct) => {
                    let ch: char = vector.data(env).to_le_bytes()[index].into();

                    Some(ch.into())
                }
                Tag::Indirect(_) => <Vector as Gc>::gc_image_ref(context, vector, index),
            },
            _ => panic!(),
        }
    }

    fn ref_type_of(context: &mut GcContext, vector: Tag) -> Type {
        match vector {
            Tag::Direct(direct) => match direct.dtype() {
                DirectType::String => Type::Char,
                DirectType::ByteVec => Type::Byte,
                _ => panic!(),
            },
            Tag::Indirect(_) => {
                let image = Self::gc_ref_image(context, vector);

                match VTYPEMAP
                    .iter()
                    .copied()
                    .find(|desc| image.type_.eq_(&desc.0))
                {
                    Some(desc) => desc.1,
                    None => panic!(),
                }
            }
        }
    }

    fn ref_length(context: &mut GcContext, vector: Tag) -> usize {
        match vector {
            Tag::Direct(direct) => direct.ext() as usize,
            Tag::Indirect(_) => {
                let image = Self::gc_ref_image(context, vector);

                usize::try_from(Fixnum::as_i64(image.length)).unwrap()
            }
        }
    }

    fn mark(context: &mut GcContext, env: &Env, vector: Tag) {
        match vector {
            Tag::Direct(_) => (),
            Tag::Indirect(_) => {
                let marked = context.mark_image(vector).unwrap();

                if !marked && Self::ref_type_of(context, vector) == Type::T {
                    for index in 0..Self::ref_length(context, vector) {
                        let value = Self::gc_ref(context, env, vector, index).unwrap();

                        context.mark(env, value);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn vector_test() {
        assert!(true);
    }
}
