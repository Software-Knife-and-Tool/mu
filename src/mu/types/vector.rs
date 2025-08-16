//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// vector type
use {
    crate::{
        core::{
            direct::DirectTag,
            direct::DirectType,
            env::Env,
            exception,
            type_image::TypeImage,
            types::{Tag, Type},
        },
        types::{fixnum::Fixnum, symbol::Symbol},
        vectors::{
            image::{VecImage, VecImageType, VectorImage, VectorImageType},
            read::Read,
            write::Write,
        },
    },
    futures_lite::future::block_on,
    std::str,
};

lazy_static! {
    pub static ref VTYPEMAP: Vec<(Tag, Type)> = vec![
        (Symbol::keyword("bit"), Type::Bit),
        (Symbol::keyword("byte"), Type::Byte),
        (Symbol::keyword("char"), Type::Char),
        (Symbol::keyword("fixnum"), Type::Fixnum),
        (Symbol::keyword("float"), Type::Float),
        (Symbol::keyword("t"), Type::T),
    ];
}

#[derive(Clone)]
pub enum Vector {
    Direct(Tag),
    Indirect(VectorImage, VectorImageType),
}

impl Vector {
    const IMAGE_LEN: usize = 2; // heap words in image

    pub fn to_type(keyword: Tag) -> Option<Type> {
        VTYPEMAP
            .iter()
            .copied()
            .find(|tab| keyword.eq_(&tab.0))
            .map(|tab| tab.1)
    }

    pub fn type_of(env: &Env, vector: Tag) -> Type {
        match vector {
            Tag::Image(_) => panic!(),
            Tag::Direct(direct) => match direct.dtype() {
                DirectType::String => Type::Char,
                DirectType::ByteVec => Type::Byte,
                _ => panic!(),
            },
            Tag::Indirect(_) => {
                let image = Self::to_image(env, vector);

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

    pub fn length(env: &Env, vector: Tag) -> usize {
        match vector {
            Tag::Image(_) => panic!(),
            Tag::Direct(direct) => direct.ext() as usize,
            Tag::Indirect(_) => {
                let image = Self::to_image(env, vector);
                Fixnum::as_i64(image.length) as usize
            }
        }
    }

    pub fn view(env: &Env, vector: Tag) -> Tag {
        let vec = vec![
            Fixnum::with_or_panic(Self::length(env, vector)),
            match Tag::type_key(Self::type_of(env, vector)) {
                Some(key) => key,
                None => panic!(),
            },
        ];

        Vector::from(vec).evict(env)
    }

    pub fn as_string(env: &Env, tag: Tag) -> String {
        assert_eq!(tag.type_of(), Type::Vector);
        match tag {
            Tag::Image(_) => panic!(),
            Tag::Direct(dir) => match dir.dtype() {
                DirectType::String => {
                    str::from_utf8(&dir.data().to_le_bytes()).unwrap()[..dir.ext() as usize].into()
                }
                _ => panic!(),
            },
            Tag::Indirect(image) => {
                let heap_ref = block_on(env.heap.read());
                let vec: VectorImage = Self::to_image(env, tag);

                str::from_utf8(
                    heap_ref
                        .image_data_slice(
                            image.image_id() as usize + Self::IMAGE_LEN,
                            0,
                            Fixnum::as_i64(vec.length) as usize,
                        )
                        .unwrap(),
                )
                .unwrap()
                .into()
            }
        }
    }

    pub fn iter(env: &Env, vec: Tag) -> VectorIter<'_> {
        VectorIter { env, vec, index: 0 }
    }

    pub fn ref_(env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        assert_eq!(vector.type_of(), Type::Vector);

        match vector {
            Tag::Image(_) => panic!(),
            Tag::Direct(direct) => match direct.dtype() {
                DirectType::String => {
                    let ch: char = vector.data(env).to_le_bytes()[index].into();

                    Some(ch.into())
                }
                DirectType::ByteVec => {
                    let byte: u8 = vector.data(env).to_le_bytes()[index];

                    Some(byte.into())
                }
                _ => panic!(),
            },
            Tag::Indirect(_) => VecImageType::ref_(env, vector, index),
        }
    }

    pub fn to_image(env: &Env, tag: Tag) -> VectorImage {
        assert_eq!(tag.type_of(), Type::Vector);

        let heap_ref = block_on(env.heap.read());

        match tag {
            Tag::Indirect(image) => VectorImage {
                type_: Tag::from_slice(heap_ref.image_slice(image.image_id() as usize).unwrap()),
                length: Tag::from_slice(
                    heap_ref.image_slice(image.image_id() as usize + 1).unwrap(),
                ),
            },
            _ => panic!(),
        }
    }

    pub fn to_image_tag(&self, env: &Env) -> Tag {
        let image = TypeImage::Vector((self.clone(), VectorImageType::T(vec![])));

        TypeImage::to_tag(&image, env, Type::Vector as u8)
    }

    pub fn heap_size(env: &Env, vector: Tag) -> usize {
        match vector {
            Tag::Image(_) => panic!(),
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

    pub fn evict(&self, env: &Env) -> Tag {
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

    pub fn read(env: &Env, syntax: char, stream: Tag) -> exception::Result<Tag> {
        <Vector as Read>::read(env, syntax, stream)
    }

    pub fn write(env: &Env, vector: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        <Vector as Write>::write(env, vector, escape, stream)
    }
}

// iterator
pub struct VectorIter<'a> {
    env: &'a Env,
    pub vec: Tag,
    pub index: usize,
}

impl<'a> VectorIter<'a> {
    #[allow(dead_code)]
    pub fn new(env: &'a Env, vec: Tag) -> Self {
        Self { env, vec, index: 0 }
    }
}

impl Iterator for VectorIter<'_> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= Vector::length(self.env, self.vec) {
            None
        } else {
            let el = Vector::ref_(self.env, self.vec, self.index).unwrap();

            self.index += 1;

            Some(el)
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
