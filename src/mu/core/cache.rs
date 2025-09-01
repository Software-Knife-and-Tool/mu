//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// image cache
#[rustfmt::skip]
use {
    crate::{
        core::{
            env::Env,
            image::Image,
            tag::Tag,
            type_::Type
        },
        types::{
            async_::Async,
            cons::Cons,
            function::Function,
            struct_::Struct,
            symbol::Symbol,
            vector::Vector,
        },
    },
    futures_lite::future::block_on,
    std::collections::HashMap,
};

lazy_static! {
    pub static ref CACHETYPEMAP: Vec::<(Type, CacheId)> = vec![
        (Type::Async, CacheId::Async),
        (Type::Cons, CacheId::Cons),
        (Type::Function, CacheId::Function),
        (Type::Struct, CacheId::Struct),
        (Type::Symbol, CacheId::Symbol),
        (Type::Vector, CacheId::Vector),
    ];
}

#[derive(Clone)]
pub struct Cache {
    pub tag_id: u64,
    pub cache: HashMap<u64, Image>,
    pub type_info: [CacheTypeInfo; Cache::NCACHETYPES],
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CacheId {
    Async,
    Cons,
    Function,
    Struct,
    Symbol,
    Vector,
}

#[derive(Debug, Copy, Clone)]
pub struct CacheTypeInfo {
    pub size: usize,
    pub total: usize,
}

impl Cache {
    const NCACHETYPES: usize = 6;

    pub fn new() -> Self {
        Cache {
            tag_id: 0,
            cache: HashMap::new(),
            type_info: [CacheTypeInfo { size: 0, total: 0 }; Self::NCACHETYPES],
        }
    }

    pub fn map_type(id: CacheId) -> Type {
        CACHETYPEMAP
            .iter()
            .copied()
            .find(|map| id == map.1)
            .map(|map| map.0)
            .unwrap()
    }

    pub fn map_cache_id(type_: Type) -> Option<CacheId> {
        CACHETYPEMAP
            .iter()
            .copied()
            .find(|map| type_ == map.0)
            .map(|map| map.1)
    }

    pub fn add(env: &Env, image: Image) -> u64 {
        let mut images_ref = block_on(env.cache.write());
        let tag_id = images_ref.tag_id;
        let type_info = &mut images_ref.type_info;
        let cache_id = Self::map_cache_id(image.type_of()).unwrap();

        let image_size = match image.type_of() {
            Type::Async => std::mem::size_of::<Async>(),
            Type::Cons => std::mem::size_of::<Cons>(),
            Type::Function => std::mem::size_of::<Function>(),
            Type::Struct => std::mem::size_of::<Struct>(),
            Type::Symbol => std::mem::size_of::<Symbol>(),
            Type::Vector => std::mem::size_of::<Vector>(),
            _ => panic!(),
        };

        type_info[cache_id as usize].total += 1;
        type_info[cache_id as usize].size += image_size;

        images_ref.tag_id += 1;
        images_ref.cache.insert(tag_id, image);

        tag_id
    }

    pub fn update(env: &Env, image: Image, tag: Tag) {
        let (index, _) = Image::detag(tag);
        let mut image_ref = block_on(env.cache.write());

        image_ref.cache.insert(index as u64, image);
    }

    pub fn ref_(env: &Env, index: usize) -> Image {
        let images_ref = block_on(env.cache.read());

        images_ref.cache[&(index as u64)].clone()
    }

    pub fn type_info(env: &Env, type_: Type) -> Option<CacheTypeInfo> {
        let images_ref = block_on(env.cache.read());

        Self::map_cache_id(type_).map(|cache_id| images_ref.type_info[cache_id as usize])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
