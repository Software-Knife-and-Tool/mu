//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// heap
#![allow(unused_parens)] // modular bitmap is a mess
#[rustfmt::skip]
use {
    crate::{
        core::{
            config::Config,
            direct::DirectTag,
            env::Env,
            tag::Tag,
            type_::Type,
        },
        types::{
            cons::Cons,
            function::Function,
            struct_::Struct,
            symbol::Symbol,
            vector::Vector
        },
    },
    futures_lite::future::block_on,
    memmap,
    modular_bitfield::specifiers::{B11, B4},
    page_size,
    std::{
        fmt,
        fs::{remove_file, OpenOptions},
        io::{Seek, SeekFrom, Write},
        mem::size_of,
    },
};

// #[repr(align(8))]
#[bitfield]
#[derive(Specifier, Debug, Copy, Clone)]
pub struct HeapImageInfo {
    pub reloc: u32, // relocation
    #[skip]
    __: B11, // expansion
    pub mark: bool, // reference counting
    pub len: u16,   // in bytes
    pub image_type: B4, // tag type
}

impl Default for HeapImageInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for HeapImageInfo {
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

#[derive(Debug, Copy, Clone)]
pub struct HeapTypeInfo {
    pub size: usize,
    pub total: usize,
    pub free: usize,
}

#[derive(Debug)]
pub struct Heap {
    pub mmap: Box<memmap::MmapMut>, // heap base
    pub npages: usize,              // total size of heap in pages
    pub size: usize,                // total size of heap in bytes
    pub page_size: usize,           // system page size
    pub alloc_map: [HeapTypeInfo; Type::NTYPES],
    // map of allocated objects
    pub alloc_barrier: usize, // unallocated space barrier
    pub free_map: [Vec<usize>; Type::NTYPES],
    // map of free objects
    pub free_space: usize,   // number of aggregate free bytes
    pub gc_allocated: usize, // bytes allocated since last gc
}

pub struct HeapRequest<'a> {
    pub env: &'a Env,
    pub image: &'a [[u8; 8]],
    pub type_id: u8,
    pub vdata: Option<&'a [u8]>,
}

impl Heap {
    pub fn new(config: &Config) -> Self {
        let path = &format!("/var/tmp/mu.{}.heap", std::process::id());

        let page_size = page_size::get();
        let npages = config.npages;

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("unable to open heap mmap file");

        remove_file(path).unwrap();

        f.seek(SeekFrom::Start((npages * page_size) as u64))
            .unwrap();
        f.write_all(&[0]).unwrap();
        f.rewind().unwrap();

        let data = unsafe {
            memmap::MmapOptions::new()
                .map_mut(&f)
                .expect("Could not access data from memory mapped file")
        };

        Heap {
            mmap: Box::new(data),
            npages,
            page_size,
            size: npages * page_size,
            alloc_map: [HeapTypeInfo {
                size: 0,
                total: 0,
                free: 0,
            }; Type::NTYPES],
            alloc_barrier: 0,
            free_map: [const { Vec::new() }; Type::NTYPES],
            free_space: npages * page_size,
            gc_allocated: 0,
        }
    }

    pub fn image(env: &Env) -> (Vec<u8>, Vec<u8>) {
        let heap_ref = block_on(env.heap.write());
        let image = heap_ref.heap_slice();

        (image.to_vec(), vec![])
    }

