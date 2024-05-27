//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image management
use futures::executor::block_on;
use {
    crate::{allocators::bump_allocator::BumpAllocator, core::env::Env},
    object::{build::elf::Builder, Endianness},
};

// bump allocator elf builder
pub struct BumpAllocatorElfBuilder {
    pub image: Option<Vec<u8>>,
    pub alloc_map: Option<Vec<u8>>,
    pub free_map: Option<Vec<u8>>,
    pub write_barrier: Option<usize>,
}

impl BumpAllocatorElfBuilder {
    pub fn new() -> Self {
        Self {
            image: None,
            alloc_map: None,
            free_map: None,
            write_barrier: None,
        }
    }

    pub fn image(&mut self, image: Vec<u8>) -> &mut Self {
        self.image = Some(image);
        self
    }

    pub fn alloc_map(&mut self, _map: Vec<Vec<u8>>) -> &mut Self {
        self.alloc_map = Some(vec![]);
        self
    }

    pub fn free_map(&mut self, _map: Vec<Vec<u8>>) -> &mut Self {
        self.free_map = Some(vec![]);
        self
    }

    pub fn write_barrier(&mut self, write_barrier: usize) -> &mut Self {
        self.write_barrier = Some(write_barrier);
        self
    }

    pub fn build(&self) -> Option<()> {
        None
    }
}

pub trait Core {
    fn builder(_: &Env) -> Builder;
}

impl Core for BumpAllocator {
    fn builder(env: &Env) -> Builder {
        let heap_ref = block_on(env.heap.write());
        let _image = heap_ref.heap_slice();

        Builder::new(Endianness::Little, true)
    }
}

#[cfg(test)]
mod tests {}
