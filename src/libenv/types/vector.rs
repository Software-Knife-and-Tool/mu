//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env image vector type
use crate::{
    core::{
        direct::{DirectInfo, DirectTag, DirectType},
        env::Env,
        indirect::IndirectTag,
        types::{Tag, TagType, Type},
    },
    types::{
        fixnum::Fixnum,
        symbol::{Core as _, Symbol},
        vectors::{Core, Vector},
    },
};

use futures::executor::block_on;

pub struct VectorImage {
    pub vtype: Tag,  // type keyword
    pub length: Tag, // fixnum
}

pub enum IVec {
    Byte(Vec<u8>),
    Char(String),
    Fixnum(Vec<i64>),
    Float(Vec<f32>),
    T(Vec<Tag>),
}

// vector types
#[allow(dead_code)]
pub enum IndirectVector<'a> {
    Byte(&'a VectorImage, &'a IVec),
    Char(&'a VectorImage, &'a IVec),
    Fixnum(&'a VectorImage, &'a IVec),
    Float(&'a VectorImage, &'a IVec),
    T(&'a VectorImage, &'a IVec),
}

pub trait IVector {
    const IMAGE_LEN: usize = 2; // heap words in image

    fn image(_: &VectorImage) -> Vec<[u8; 8]>;
    fn evict(&self, _: &Env) -> Tag;
    fn ref_(_: &Env, _: Tag, _: usize) -> Option<Tag>;
}

impl<'a> IVector for IndirectVector<'a> {
    fn image(image: &VectorImage) -> Vec<[u8; 8]> {
        let slices = vec![image.vtype.as_slice(), image.length.as_slice()];

        slices
    }

    fn evict(&self, env: &Env) -> Tag {
        match self {
            IndirectVector::Byte(image, ivec) => {
                let slices = Self::image(image);

                let data = match ivec {
                    IVec::Byte(vec_u8) => &vec_u8[..],
                    _ => panic!(),
                };

                let mut heap_ref = block_on(env.heap.write());

                Tag::Indirect(
                    IndirectTag::new()
                        .with_image_id(
                            heap_ref
                                .alloc(&slices, Some(data), Type::Vector as u8)
                                .unwrap() as u64,
                        )
                        .with_heap_id(1)
                        .with_tag(TagType::Vector),
                )
            }
            IndirectVector::Char(image, ivec) => {
                let slices = Self::image(image);

                let data = match ivec {
                    IVec::Char(string) => string.as_bytes(),
                    _ => panic!(),
                };

                let mut heap_ref = block_on(env.heap.write());

                Tag::Indirect(
                    IndirectTag::new()
                        .with_image_id(
                            heap_ref
                                .alloc(&slices, Some(data), Type::Vector as u8)
                                .unwrap() as u64,
                        )
                        .with_heap_id(1)
                        .with_tag(TagType::Vector),
                )
            }
            IndirectVector::T(image, vec) => {
                let mut slices = Self::image(image);

                match vec {
                    IVec::T(vec) => {
                        slices.extend(vec.iter().map(|tag| tag.as_slice()));
                    }
                    _ => panic!(),
                }

                let mut heap_ref = block_on(env.heap.write());

                Tag::Indirect(
                    IndirectTag::new()
                        .with_image_id(
                            heap_ref.alloc(&slices, None, Type::Vector as u8).unwrap() as u64
                        )
                        .with_heap_id(1)
                        .with_tag(TagType::Vector),
                )
            }
            IndirectVector::Fixnum(image, vec) => {
                let mut slices = Self::image(image);

                match vec {
                    IVec::Fixnum(vec) => {
                        slices.extend(vec.iter().map(|n| n.to_le_bytes()));
                    }
                    _ => panic!(),
                }

                let mut heap_ref = block_on(env.heap.write());

                Tag::Indirect(
                    IndirectTag::new()
                        .with_image_id(
                            heap_ref.alloc(&slices, None, Type::Vector as u8).unwrap() as u64
                        )
                        .with_heap_id(1)
                        .with_tag(TagType::Vector),
                )
            }
            IndirectVector::Float(image, vec) => {
                let data = match vec {
                    IVec::Float(vec_u4) => {
                        let mut vec_u8 = Vec::<u8>::new();
                        for float in vec_u4 {
                            let slice = float.to_le_bytes();

                            vec_u8.extend(slice.iter());
                        }
                        vec_u8
                    }
                    _ => panic!(),
                };

                let mut heap_ref = block_on(env.heap.write());

                Tag::Indirect(
                    IndirectTag::new()
                        .with_image_id(
                            heap_ref
                                .alloc(&Self::image(image), Some(&data), Type::Vector as u8)
                                .unwrap() as u64,
                        )
                        .with_heap_id(1)
                        .with_tag(TagType::Vector),
                )
            }
        }
    }

