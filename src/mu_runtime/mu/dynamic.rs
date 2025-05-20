//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! function call frame
//!    Frame
//!    apply
//!    frame_push
//!    frame_pop
//!    frame_ref
use crate::mu::{env::Env, type_image::TypeImage, types::Tag};

use {futures_lite::future::block_on, futures_locks::RwLock};

pub struct Dynamic {
    pub dynamic: RwLock<Vec<(u64, usize)>>,
    pub images: RwLock<Vec<TypeImage>>,
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
            images: RwLock::new(Vec::<TypeImage>::new()),
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

    pub fn images_push(env: &Env, image: TypeImage) -> usize {
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
    pub fn images_ref(env: &Env, index: usize) -> TypeImage {
        let images_ref = block_on(env.dynamic.images.read());

        images_ref[index].clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dynamic() {
        assert_eq!(2 + 2, 4);
    }
}
