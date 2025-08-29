//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// type image cache
use {
    crate::core::{env::Env, image::Image, tag::Tag},
    futures_lite::future::block_on,
};

#[derive(Clone)]
pub struct ImageCache {
    pub cache: Vec<Image>,
    // pub type_map: Vec<Vec<Image>>,
}

impl Default for ImageCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageCache {
    pub fn new() -> Self {
        ImageCache {
            cache: Vec::new(),
            // type_map: Vec::new(),
        }

        /*
        cache.type_map.reserve_exact(6);
        cache.type_map.extend_from_slice(&[vec![], vec![], vec![], vec![], vec![], vec![]]);

        cache
        */
    }

    pub fn push(env: &Env, image: Image) -> usize {
        let mut images_ref = block_on(env.image_cache.write());
        let offset = images_ref.cache.len();

        /*
        match image {
            Image::Async(_) => images_ref.types[0].push(image.clone()),
            Image::Cons(_) => images_ref.types[1].push(image.clone()),
            Image::Function(_) => images_ref.types[2].push(image.clone()),
            Image::Struct(_) => images_ref.types[3].push(image.clone()),
            Image::Symbol(_) => images_ref.types[4].push(image.clone()),
            Image::Vector(_) => images_ref.types[5].push(image.clone()),
        }
         */

        images_ref.cache.push(image);

        offset
    }

    pub fn pop(env: &Env) {
        let mut images_ref = block_on(env.image_cache.write());

        images_ref.cache.pop();
    }

    pub fn update(env: &Env, image: Image, tag: Tag) {
        let (index, _) = Image::detag(tag);
        let mut image_ref = block_on(env.image_cache.write());

        image_ref.cache[index] = image
    }

    #[allow(dead_code)]
    pub fn ref_(env: &Env, index: usize) -> Image {
        let images_ref = block_on(env.image_cache.read());

        images_ref.cache[index].clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
