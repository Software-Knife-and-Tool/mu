//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! typed vectors
use {
    crate::{
        core::{
            direct::{DirectExt, DirectTag, DirectType},
            env::Env,
            exception,
            gc::{Gc, HeapGcRef},
            indirect::IndirectTag,
            types::{Tag, TagType, Type},
        },
        streams::write::Core as _,
        types::{
            fixnum::{Core as _, Fixnum},
            stream::{Core as _, Stream},
            symbol::{Core as _, Symbol},
            vector::{Core as _, Vector},
        },
    },
    std::str,
};

use futures::executor::block_on;

pub struct VectorImage {
    pub type_: Tag,  // type keyword
    pub length: Tag, // fixnum
}

pub enum VectorImageType {
    Bit(Vec<u8>),
    Byte(Vec<u8>),
    Char(String),
    Fixnum(Vec<i64>),
    Float(Vec<f32>),
    T(Vec<Tag>),
}

// vector types
#[allow(dead_code)]
pub enum VecImageType<'a> {
    Bit(&'a VectorImage, &'a VectorImageType),
    Byte(&'a VectorImage, &'a VectorImageType),
    Char(&'a VectorImage, &'a VectorImageType),
    Fixnum(&'a VectorImage, &'a VectorImageType),
    Float(&'a VectorImage, &'a VectorImageType),
    T(&'a VectorImage, &'a VectorImageType),
}

impl From<Vec<Tag>> for Vector {
    fn from(vec: Vec<Tag>) -> Vector {
        let image = VectorImage {
            type_: Symbol::keyword("t"),
            length: Fixnum::with_or_panic(vec.len()),
        };

        Vector::Indirect(image, VectorImageType::T(vec.to_vec()))
    }
}

impl From<&str> for Vector {
    fn from(str: &str) -> Vector {
        let len = str.len();

        if len > DirectTag::DIRECT_STR_MAX {
            let image = VectorImage {
                type_: Symbol::keyword("char"),
                length: Fixnum::with_or_panic(str.len()),
            };

            Vector::Indirect(image, VectorImageType::Char(str.into()))
        } else {
            let mut data: [u8; 8] = 0_u64.to_le_bytes();

            for (src, dst) in str.as_bytes().iter().zip(data.iter_mut()) {
                *dst = *src
            }

            Vector::Direct(DirectTag::to_tag(
                u64::from_le_bytes(data),
                DirectExt::Length(len),
                DirectType::String,
            ))
        }
    }
}

impl From<&[u8]> for Vector {
    fn from(bytes: &[u8]) -> Vector {
        let len = bytes.len();

        if len > DirectTag::DIRECT_STR_MAX {
            let image = VectorImage {
                type_: Symbol::keyword("byte"),
                length: Fixnum::with_or_panic(bytes.len()),
            };

            Vector::Indirect(image, VectorImageType::Byte(bytes.to_vec()))
        } else {
            let mut data: [u8; 8] = 0_u64.to_le_bytes();

            for (src, dst) in bytes.to_vec().iter().zip(data.iter_mut()) {
                *dst = *src
            }

            Vector::Direct(DirectTag::to_tag(
                u64::from_le_bytes(data),
                DirectExt::Length(len),
                DirectType::ByteVec,
            ))
        }
    }
}

impl From<String> for Vector {
    fn from(str: String) -> Vector {
        (&*str).into()
    }
}

impl From<Vec<i64>> for Vector {
    fn from(vec: Vec<i64>) -> Vector {
        let image = VectorImage {
            type_: Symbol::keyword("fixnum"),
            length: Fixnum::with_or_panic(vec.len()),
        };

        Vector::Indirect(image, VectorImageType::Fixnum(vec.clone()))
    }
}

impl From<(Vec<i8>, usize)> for Vector {
    fn from(vec_def: (Vec<i8>, usize)) -> Vector {
        let (vec, len) = vec_def;

        let image = VectorImage {
            type_: Symbol::keyword("bit"),
            length: Fixnum::with_or_panic(len),
        };

        let u8_slice = vec.iter().map(|i8_| *i8_ as u8).collect::<Vec<u8>>();

        Vector::Indirect(image, VectorImageType::Bit(u8_slice))
    }
}

impl From<Vec<u8>> for Vector {
    fn from(vec: Vec<u8>) -> Vector {
        (&*vec).into()
    }
}

impl From<Vec<f32>> for Vector {
    fn from(vec: Vec<f32>) -> Vector {
        let image = VectorImage {
            type_: Symbol::keyword("float"),
            length: Fixnum::with_or_panic(vec.len()),
        };

        Vector::Indirect(image, VectorImageType::Float(vec.clone()))
    }
}

pub trait VecImage {
    const IMAGE_LEN: usize = 2; // heap words in image

    fn image(_: &VectorImage) -> Vec<[u8; 8]>;
    fn evict(&self, _: &Env) -> Tag;
    fn ref_(_: &Env, _: Tag, _: usize) -> Option<Tag>;
    fn gc_ref(_: &mut Gc, _: Tag, _: usize) -> Option<Tag>;
}

impl<'a> VecImage for VecImageType<'a> {
    fn image(image: &VectorImage) -> Vec<[u8; 8]> {
        let slices = vec![image.type_.as_slice(), image.length.as_slice()];

        slices
    }

