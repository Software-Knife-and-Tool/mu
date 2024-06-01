//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! typed vectors
use {
    crate::{
        core::{
            direct::{DirectInfo, DirectTag, DirectType},
            env::Env,
            exception,
            gc::{Gc, HeapGcRef},
            indirect::IndirectTag,
            types::{Tag, TagType, Type},
        },
        streams::write::Core as _,
        types::{
            core_stream::{Core as _, Stream},
            fixnum::{Core as _, Fixnum},
            symbol::{Core as _, Symbol},
            vector::{Vector, VectorIter},
        },
    },
    futures::executor::block_on,
    std::str,
};

impl From<Vec<Tag>> for Vector {
    fn from(vec: Vec<Tag>) -> Vector {
        let image = VectorImage {
            vtype: Symbol::keyword("t"),
            length: Fixnum::with_or_panic(vec.len()),
        };

        Vector::Indirect(image, IndirectType::T(vec.to_vec()))
    }
}

impl From<&str> for Vector {
    fn from(str: &str) -> Vector {
        let len = str.len();

        if len > DirectTag::DIRECT_STR_MAX {
            let image = VectorImage {
                vtype: Symbol::keyword("char"),
                length: Fixnum::with_or_panic(str.len()),
            };

            Vector::Indirect(image, IndirectType::Char(str.to_string()))
        } else {
            let mut data: [u8; 8] = 0_u64.to_le_bytes();

            for (src, dst) in str.as_bytes().iter().zip(data.iter_mut()) {
                *dst = *src
            }

            Vector::Direct(DirectTag::to_direct(
                u64::from_le_bytes(data),
                DirectInfo::Length(len),
                DirectType::ByteVector,
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
            vtype: Symbol::keyword("fixnum"),
            length: Fixnum::with_or_panic(vec.len()),
        };

        Vector::Indirect(image, IndirectType::Fixnum(vec.to_vec()))
    }
}

impl From<Vec<u8>> for Vector {
    fn from(vec: Vec<u8>) -> Vector {
        let image = VectorImage {
            vtype: Symbol::keyword("byte"),
            length: Fixnum::with_or_panic(vec.len()),
        };

        Vector::Indirect(image, IndirectType::Byte(vec.to_vec()))
    }
}

impl From<Vec<f32>> for Vector {
    fn from(vec: Vec<f32>) -> Vector {
        let image = VectorImage {
            vtype: Symbol::keyword("float"),
            length: Fixnum::with_or_panic(vec.len()),
        };

        Vector::Indirect(image, IndirectType::Float(vec.to_vec()))
    }
}

pub struct VectorImage {
    pub vtype: Tag,  // type keyword
    pub length: Tag, // fixnum
}

pub enum IndirectType {
    Byte(Vec<u8>),
    Char(String),
    Fixnum(Vec<i64>),
    Float(Vec<f32>),
    T(Vec<Tag>),
}

// vector types
#[allow(dead_code)]
pub enum IndirectVector<'a> {
    Byte(&'a VectorImage, &'a IndirectType),
    Char(&'a VectorImage, &'a IndirectType),
    Fixnum(&'a VectorImage, &'a IndirectType),
    Float(&'a VectorImage, &'a IndirectType),
    T(&'a VectorImage, &'a IndirectType),
}

pub trait IndirectVectorType {
    const IMAGE_LEN: usize = 2; // heap words in image

    fn image(_: &VectorImage) -> Vec<[u8; 8]>;
    fn evict(&self, _: &Env) -> Tag;
    fn ref_heap(_: &Env, _: Tag, _: usize) -> Option<Tag>;
    fn gc_ref(_: &mut Gc, _: Tag, _: usize) -> Option<Tag>;
}

impl<'a> IndirectVectorType for IndirectVector<'a> {
    fn image(image: &VectorImage) -> Vec<[u8; 8]> {
        let slices = vec![image.vtype.as_slice(), image.length.as_slice()];

        slices
    }

    fn evict(&self, env: &Env) -> Tag {
        let mut heap_ref = block_on(env.heap.write());

        let image_id = match self {
            IndirectVector::Byte(image, ivec) => {
                let data = match ivec {
                    IndirectType::Byte(vec_u8) => &vec_u8[..],
                    _ => panic!(),
                };

                heap_ref
                    .alloc(&Self::image(image), Some(data), Type::Vector as u8)
                    .unwrap() as u64
            }
            IndirectVector::Char(image, ivec) => {
                let data = match ivec {
                    IndirectType::Char(string) => string.as_bytes(),
                    _ => panic!(),
                };

                heap_ref
                    .alloc(&Self::image(image), Some(data), Type::Vector as u8)
                    .unwrap() as u64
            }
            IndirectVector::T(image, ivec) => {
                let mut slices = Self::image(image);

                match ivec {
                    IndirectType::T(vec) => {
                        slices.extend(vec.iter().map(|tag| tag.as_slice()));
                    }
                    _ => panic!(),
                }

                heap_ref.alloc(&slices, None, Type::Vector as u8).unwrap() as u64
            }
            IndirectVector::Fixnum(image, ivec) => {
                let mut slices = Self::image(image);

                match ivec {
                    IndirectType::Fixnum(vec) => {
                        slices.extend(vec.iter().map(|n| n.to_le_bytes()));
                    }
                    _ => panic!(),
                }

                heap_ref.alloc(&slices, None, Type::Vector as u8).unwrap() as u64
            }
            IndirectVector::Float(image, ivec) => {
                let data = match ivec {
                    IndirectType::Float(vec_u4) => {
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

        match Vector::to_type(image.vtype).unwrap() {
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

    fn ref_heap(env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        let image = Vector::to_image(env, vector);

        if index >= Fixnum::as_i64(image.length) as usize {
            return None;
        }

        let heap_ref = block_on(env.heap.read());

        let vimage = match vector {
            Tag::Indirect(image) => image,
            _ => panic!(),
        };

        match Vector::to_type(image.vtype).unwrap() {
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
                    vtype: Tag::from_slice(
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
                    vtype: Tag::from_slice(
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
                    IndirectType::T(_) => IndirectVector::T(image, ivec),
                    IndirectType::Char(_) => IndirectVector::Char(image, ivec),
                    IndirectType::Byte(_) => IndirectVector::Byte(image, ivec),
                    IndirectType::Fixnum(_) => IndirectVector::Fixnum(image, ivec),
                    IndirectType::Float(_) => IndirectVector::Float(image, ivec),
                };

                match ivec {
                    IndirectType::T(_) => indirect.evict(env),
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
            Tag::Direct(_) => match str::from_utf8(&vector.data(env).to_le_bytes()) {
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
            Tag::Indirect(_) => match Self::type_of(env, vector) {
                Type::Char => {
                    if escape {
                        env.write_string("\"", stream)?;
                    }

                    for ch in VectorIter::new(env, vector) {
                        env.write_stream(ch, false, stream)?;
                    }

                    if escape {
                        env.write_string("\"", stream)?;
                    }

                    Ok(())
                }
                _ => {
                    env.write_string("#(", stream)?;
                    env.write_stream(Self::to_image(env, vector).vtype, true, stream)?;

                    for tag in VectorIter::new(env, vector) {
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