    fn ref_(env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        let image = Vector::to_image(env, vector);

        let len = Fixnum::as_i64(image.length) as usize;
        if index >= len {
            return None;
        }

        match Vector::to_type(image.vtype).unwrap() {
            Type::Byte => match vector {
                Tag::Indirect(image) => {
                    let heap_ref = block_on(env.heap.read());
                    let slice = heap_ref
                        .image_data_slice(image.image_id() as usize + Self::IMAGE_LEN, index, 1)
                        .unwrap();

                    Some(Tag::from(slice[0] as i64))
                }
                _ => panic!(),
            },
            Type::Char => match vector {
                Tag::Indirect(image) => {
                    let heap_ref = block_on(env.heap.read());
                    let slice = heap_ref
                        .image_data_slice(image.image_id() as usize + Self::IMAGE_LEN, index, 1)
                        .unwrap();

                    Some(Tag::from(slice[0] as char))
                }
                _ => panic!(),
            },
            Type::T => match vector {
                Tag::Indirect(image) => {
                    let heap_ref = block_on(env.heap.read());

                    Some(Tag::from_slice(
                        heap_ref
                            .image_data_slice(
                                image.image_id() as usize + Self::IMAGE_LEN,
                                index * 8,
                                8,
                            )
                            .unwrap(),
                    ))
                }
                _ => panic!(),
            },
            Type::Fixnum => match vector {
                Tag::Indirect(image) => {
                    let heap_ref = block_on(env.heap.read());
                    let slice = heap_ref
                        .image_data_slice(image.image_id() as usize + Self::IMAGE_LEN, index * 8, 8)
                        .unwrap();

                    Some(Tag::from(i64::from_le_bytes(
                        slice[0..8].try_into().unwrap(),
                    )))
                }
                _ => panic!(),
            },
            Type::Float => match vector {
                Tag::Indirect(image) => {
                    let heap_ref = block_on(env.heap.read());
                    let slice = heap_ref
                        .image_data_slice(image.image_id() as usize + Self::IMAGE_LEN, index * 4, 4)
                        .unwrap();

                    Some(Tag::from(f32::from_le_bytes(
                        slice[0..4].try_into().unwrap(),
                    )))
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

// typed vector allocation
pub struct TypedVec<T: VecType> {
    pub vec: T,
}

pub trait VecType {
    fn to_vector(&self) -> Vector;
}

impl VecType for String {
    fn to_vector(&self) -> Vector {
        let len = self.len();

        if len > DirectTag::DIRECT_STR_MAX {
            let image = VectorImage {
                vtype: Symbol::keyword("char"),
                length: Tag::from(self.len() as i64),
            };

            Vector::Indirect(image, IVec::Char(self.to_string()))
        } else {
            let mut data: [u8; 8] = 0u64.to_le_bytes();

            for (src, dst) in self.as_bytes().iter().zip(data.iter_mut()) {
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

impl VecType for Vec<Tag> {
    fn to_vector(&self) -> Vector {
        let image = VectorImage {
            vtype: Symbol::keyword("t"),
            length: Tag::from(self.len() as i64),
        };

        Vector::Indirect(image, IVec::T(self.to_vec()))
    }
}

impl VecType for Vec<i64> {
    fn to_vector(&self) -> Vector {
        let image = VectorImage {
            vtype: Symbol::keyword("fixnum"),
            length: Tag::from(self.len() as i64),
        };

        Vector::Indirect(image, IVec::Fixnum(self.to_vec()))
    }
}

impl VecType for Vec<u8> {
    fn to_vector(&self) -> Vector {
        let image = VectorImage {
            vtype: Symbol::keyword("byte"),
            length: Tag::from(self.len() as i64),
        };

        Vector::Indirect(image, IVec::Byte(self.to_vec()))
    }
}

impl VecType for Vec<f32> {
    fn to_vector(&self) -> Vector {
        let image = VectorImage {
            vtype: Symbol::keyword("float"),
            length: Tag::from(self.len() as i64),
        };

        Vector::Indirect(image, IVec::Float(self.to_vec()))
    }
}

// iterator
pub struct VectorIter<'a> {
    env: &'a Env,
    pub vec: Tag,
    pub index: usize,
}

impl<'a> VectorIter<'a> {
    pub fn new(env: &'a Env, vec: Tag) -> Self {
        Self { env, vec, index: 0 }
    }
}

impl<'a> Iterator for VectorIter<'a> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= Vector::length(self.env, self.vec) {
            None
        } else {
            let el = Vector::ref_(self.env, self.vec, self.index);
            self.index += 1;

            Some(el.unwrap())
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
