//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! typed vectors
use {
    crate::{
        core::{
            direct::{DirectInfo, DirectTag, DirectType},
            env::Env,
            gc::Gc,
            indirect::IndirectTag,
            types::{Tag, TagType, Type},
        },
        types::{
            fixnum::Fixnum,
            symbol::{Core as _, Symbol},
            vector::{Core, Vector},
        },
    },
    futures::executor::block_on,
};

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
    fn ref_heap(_: &Env, _: Tag, _: usize) -> Option<Tag>;
    fn gc_ref(_: &mut Gc, _: Tag, _: usize) -> Option<Tag>;
}

impl<'a> IVector for IndirectVector<'a> {
    fn image(image: &VectorImage) -> Vec<[u8; 8]> {
        let slices = vec![image.vtype.as_slice(), image.length.as_slice()];

        slices
    }

    fn evict(&self, env: &Env) -> Tag {
        let mut heap_ref = block_on(env.heap.write());

        let image_id = match self {
            IndirectVector::Byte(image, ivec) => {
                let data = match ivec {
                    IVec::Byte(vec_u8) => &vec_u8[..],
                    _ => panic!(),
                };

                heap_ref
                    .alloc(&Self::image(image), Some(data), Type::Vector as u8)
                    .unwrap() as u64
            }
            IndirectVector::Char(image, ivec) => {
                let data = match ivec {
                    IVec::Char(string) => string.as_bytes(),
                    _ => panic!(),
                };

                heap_ref
                    .alloc(&Self::image(image), Some(data), Type::Vector as u8)
                    .unwrap() as u64
            }
            IndirectVector::T(image, ivec) => {
                let mut slices = Self::image(image);

                match ivec {
                    IVec::T(vec) => {
                        slices.extend(vec.iter().map(|tag| tag.as_slice()));
                    }
                    _ => panic!(),
                }

                heap_ref.alloc(&slices, None, Type::Vector as u8).unwrap() as u64
            }
            IndirectVector::Fixnum(image, ivec) => {
                let mut slices = Self::image(image);

                match ivec {
                    IVec::Fixnum(vec) => {
                        slices.extend(vec.iter().map(|n| n.to_le_bytes()));
                    }
                    _ => panic!(),
                }

                heap_ref.alloc(&slices, None, Type::Vector as u8).unwrap() as u64
            }
            IndirectVector::Float(image, ivec) => {
                let data = match ivec {
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

                Some(Tag::from(slice[0] as i64))
            }
            Type::Char => {
                let slice = gc
                    .lock
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index, 1)
                    .unwrap();

                Some(Tag::from(slice[0] as char))
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

                Some(Tag::from(i64::from_le_bytes(
                    slice[0..8].try_into().unwrap(),
                )))
            }
            Type::Float => {
                let slice = gc
                    .lock
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index * 4, 4)
                    .unwrap();

                Some(Tag::from(f32::from_le_bytes(
                    slice[0..4].try_into().unwrap(),
                )))
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

                Some(Tag::from(slice[0] as i64))
            }
            Type::Char => {
                let slice = heap_ref
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index, 1)
                    .unwrap();

                Some(Tag::from(slice[0] as char))
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

                Some(Tag::from(i64::from_le_bytes(
                    slice[0..8].try_into().unwrap(),
                )))
            }
            Type::Float => {
                let slice = heap_ref
                    .image_data_slice(vimage.image_id() as usize + Self::IMAGE_LEN, index * 4, 4)
                    .unwrap();

                Some(Tag::from(f32::from_le_bytes(
                    slice[0..4].try_into().unwrap(),
                )))
            }
            _ => panic!(),
        }
    }
}

// typed vector allocation
pub struct TypedVector<T: VecType> {
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
            let el = Vector::ref_heap(self.env, self.vec, self.index);
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
