//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image management
use crate::core::env::Env;
use futures::executor::block_on;

pub struct Image {
    image: Vec<u8>,
}

impl Image {}

pub trait Core {
    fn image(&self) -> Vec<u8>;
}

impl Core for Env {
    fn image(&self) -> Vec<u8> {
        let heap_ref = block_on(self.heap.write());
        let image = heap_ref.heap_slice();

        image.to_vec()
    }
}

#[cfg(test)]
mod tests {}
