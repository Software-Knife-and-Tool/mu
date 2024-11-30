//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env heap
use crate::heaps::allocator::{AllocTypeInfo, AllocatorImageInfo};
use {futures::executor::block_on, futures_locks::RwLock};

#[derive(Debug)]
pub struct StackAllocator {
    pub stack: RwLock<Vec<u64>>,
    pub alloc_map: RwLock<Vec<RwLock<AllocTypeInfo>>>,
}

impl StackAllocator {
    pub fn new() -> Self {
        let heap = StackAllocator {
            stack: RwLock::new(Vec::<u64>::new()),
            alloc_map: RwLock::new(Vec::new()),
        };

        let mut alloc_ref = block_on(heap.alloc_map.write());

        for _ in 0..16 {
            alloc_ref.push(RwLock::new(AllocTypeInfo {
                size: 0,
                total: 0,
                free: 0,
            }))
        }

        heap
    }

    pub fn iter(&self) -> StackAllocatorIter {
        StackAllocatorIter {
            heap: self,
            offset: 8,
        }
    }

    // allocation metrics
    fn alloc_id(&self, id: u8) -> AllocTypeInfo {
        let alloc_ref = block_on(self.alloc_map.read());
        let alloc_type = block_on(alloc_ref[id as usize].read());

        *alloc_type
    }

    fn alloc_map(&self, id: u8, size: usize) {
        let alloc_ref = block_on(self.alloc_map.write());
        let mut alloc_type = block_on(alloc_ref[id as usize].write());

        alloc_type.size += size;
        alloc_type.total += 1;
    }

    // allocate
    pub fn alloc(&mut self, src: &[[u8; 8]], id: u8) -> usize {
        let ntypes = src.len() as u64;
        let mut stack_ref = block_on(self.stack.write());

        let image = stack_ref.len();
        let hinfo = AllocatorImageInfo::new()
            .with_reloc(0)
            .with_len(((ntypes + 1) * 8) as u16)
            .with_mark(false)
            .with_image_type(id)
            .into_bytes();

        stack_ref.reserve(src.len() + 1);
        stack_ref.push(u64::from_le_bytes(hinfo));

        for n in src {
            stack_ref.push(u64::from_le_bytes(*n));
        }

        self.alloc_map(id, src.len() * 8);

        image
    }

    pub fn valloc(&mut self, src: &[[u8; 8]], vdata: &[u8], id: u8) -> usize {
        let ntypes = src.len() as u64;
        let mut stack_ref = block_on(self.stack.write());

        let image = stack_ref.len();
        let hinfo = AllocatorImageInfo::new()
            .with_reloc(0)
            .with_len(((ntypes + 1) * 8) as u16)
            .with_mark(false)
            .with_image_type(id)
            .into_bytes();

        stack_ref.reserve(src.len() + 1);
        stack_ref.push(u64::from_le_bytes(hinfo));

        for n in src {
            stack_ref.push(u64::from_le_bytes(*n));
        }

        // todo: add vector copy here
        self.alloc_map(id, src.len() * 8 + vdata.len());

        image
    }

    // rewrite info header
    pub fn write_info(&mut self, _info: AllocatorImageInfo, _off: usize) {
        let mut _stack_ref = block_on(self.stack.write());

        // stack_ref[off] = u64::from_le_bytes(std::slice_from_raw_parts(info));
    }

    // info header from heap tag
    pub fn image_info(&self, off: usize) -> Option<AllocatorImageInfo> {
        let stack_ref = block_on(self.stack.write());

        if off == 0 || off > stack_ref.len() {
            None
        } else {
            let info = stack_ref[off].to_le_bytes();

            Some(AllocatorImageInfo::from_bytes(info))
        }
    }

    pub fn image_reloc(&self, off: usize) -> Option<u32> {
        self.image_info(off).map(|info| info.reloc())
    }

    pub fn image_length(&self, off: usize) -> Option<usize> {
        self.image_info(off).map(|info| info.len() as usize)
    }

    pub fn image_refbit(&self, off: usize) -> Option<bool> {
        self.image_info(off).map(|info| info.mark())
    }

    pub fn image_tag_type(&self, off: usize) -> Option<u8> {
        self.image_info(off).map(|info| info.image_type())
    }

    // read and write image data
    pub fn write_image(&mut self, image: &[[u8; 8]], offset: usize) {
        let mut stack_ref = block_on(self.stack.write());
        let mut index = offset;

        for n in image {
            stack_ref[index] = u64::from_le_bytes(*n);
            index += 1;
        }
    }

    pub fn image_slice(&self, off: usize, _len: usize) -> Option<&[u8]> {
        let stack_ref = block_on(self.stack.write());

        if off == 0 || off > stack_ref.len() {
            None
        } else {
            // Some(&stack_ref[off].to_le_bytes())
            Some(&[0])
        }
    }

    // gc
    pub fn gc_clear(&mut self) {
        let mut off: usize = 8;
        let alloc_ref = block_on(self.alloc_map.read());

        while let Some(mut info) = self.image_info(off) {
            info.set_mark(false);
            self.write_info(info, off);
            off += info.len() as usize
        }

        for (id, _) in alloc_ref.iter().enumerate() {
            let mut alloc_type = block_on(alloc_ref[id].write());

            alloc_type.free = 0;
        }
    }

    pub fn gc_sweep(&mut self) {
        let mut off: usize = 8;
        let alloc_ref = block_on(self.alloc_map.write());

        while let Some(info) = self.image_info(off) {
            if !info.mark() {
                let id = info.image_type() as usize;
                let mut alloc_type = block_on(alloc_ref[id].write());

                alloc_type.free += 1;
            }
            off += info.len() as usize
        }
    }

    pub fn set_image_refbit(&mut self, off: usize) {
        match self.image_info(off) {
            Some(mut info) => {
                info.set_mark(true);
                self.write_info(info, off)
            }
            None => panic!(),
        }
    }

    pub fn get_image_refbit(&self, off: usize) -> Option<bool> {
        self.image_info(off).map(|info| info.mark())
    }
}

// iterators
pub struct StackAllocatorIter<'a> {
    pub heap: &'a StackAllocator,
    pub offset: usize,
}

impl Iterator for StackAllocatorIter<'_> {
    type Item = (AllocatorImageInfo, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.heap.image_info(self.offset) {
            Some(info) => {
                let id = self.offset;
                self.offset += info.len() as usize;
                Some((info, id))
            }
            None => None,
        }
    }
}
