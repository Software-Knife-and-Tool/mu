//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// dynamic allocations
use {
    crate::{
        core::{
            env::Env,
            image::Image,
            types::{Tag, Type},
        },
        types::{
            async_::Async, cons::Cons, function::Function, struct_::Struct, symbol::Symbol,
            vector::Vector,
        },
    },
    futures_lite::future::block_on,
    futures_locks::RwLock,
};

#[derive(Debug, Copy, Clone)]
pub struct DynamicTypeInfo {
    pub size: usize,
    pub total: usize,
}

pub struct Dynamic {
    pub dynamic: RwLock<Vec<(u64, usize)>>,
    pub images: RwLock<Vec<Image>>,
}

impl Default for Dynamic {
    fn default() -> Self {
        Self::new()
    }
}

impl Dynamic {
    pub fn new() -> Self {
        Self {
            dynamic: RwLock::new(Vec::<(u64, usize)>::new()),
            images: RwLock::new(Vec::<Image>::new()),
        }
    }

    pub fn dynamic_push(env: &Env, func: Tag, offset: usize) {
        let mut dynamic_ref = block_on(env.dynamic.dynamic.write());

        dynamic_ref.push((func.as_u64(), offset));
    }

    pub fn dynamic_pop(env: &Env) {
        let mut dynamic_ref = block_on(env.dynamic.dynamic.write());

        dynamic_ref.pop();
    }

    #[allow(dead_code)]
    pub fn dynamic_ref(env: &Env, index: usize) -> (Tag, usize) {
        let dynamic_ref = block_on(env.dynamic.dynamic.read());

        let (func, offset) = dynamic_ref[index];

        ((&func.to_le_bytes()).into(), offset)
    }

    pub fn images_type_info(env: &Env, type_: Type) -> DynamicTypeInfo {
        let images_ref = block_on(env.dynamic.images.read());
        let mut type_info = DynamicTypeInfo { size: 0, total: 0 };

        for image in images_ref.iter() {
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

    pub fn images_push(env: &Env, image: Image) -> usize {
        let mut images_ref = block_on(env.dynamic.images.write());

        let offset = images_ref.len();

        images_ref.push(image);

        offset
    }

    pub fn images_pop(env: &Env) {
        let mut images_ref = block_on(env.dynamic.images.write());

        images_ref.pop();
    }

    #[allow(dead_code)]
    pub fn images_ref(env: &Env, index: usize) -> Image {
        let images_ref = block_on(env.dynamic.images.read());

        images_ref[index].clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dynamic() {
        assert!(true);
    }
}
