//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env allocators
//!    Env
use futures::executor::block_on;
use {
    crate::{
        allocators::{bump_allocator::BumpAllocator, stack_allocator::StackAllocator},
        core::{
            env::Env,
            heap::Heap,
            types::{Tag, Type},
        },
    },
    modular_bitfield::specifiers::{B11, B4},
    std::fmt,
};

#[bitfield]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct AllocatorImageInfo {
    pub reloc: u32, // relocation
    #[skip]
    __: B11, // expansion
    pub mark: bool, // reference counting
    pub len: u16,   // in bytes
    pub image_type: B4, // tag type
}

impl Default for AllocatorImageInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AllocatorImageInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "mark: {}, len: {} type: {}",
            self.mark(),
            (self.len() / 8) - 1,
            self.image_type()
        )
    }
}

pub enum AllocatorTypes {
    Bump(BumpAllocator),
    Stack(StackAllocator),
}

#[derive(Debug, Copy, Clone)]
pub struct AllocTypeInfo {
    pub size: usize,
    pub total: usize,
    pub free: usize,
}

pub trait Allocator {
    fn alloc(&mut self, _: &[[u8; 8]], _: Type) -> usize;
    fn image_info(&self, _: Tag) -> Option<AllocatorImageInfo>;
    fn is_marked(&self, _: Tag) -> Option<bool>;
    fn mark_image(&self, _: &Env, tag: Tag) -> Option<bool>;
    fn clear(&self);
    fn sweep(&self);
}

impl Allocator for Heap {
    fn alloc(&mut self, data: &[[u8; 8]], type_: Type) -> usize {
        (self.allocator).alloc(data, None, type_ as u8).unwrap()
    }

    fn image_info(&self, tag: Tag) -> Option<AllocatorImageInfo> {
        match tag {
            Tag::Direct(_) => None,
            Tag::Indirect(indirect) => (self.allocator).image_info(indirect.image_id() as usize),
        }
    }

    fn is_marked(&self, tag: Tag) -> Option<bool> {
        match tag {
            Tag::Direct(_) => None,
            Tag::Indirect(_) => self.image_info(tag).map(|info| info.mark()),
        }
    }

    fn mark_image(&self, env: &Env, tag: Tag) -> Option<bool> {
        match tag {
            Tag::Direct(_) => None,
            Tag::Indirect(indirect) => match self.image_info(tag) {
                None => None,
                Some(info) => {
                    let mark = info.mark();

                    if !mark {
                        let mut heap_ref = block_on(env.heap.write());
                        heap_ref.set_image_mark(indirect.image_id() as usize)
                    }

                    Some(mark)
                }
            },
        }
    }

    fn clear(&self) {}
    fn sweep(&self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn env() {
        assert_eq!(2 + 2, 4);
    }
}
