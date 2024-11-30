//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! typed vectors
use {
    crate::{
        core::{
            direct::{DirectTag, DirectType},
            env::Env,
            exception,
            gc::HeapGcRef,
            types::{Tag, Type},
        },
        streams::write::Core as _,
        types::{
            fixnum::Fixnum,
            stream::{Core as _, Stream},
            vector::{Core as _, Vector},
        },
        vectors::image::{VecImage, VecImageType, VectorImage, VectorImageType},
    },
    std::str,
};

use futures::executor::block_on;

impl Vector {
    pub fn to_image(env: &Env, tag: Tag) -> VectorImage {
        let heap_ref = block_on(env.heap.read());

        match tag.type_of() {
            Type::Vector => match tag {
                Tag::Indirect(image) => VectorImage {
                    type_: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize).unwrap(),
                    ),
                    length: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize + 1).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
    pub fn gc_ref_image(heap_ref: &mut HeapGcRef, tag: Tag) -> VectorImage {
        match tag.type_of() {
            Type::Vector => match tag {
                Tag::Indirect(image) => VectorImage {
                    type_: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize).unwrap(),
                    ),
                    length: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize + 1).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

pub trait Core<'a> {
    fn evict(&self, _: &Env) -> Tag;
    fn heap_size(_: &Env, _: Tag) -> usize;
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl Core<'_> for Vector {
    fn heap_size(env: &Env, vector: Tag) -> usize {
        match vector {
            Tag::Direct(_) => std::mem::size_of::<DirectTag>(),
            Tag::Indirect(_) => {
                let len = Self::length(env, vector);
                let size = match Self::type_of(env, vector) {
                    Type::Byte | Type::Char => 1,
                    Type::Fixnum | Type::Float | Type::T => 8,
                    _ => panic!(),
                };

                std::mem::size_of::<VectorImage>() + (size * len)
            }
        }
    }

    fn evict(&self, env: &Env) -> Tag {
        match self {
            Vector::Direct(tag) => *tag,
            Vector::Indirect(image, ivec) => {
                let indirect = match ivec {
                    VectorImageType::T(_) => VecImageType::T(image, ivec),
                    VectorImageType::Char(_) => VecImageType::Char(image, ivec),
                    VectorImageType::Bit(_) => VecImageType::Bit(image, ivec),
                    VectorImageType::Byte(_) => VecImageType::Byte(image, ivec),
                    VectorImageType::Fixnum(_) => VecImageType::Fixnum(image, ivec),
                    VectorImageType::Float(_) => VecImageType::Float(image, ivec),
                };

                match ivec {
                    VectorImageType::T(_) => indirect.evict(env),
                    _ => match Self::cached(env, &indirect) {
                        Some(tag) => tag,
                        None => {
                            let tag = indirect.evict(env);

                            Self::cache(env, tag);
                            tag
                        }
                    },
                }
            }
        }
    }

    fn write(env: &Env, vector: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        match vector {
            Tag::Direct(direct) => match direct.dtype() {
                DirectType::String => match str::from_utf8(&vector.data(env).to_le_bytes()) {
                    Ok(s) => {
                        if escape {
                            env.write_string("\"", stream).unwrap()
                        }

                        for nth in 0..DirectTag::length(vector) {
                            Stream::write_char(env, stream, s.as_bytes()[nth] as char)?;
                        }

                        if escape {
                            env.write_string("\"", stream).unwrap()
                        }

                        Ok(())
                    }
                    Err(_) => panic!(),
                },
                DirectType::ByteVec => {
                    env.write_string("#(:byte", stream)?;

                    for tag in Vector::iter(env, vector) {
                        env.write_string(" ", stream)?;
                        env.write_stream(tag, false, stream)?;
                    }

                    env.write_string(")", stream)
                }
                _ => panic!(),
            },
            Tag::Indirect(_) => match Self::type_of(env, vector) {
                Type::Char => {
                    if escape {
                        env.write_string("\"", stream)?;
                    }

                    for ch in Vector::iter(env, vector) {
                        env.write_stream(ch, false, stream)?;
                    }

                    if escape {
                        env.write_string("\"", stream)?;
                    }

                    Ok(())
                }
                Type::Bit => {
                    env.write_string("#*", stream)?;

                    let _len = Vector::length(env, vector);
                    for bit in Vector::iter(env, vector) {
                        let digit = Fixnum::as_i64(bit);

                        env.write_string(if digit == 1 { "1" } else { "0" }, stream)?
                    }

                    Ok(())
                }
                _ => {
                    env.write_string("#(", stream)?;
                    env.write_stream(Self::to_image(env, vector).type_, true, stream)?;

                    for tag in Vector::iter(env, vector) {
                        env.write_string(" ", stream)?;
                        env.write_stream(tag, false, stream)?;
                    }

                    env.write_string(")", stream)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
