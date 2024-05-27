//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image management
use futures::executor::block_on;
use {
    crate::core::env::Env,
    //    object::elf,
};

pub struct Image {}

pub trait Core {
    fn image(_: &Env) -> Vec<u8>;
}

impl Core for Image {
    fn image(env: &Env) -> Vec<u8> {
        let heap_ref = block_on(env.heap.write());
        let image = heap_ref.heap_slice();

        image.to_vec()
    }
}

#[cfg(test)]
mod tests {}