    fn evict(&self, env: &Env) -> Tag {
        let mut heap_ref = block_on(env.heap.write());

        let image_id = match self {
            VecImageType::Byte(image, ivec) => {
                let data = match ivec {
                    VectorImageType::Byte(vec_u8) => &vec_u8[..],
                    _ => panic!(),
                };

                heap_ref
                    .alloc(&Self::image(image), Some(data), Type::Vector as u8)
                    .unwrap() as u64
            }
            VecImageType::Bit(image, ivec) => {
                let data = match ivec {
                    VectorImageType::Bit(vec_u8) => &vec_u8[..],
                    _ => panic!(),
                };

                heap_ref
                    .alloc(&Self::image(image), Some(data), Type::Vector as u8)
                    .unwrap() as u64
            }
            VecImageType::Char(image, ivec) => {
                let data = match ivec {
                    VectorImageType::Char(string) => string.as_bytes(),
                    _ => panic!(),
                };

                heap_ref
                    .alloc(&Self::image(image), Some(data), Type::Vector as u8)
                    .unwrap() as u64
            }
            VecImageType::T(image, ivec) => {
                let mut slices = Self::image(image);

                match ivec {
                    VectorImageType::T(vec) => {
                        slices.extend(vec.iter().map(|tag| tag.as_slice()));
                    }
                    _ => panic!(),
                }

                heap_ref.alloc(&slices, None, Type::Vector as u8).unwrap() as u64
            }
            VecImageType::Fixnum(image, ivec) => {
                let mut slices = Self::image(image);

                match ivec {
                    VectorImageType::Fixnum(vec) => {
                        slices.extend(vec.iter().map(|n| n.to_le_bytes()));
                    }
                    _ => panic!(),
                }

                heap_ref.alloc(&slices, None, Type::Vector as u8).unwrap() as u64
            }
            VecImageType::Float(image, ivec) => {
                let data = match ivec {
                    VectorImageType::Float(vec_u4) => {
                        let mut vec_u8 = Vec::<u8>::new();
                        for float in vec_u4 {
                            let slice = float.to_le_bytes();

                            vec_u8.extend(slice.iter());
                        }
                        vec_u8
                    }
                    _ => panic!(),
                };

                heap_ref
                    .alloc(&Self::image(image), Some(&data), Type::Vector as u8)
                    .unwrap() as u64
            }
        };

        Tag::Indirect(
            IndirectTag::new()
                .with_image_id(image_id)
                .with_heap_id(1)
                .with_tag(TagType::Vector),
        )
    }

    fn gc_ref(gc: &mut Gc, vector: Tag, index: usize) -> Option<Tag> {
        let image = Vector::gc_ref_image(&mut gc.lock, vector);

        if index >= Fixnum::as_i64(image.length) as usize {
            return None;
        }

        let vimage = match vector {
            Tag::Indirect(image) => image,
            _ => panic!(),
        };

        match Vector::to_type(image.type_).unwrap() {
            Type::Byte => {
                let slice = gc
                    .lock
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index, 1)
                    .unwrap();

                Some(slice[0].into())
            }
            Type::Char => {
                let slice = gc
                    .lock
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index, 1)
                    .unwrap();

                let ch: char = slice[0].into();

                Some(ch.into())
            }
            Type::T => Some(Tag::from_slice(
                gc.lock
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index * 8, 8)
                    .unwrap(),
            )),
            Type::Fixnum => {
                let slice = gc
                    .lock
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index * 8, 8)
                    .unwrap();

                Some(Fixnum::with_i64_or_panic(i64::from_le_bytes(
                    slice[0..8].try_into().unwrap(),
                )))
            }
            Type::Float => {
                let slice = gc
                    .lock
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index * 4, 4)
                    .unwrap();

                Some(f32::from_le_bytes(slice[0..4].try_into().unwrap()).into())
            }
            _ => panic!(),
        }
    }

    fn ref_(env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        let image = Vector::to_image(env, vector);

        if index >= Fixnum::as_i64(image.length) as usize {
            return None;
        }

        let heap_ref = block_on(env.heap.read());

        let vimage = match vector {
            Tag::Indirect(image) => image,
            _ => panic!(),
        };

        match Vector::to_type(image.type_).unwrap() {
            Type::Bit => {
                let byte_index = index / 8;
                let slice = heap_ref
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, byte_index, 1)
                    .unwrap();

                let bit_index = 7 - (index % 8);

                Some(((slice[0] >> bit_index) & 1).into())
            }
            Type::Byte => {
                let slice = heap_ref
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index, 1)
                    .unwrap();

                Some(slice[0].into())
            }
            Type::Char => {
                let slice = heap_ref
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index, 1)
                    .unwrap();

                let ch: char = slice[0].into();

                Some(ch.into())
            }
            Type::T => Some(Tag::from_slice(
                heap_ref
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index * 8, 8)
                    .unwrap(),
            )),
            Type::Fixnum => {
                let slice = heap_ref
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index * 8, 8)
                    .unwrap();

                Some(Fixnum::with_i64_or_panic(i64::from_le_bytes(
                    slice[0..8].try_into().unwrap(),
                )))
            }
            Type::Float => {
                let slice = heap_ref
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index * 4, 4)
                    .unwrap();

                Some(f32::from_le_bytes(slice[0..4].try_into().unwrap()).into())
            }
            _ => panic!(),
        }
    }
}

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

impl<'a> Core<'a> for Vector {
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
