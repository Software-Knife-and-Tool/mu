//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// typed vectors
use {
    crate::{
        core::{
            direct::{DirectExt, DirectTag, DirectType},
            env::Env,
            indirect::IndirectTag,
            tag::{Tag, TagType},
            type_::Type,
        },
        namespaces::heap::HeapRequest,
        types::{
            fixnum::Fixnum,
            symbol::Symbol,
            vector::{Vector, VectorType},
        },
    },
    futures_lite::future::block_on,
    std::str,
};

#[derive(Clone)]
pub struct VectorImage {
    pub type_: Tag,  // type keyword
    pub length: Tag, // fixnum
}

#[derive(Clone)]
pub enum VectorImageType {
    Bit(Vec<u8>),
    Byte(Vec<u8>),
    Char(String),
    Fixnum(Vec<i64>),
    Float(Vec<f32>),
    T(Vec<Tag>),
}

// vector types
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
            length: Fixnum::with_usize_or_panic(vec.len()),
        };

        Vector::Indirect(image, VectorImageType::T(vec.clone()))
    }
}

impl From<&str> for Vector {
    fn from(str: &str) -> Vector {
        let len = str.len();

        if len > DirectTag::DIRECT_STR_MAX {
            let image = VectorImage {
                type_: Symbol::keyword("char"),
                length: Fixnum::with_usize_or_panic(str.len()),
            };

            Vector::Indirect(image, VectorImageType::Char(str.into()))
        } else {
            let mut data: [u8; 8] = 0_u64.to_le_bytes();

            for (src, dst) in str.as_bytes().iter().zip(data.iter_mut()) {
                *dst = *src;
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
                length: Fixnum::with_usize_or_panic(bytes.len()),
            };

            Vector::Indirect(image, VectorImageType::Byte(bytes.to_vec()))
        } else {
            let mut data: [u8; 8] = 0_u64.to_le_bytes();

            for (src, dst) in bytes.to_vec().iter().zip(data.iter_mut()) {
                *dst = *src;
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
            length: Fixnum::with_usize_or_panic(vec.len()),
        };

        Vector::Indirect(image, VectorImageType::Fixnum(vec.clone()))
    }
}

impl From<(Vec<u8>, usize)> for Vector {
    fn from(vec_def: (Vec<u8>, usize)) -> Vector {
        let (vec, len) = vec_def;

        let image = VectorImage {
            type_: Symbol::keyword("bit"),
            length: Fixnum::with_usize_or_panic(len),
        };

        Vector::Indirect(image, VectorImageType::Bit(vec))
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
            length: Fixnum::with_usize_or_panic(vec.len()),
        };

        Vector::Indirect(image, VectorImageType::Float(vec.clone()))
    }
}

pub trait VecImage {
    const IMAGE_LEN: usize = 2; // heap words in image

    fn image(_: &VectorImage) -> Vec<[u8; 8]>;
    fn with_heap(&self, _: &Env) -> Tag;
    fn ref_(_: &Env, _: Tag, _: usize) -> Option<Tag>;
}

impl VecImage for VecImageType<'_> {
    fn image(image: &VectorImage) -> Vec<[u8; 8]> {
        vec![image.type_.as_slice(), image.length.as_slice()]
    }

    fn with_heap(&self, env: &Env) -> Tag {
        let mut heap_ref = block_on(env.heap.write());
        let mut fvec = Vec::<u8>::new(); // extend lifetime of float slices

        let (image, vdata) = match self {
            VecImageType::Byte(image, ivec) => match ivec {
                VectorImageType::Byte(vec_u8) => (&Self::image(image), Some(&vec_u8[..])),
                _ => panic!(),
            },
            VecImageType::Bit(image, ivec) => match ivec {
                VectorImageType::Bit(vec_u8) => (&Self::image(image), Some(&vec_u8[..])),
                _ => panic!(),
            },
            VecImageType::Char(image, ivec) => match ivec {
                VectorImageType::Char(string) => (&Self::image(image), Some(string.as_bytes())),
                _ => panic!(),
            },
            VecImageType::T(image, ivec) => {
                let mut slices = Self::image(image);

                match ivec {
                    VectorImageType::T(vec) => {
                        slices.extend(vec.iter().map(Tag::as_slice));

                        (&slices.clone(), None)
                    }
                    _ => panic!(),
                }
            }
            VecImageType::Fixnum(image, ivec) => {
                let mut slices = Self::image(image);

                match ivec {
                    VectorImageType::Fixnum(vec) => {
                        slices.extend(vec.iter().map(|n| n.to_le_bytes()));

                        (&slices.clone(), None)
                    }
                    _ => panic!(),
                }
            }
            VecImageType::Float(image, ivec) => match ivec {
                VectorImageType::Float(vec_u4) => {
                    for float in vec_u4 {
                        let slice = float.to_le_bytes();

                        fvec.extend(slice.iter());
                    }

                    (&Self::image(image), Some(fvec.as_slice()))
                }
                _ => panic!(),
            },
        };

        let ha = HeapRequest {
            env,
            image,
            vdata,
            type_id: Type::Vector as u8,
        };

        let image_id = match heap_ref.alloc(&ha) {
            Some(id) => id as u64,
            None => panic!(),
        };

        Tag::Indirect(
            IndirectTag::new()
                .with_image_id(image_id)
                .with_heap_id(1)
                .with_tag(TagType::Vector),
        )
    }

    fn ref_(env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        assert!(vector.type_of() == Type::Vector);

        let Tag::Indirect(vimage) = vector else {
            panic!()
        };
        let len = usize::try_from(vimage.image_id()).unwrap() + Self::IMAGE_LEN;
        let image = Vector::to_image(env, vector);

        if index >= usize::try_from(Fixnum::as_i64(image.length)).unwrap() {
            None?;
        }

        let heap_ref = block_on(env.heap.read());

        match Vector::to_vectype(image.type_).unwrap() {
            VectorType::Bit(_) => {
                let slice = heap_ref.image_data_slice(len, index / 8, 1)?;
                let bit_index = 7 - (index % 8);

                Some(((slice[0] >> bit_index) & 1).into())
            }
            VectorType::Byte(_) => {
                let slice = heap_ref.image_data_slice(len, index, 1)?;

                Some(slice[0].into())
            }
            VectorType::Char(_) => {
                let slice = heap_ref.image_data_slice(len, index, 1)?;
                let ch: char = slice[0].into();

                Some(ch.into())
            }
            VectorType::T(_) => Some(Tag::from_slice(heap_ref.image_data_slice(
                len,
                index * 8,
                8,
            )?)),
            VectorType::Fixnum(_) => {
                let slice = heap_ref.image_data_slice(len, index * 8, 8)?;

                Some(Fixnum::with_i64_or_panic(i64::from_le_bytes(
                    slice[0..8].try_into().unwrap(),
                )))
            }
            VectorType::Float(_) => {
                let slice = heap_ref.image_data_slice(len, index * 4, 4)?;

                Some(f32::from_le_bytes(slice[0..4].try_into().unwrap()).into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
