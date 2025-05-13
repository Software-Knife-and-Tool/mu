//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image management
#![allow(dead_code)]
use crate::core::env::Env;
use futures_lite::future::block_on;

pub struct Image {
    image: Vec<u8>,
}

impl Image {
    pub fn image(env: &Env) -> (Vec<u8>, Vec<u8>) {
        let heap_ref = block_on(env.heap.write());
        let image = heap_ref.heap_slice();

        (image.to_vec(), vec![])
    }
}

#[cfg(test)]
mod tests {}
