//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use json::{self, object};

pub struct BumpAllocatorImage {
    pub image: Vec<u8>,
    pub alloc_map: Vec<u8>,
    pub free_map: Vec<u8>,
    pub write_barrier: Vec<u8>,
}

impl BumpAllocatorImage {
    pub fn to_json(&self) -> Option<String> {
        let allocator = object! {
            image: self.image.clone(),
            alloc_map: self.alloc_map.clone(),
            free_map: self.free_map.clone(),
            write_barrier: self.write_barrier.clone(),
        };

        Some(allocator.dump())
    }

    /*
        pub fn from_json(_json: String) -> Option<Self> {
            None
    }
        */
}

pub struct BumpAllocatorImageBuilder {
    image: Option<Vec<u8>>,
    alloc_map: Option<Vec<u8>>,
    free_map: Option<Vec<u8>>,
    write_barrier: Option<Vec<u8>>,
}

impl BumpAllocatorImageBuilder {
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

    pub fn alloc_map(&mut self, map: Vec<Vec<u8>>) -> &mut Self {
        let mut vec: Vec<u8> = vec![];

        for context in map {
            vec.extend(context)
        }

        self.alloc_map = Some(vec);
        self
    }

    pub fn free_map(&mut self, map: Vec<Vec<u8>>) -> &mut Self {
        let mut vec: Vec<u8> = vec![];

        for context in map {
            vec.extend(context)
        }

        self.free_map = Some(vec);
        self
    }

    pub fn write_barrier(&mut self, write_barrier: usize) -> &mut Self {
        self.write_barrier = Some(write_barrier.to_le_bytes().to_vec());
        self
    }

    pub fn build(&self) -> BumpAllocatorImage {
        BumpAllocatorImage {
            image: match &self.image {
                Some(vec) => vec.to_vec(),
                None => vec![],
            },
            alloc_map: match &self.alloc_map {
                Some(vec) => vec.to_vec(),
                None => vec![],
            },
            free_map: match &self.free_map {
                Some(vec) => vec.to_vec(),
                None => vec![],
            },
            write_barrier: match &self.write_barrier {
                Some(vec) => vec.to_vec(),
                None => vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            },
        }
    }
}

#[cfg(test)]
mod tests {}
