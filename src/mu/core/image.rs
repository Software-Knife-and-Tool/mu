//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// type images
#[rustfmt::skip]
use {
    crate::{
        core::{
            direct::{DirectTag, DirectType, ExtType},
            env::Env,
            image_cache::ImageCache,
            tag::{Tag, TagType},
            type_::{Type},
        },
        types::{
            async_::Async,
            cons::Cons,
            function::Function,
            struct_::Struct,
            symbol::Symbol,
            vector::Vector,
        },
        vectors::image::VectorImageType,
    },
    futures_lite::future::block_on,
};

#[derive(Debug, Copy, Clone)]
pub struct ImageTypeInfo {
    pub size: usize,
    pub total: usize,
}

#[derive(Clone)]
pub enum Image {
    Async(Async),
    Cons(Cons),
    Function(Function),
    Struct(Struct),
    Symbol(Symbol),
    Vector((Vector, VectorImageType)),
}

impl From<Image> for Tag {
    fn from(_image: Image) -> Tag {
        Tag::nil()
    }
}

impl Image {
    pub fn to_tag(&self, env: &Env, type_id: u8) -> Tag {
        let offset = ImageCache::push(env, self.clone());
        let data = ((offset << 8) as u64) | ((type_id & 0xf) as u64);

        Tag::Image(
            DirectTag::new()
                .with_data(data)
                .with_ext(ExtType::Image as u8)
                .with_dtype(DirectType::Ext)
                .with_tag(TagType::Direct),
        )
    }

    pub fn detag(tag: Tag) -> (usize, u8) {
        match tag {
            Tag::Direct(fn_) | Tag::Image(fn_) => {
                let data = fn_.data() as usize;

                (data >> 8, data as u8)
            }
            _ => panic!(),
        }
    }

    pub fn type_of(&self) -> Type {
        match self {
            Self::Async(_) => Type::Async,
            Self::Cons(_) => Type::Cons,
            Self::Function(_) => Type::Function,
            Self::Struct(_) => Type::Struct,
            Self::Symbol(_) => Type::Symbol,
            Self::Vector(_) => Type::Vector,
        }
    }

    pub fn type_info(env: &Env, type_: Type) -> ImageTypeInfo {
        let images_ref = block_on(env.image_cache.read());
        let mut type_info = ImageTypeInfo { size: 0, total: 0 };

        for image in images_ref.cache.iter() {
            if type_ == image.type_of() {
                let image_size = match type_ {
                    Type::Async => std::mem::size_of::<Async>(),
                    Type::Cons => std::mem::size_of::<Cons>(),
                    Type::Function => std::mem::size_of::<Function>(),
                    Type::Struct => std::mem::size_of::<Struct>(),
                    Type::Symbol => std::mem::size_of::<Symbol>(),
                    Type::Vector => std::mem::size_of::<Vector>(),
                    _ => panic!(),
                };

                type_info.size += image_size;
                type_info.total += 1
            }
        }

        type_info
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