    pub fn iter(&self) -> HeapIter<'_> {
        HeapIter {
            heap: self,
            index: 1,
        }
    }

    // allocate
    pub fn alloc(&mut self, req: &HeapRequest) -> Option<usize> {
        let image_len = req.image.len();
        let vdata_size: usize = match req.vdata {
            None => 0,
            Some(vdata) => (vdata.len() + size_of::<u64>() - 1) & !(size_of::<u64>() - 1),
        };
        let image_size = ((image_len + 1) * size_of::<u64>()) + vdata_size;

        self.free_space -= image_size + size_of::<HeapImageInfo>();

        let index = match self.alloc_free(req.type_id, (image_len * size_of::<u64>()) + vdata_size)
        {
            Some(index) => {
                let data = &mut self.mmap;
                let mut off = index * size_of::<u64>();

                for image_slice in req.image {
                    data[off..(off + size_of::<u64>())].copy_from_slice(image_slice);
                    off += size_of::<u64>();
                }

                match req.vdata {
                    Some(vdata) if vdata_size != 0 => {
                        data[off..(off + vdata.len())].copy_from_slice(vdata);
                    }
                    _ => (),
                }

                index
            }
            None => {
                if self.alloc_barrier + image_size > self.size {
                    None?
                }

                self.gc_allocated += image_size;

                let hinfo = HeapImageInfo::new()
                    .with_reloc(0)
                    .with_len(image_size as u16)
                    .with_mark(false)
                    .with_image_type(req.type_id)
                    .into_bytes();

                let data = &mut self.mmap;

                data[self.alloc_barrier..(self.alloc_barrier + 8)].copy_from_slice(&hinfo);
                self.alloc_barrier += size_of::<HeapImageInfo>();

                let index = self.alloc_barrier / size_of::<u64>();

                for image_slice in req.image {
                    data[self.alloc_barrier..(self.alloc_barrier + size_of::<u64>())]
                        .copy_from_slice(image_slice);
                    self.alloc_barrier += size_of::<u64>();
                }

                if vdata_size != 0 {
                    data[self.alloc_barrier..(self.alloc_barrier + req.vdata.unwrap().len())]
                        .copy_from_slice(req.vdata.unwrap());
                    self.alloc_barrier += vdata_size;
                }

                self.alloc_map[req.type_id as usize].size +=
                    (image_len * size_of::<u64>()) + vdata_size;
                self.alloc_map[req.type_id as usize].total += 1;

                index
            }
        };

        Some(index)
    }

    // try first fit
    fn alloc_free(&mut self, type_id: u8, size: usize) -> Option<usize> {
        for (index, off) in self.free_map[type_id as usize].iter().enumerate() {
            if self.image_info(*off).unwrap().len() >= size as u16 {
                self.alloc_map[type_id as usize].total += 1;

                return Some(self.free_map[type_id as usize].remove(index));
            }
        }

        None
    }

    // rewrite info header
    pub fn write_info(&mut self, info: HeapImageInfo, index: usize) {
        let off = index * size_of::<u64>();

        self.mmap[(off - 8)..off].copy_from_slice(&(info.into_bytes()))
    }

    // info header from heap tag
    pub fn image_info(&self, index: usize) -> Option<HeapImageInfo> {
        let off = index * size_of::<u64>();

        if off == 0 || off > self.alloc_barrier {
            None
        } else {
            let data = &self.mmap;
            let mut info = 0_u64.to_le_bytes();

            info.copy_from_slice(&data[(off - 8)..off]);
            Some(HeapImageInfo::from_bytes(info))
        }
    }

    pub fn image_reloc(&self, index: usize) -> Option<u32> {
        self.image_info(index).map(|info| info.reloc())
    }

    pub fn image_length(&self, index: usize) -> Option<usize> {
        self.image_info(index).map(|info| info.len() as usize)
    }

    pub fn image_refbit(&self, index: usize) -> Option<bool> {
        self.image_info(index).map(|info| info.mark())
    }

    pub fn image_tag_type(&self, index: usize) -> Option<u8> {
        self.image_info(index).map(|info| info.image_type())
    }

    // read and write image data
    pub fn write_image(&mut self, image: &[[u8; 8]], index: usize) {
        let mut off = index * size_of::<u64>();

        for image_slice in image {
            self.mmap[off..(off + 8)].copy_from_slice(image_slice);
            off += 8;
        }
    }

    pub fn image_data_slice(&self, index: usize, offset: usize, len: usize) -> Option<&[u8]> {
        let off = (index * size_of::<u64>()) + offset;

        if off == 0 || off > self.alloc_barrier {
            None
        } else {
            let data = &self.mmap;
            Some(&data[off..off + len])
        }
    }

    pub fn image_slice(&self, index: usize) -> Option<&[u8]> {
        let off = index * size_of::<u64>();

        if off == 0 || off > self.alloc_barrier {
            None
        } else {
            let data = &self.mmap;
            Some(&data[off..off + 8])
        }
    }

    pub fn heap_slice(&self) -> &[u8] {
        let data = &self.mmap;

        &data[0..self.size]
    }

    pub fn image_size(env: &Env, tag: Tag) -> usize {
        match tag.type_of() {
            Type::Cons => Cons::image_size(env, tag),
            Type::Function => Function::image_size(env, tag),
            Type::Struct => Struct::image_size(env, tag),
            Type::Symbol => Symbol::image_size(env, tag),
            Type::Vector => Vector::image_size(env, tag),
            _ => size_of::<DirectTag>(),
        }
    }

    pub fn heap_free(env: &Env) -> usize {
        let heap_ref = block_on(env.heap.read());

        heap_ref.free_space
    }

    pub fn heap_info(env: &Env) -> (usize, usize) {
        let heap_ref = block_on(env.heap.read());

        (heap_ref.page_size, heap_ref.npages)
    }

    pub fn heap_type(env: &Env, type_: Type) -> HeapTypeInfo {
        let heap_ref = block_on(env.heap.read());

        heap_ref.alloc_map[type_ as usize]
    }
}

pub trait Gc {
    fn clear_marks(&mut self);
    fn sweep(&mut self);
    fn set_image_mark(&mut self, _: usize);
    fn get_image_mark(&self, _: usize) -> Option<bool>;
}

impl Gc for Heap {
    fn clear_marks(&mut self) {
        let mut index: usize = 1;

        while let Some(mut info) = self.image_info(index) {
            info.set_mark(false);
            self.write_info(info, index);
            index += (info.len() as usize) / size_of::<u64>()
        }

        for type_map in self.alloc_map.iter_mut() {
            type_map.free = 0
        }

        for free_map in self.free_map.iter_mut() {
            free_map.clear()
        }
    }

    fn sweep(&mut self) {
        let free_list = self
            .iter()
            .filter(|(info, _)| !info.mark())
            .collect::<Vec<(HeapImageInfo, usize)>>();

        for (info, index) in free_list {
            let type_id = info.image_type() as usize;

            self.alloc_map[type_id].free += 1;
            self.free_map[type_id].push(index);
        }
    }

    fn set_image_mark(&mut self, index: usize) {
        match self.image_info(index) {
            Some(mut info) => {
                info.set_mark(true);
                self.write_info(info, index)
            }
            None => panic!(),
        }
    }

    fn get_image_mark(&self, index: usize) -> Option<bool> {
        self.image_info(index).map(|info| info.mark())
    }
}

// iterator
pub struct HeapIter<'a> {
    pub heap: &'a Heap,
    pub index: usize,
}

impl Iterator for HeapIter<'_> {
    type Item = (HeapImageInfo, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.heap.image_info(self.index) {
            Some(info) => {
                let type_id = self.index;
                self.index += (info.len() as usize) / size_of::<u64>();
                Some((info, type_id))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn heap_test() {
        assert!(true);
    }
}
