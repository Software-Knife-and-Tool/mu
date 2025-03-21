//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env heap interface
//!    Env
use {
    crate::{
        core::{
            config::Config,
            direct::DirectTag,
            env::Env,
            indirect::IndirectTag,
            types::{Tag, TagType, Type},
        },
        types::{cons::Cons, function::Function, struct_::Struct, symbol::Symbol, vector::Vector},
    },
    memmap,
    modular_bitfield::specifiers::{B11, B4},
    std::{
        fmt,
        fs::{remove_file, OpenOptions},
        io::{Seek, SeekFrom, Write},
    },
};

use {futures::executor::block_on, futures_locks::RwLock};

#[bitfield]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
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
pub struct HeapAllocator {
    pub mmap: Box<memmap::MmapMut>,
    pub alloc_map: RwLock<Vec<RwLock<HeapTypeInfo>>>,
    pub free_map: Vec<Vec<usize>>,
    pub guard_page: usize,
    pub page_size: usize,
    pub npages: usize,
    pub size: usize,
    pub write_barrier: usize, // byte offset
}

impl HeapAllocator {
    const SIZEOF_U64: usize = std::mem::size_of::<u64>();

    pub fn new(config: &Config) -> Self {
        let path = &format!("/var/tmp/mu.{}.heap", std::process::id());

        let npages = config.npages;
        let page_size = config.page_size;

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

        HeapAllocator {
            alloc_map: RwLock::new(
                (0..Tag::NTYPES)
                    .map(|_| {
                        RwLock::new(HeapTypeInfo {
                            size: 0,
                            total: 0,
                            free: 0,
                        })
                    })
                    .collect::<Vec<RwLock<HeapTypeInfo>>>(),
            ),
            free_map: (0..Tag::NTYPES)
                .map(|_| Vec::<usize>::new())
                .collect::<Vec<Vec<usize>>>(),
            mmap: Box::new(data),
            npages,
            page_size,
            size: npages * page_size,
            guard_page: (npages - 1) * page_size,
            write_barrier: 0,
        }
    }

    pub fn image(env: &Env) -> (Vec<u8>, Vec<u8>) {
        let heap_ref = block_on(env.heap.write());
        let image = heap_ref.heap_slice();

        (image.to_vec(), vec![])
    }

    pub fn iter(&self) -> HeapAllocatorIter {
        HeapAllocatorIter {
            heap: self,
            index: 1,
        }
    }

    // allocate
    pub fn alloc(
        &mut self,
        image: &[[u8; 8]],
        vdata: Option<&[u8]>,
        type_id: u8,
    ) -> Option<(bool, usize)> {
        let image_len = image.len();
        let base = self.write_barrier;
        let guard_page = self.guard_page;

        let vdata_size: usize = match vdata {
            None => 0,
            Some(vdata) => (vdata.len() + Self::SIZEOF_U64 - 1) & !(Self::SIZEOF_U64 - 1),
        };

        if base + ((image_len + 1) * Self::SIZEOF_U64 + vdata_size) > self.size {
            return None;
        }

        let high_water: bool = base >= guard_page;

        if let Some(index) = self.alloc_free(type_id, (image_len * Self::SIZEOF_U64) + vdata_size) {
            let data = &mut self.mmap;
            let mut off = index * Self::SIZEOF_U64;

            for image_slice in image {
                data[off..(off + Self::SIZEOF_U64)].copy_from_slice(image_slice);
                off += Self::SIZEOF_U64;
            }

            match vdata {
                Some(vdata) if vdata_size != 0 => {
                    data[off..(off + vdata.len())].copy_from_slice(vdata);
                }
                _ => (),
            }

            Some((false, index))
        } else {
            let hinfo = HeapImageInfo::new()
                .with_reloc(0)
                .with_len((((image_len + 1) * Self::SIZEOF_U64) + vdata_size) as u16)
                .with_mark(false)
                .with_image_type(type_id)
                .into_bytes();

            let data = &mut self.mmap;

            data[self.write_barrier..(self.write_barrier + 8)].copy_from_slice(&hinfo);
            self.write_barrier += Self::SIZEOF_U64;

            let index = self.write_barrier / Self::SIZEOF_U64;

            for image_slice in image {
                data[self.write_barrier..(self.write_barrier + Self::SIZEOF_U64)]
                    .copy_from_slice(image_slice);
                self.write_barrier += Self::SIZEOF_U64;
            }

            if vdata_size != 0 {
                data[self.write_barrier..(self.write_barrier + vdata.unwrap().len())]
                    .copy_from_slice(vdata.unwrap());
                self.write_barrier += vdata_size;
            }

            let alloc_ref = block_on(self.alloc_map.write());
            let mut alloc_type = block_on(alloc_ref[type_id as usize].write());

            alloc_type.size += (image_len * Self::SIZEOF_U64) + vdata_size;
            alloc_type.total += 1;

            Some((high_water, index))
        }
    }

    // try first fit
    fn alloc_free(&mut self, type_id: u8, size: usize) -> Option<usize> {
        for (index, off) in self.free_map[type_id as usize].iter().enumerate() {
            match self.image_info(*off) {
                Some(info) => {
                    if info.len() >= size as u16 {
                        let alloc_ref = block_on(self.alloc_map.write());
                        let mut alloc_type = block_on(alloc_ref[type_id as usize].write());

                        alloc_type.free -= 1;

                        return Some(self.free_map[type_id as usize].remove(index));
                    }
                }
                None => panic!(),
            }
        }

        None
    }

    // rewrite info header
    pub fn write_info(&mut self, info: HeapImageInfo, index: usize) {
        let off = index * Self::SIZEOF_U64;

        self.mmap[(off - 8)..off].copy_from_slice(&(info.into_bytes()))
    }

    // info header from heap tag
    pub fn image_info(&self, index: usize) -> Option<HeapImageInfo> {
        let off = index * Self::SIZEOF_U64;

        if off == 0 || off > self.write_barrier {
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
        let mut off = index * Self::SIZEOF_U64;

        for image_slice in image {
            self.mmap[off..(off + 8)].copy_from_slice(image_slice);
            off += 8;
        }
    }

    pub fn image_data_slice(&self, index: usize, offset: usize, len: usize) -> Option<&[u8]> {
        let off = (index * Self::SIZEOF_U64) + offset;

        if off == 0 || off > self.write_barrier {
            None
        } else {
            let data = &self.mmap;
            Some(&data[off..off + len])
        }
    }

    pub fn image_slice(&self, index: usize) -> Option<&[u8]> {
        let off = index * Self::SIZEOF_U64;

        if off == 0 || off > self.write_barrier {
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

    pub fn heap_size(env: &Env, tag: Tag) -> usize {
        match tag.type_of() {
            Type::Cons => Cons::heap_size(env, tag),
            Type::Function => Function::heap_size(env, tag),
            Type::Struct => Struct::heap_size(env, tag),
            Type::Symbol => Symbol::heap_size(env, tag),
            Type::Vector => Vector::heap_size(env, tag),
            _ => std::mem::size_of::<DirectTag>(),
        }
    }

    pub fn heap_free(env: &Env) -> usize {
        let heap_ref = block_on(env.heap.read());
        let mut heap_free = heap_ref.size - heap_ref.write_barrier;

        for type_id in 0..Tag::NTYPES {
            match Type::try_from(type_id).unwrap() {
                Type::Cons => {
                    for image_id in &heap_ref.free_map[type_id as usize] {
                        let ind = IndirectTag::new()
                            .with_image_id(*image_id as u64)
                            .with_heap_id(1)
                            .with_tag(TagType::Cons);

                        heap_free += Cons::heap_size(env, Tag::Indirect(ind)) + 8;
                    }
                }
                Type::Function => {
                    for image_id in &heap_ref.free_map[type_id as usize] {
                        let ind = IndirectTag::new()
                            .with_image_id(*image_id as u64)
                            .with_heap_id(1)
                            .with_tag(TagType::Function);

                        heap_free += Function::heap_size(env, Tag::Indirect(ind)) + 8;
                    }
                }
                Type::Struct => {
                    for image_id in &heap_ref.free_map[type_id as usize] {
                        let ind = IndirectTag::new()
                            .with_image_id(*image_id as u64)
                            .with_heap_id(1)
                            .with_tag(TagType::Struct);

                        heap_free += Struct::heap_size(env, Tag::Indirect(ind)) + 8;
                    }
                }
                Type::Symbol => {
                    for image_id in &heap_ref.free_map[type_id as usize] {
                        let ind = IndirectTag::new()
                            .with_image_id(*image_id as u64)
                            .with_heap_id(1)
                            .with_tag(TagType::Symbol);

                        heap_free += Symbol::heap_size(env, Tag::Indirect(ind)) + 8;
                    }
                }
                Type::Vector => {
                    for image_id in &heap_ref.free_map[type_id as usize] {
                        let ind = IndirectTag::new()
                            .with_image_id(*image_id as u64)
                            .with_heap_id(1)
                            .with_tag(TagType::Vector);

                        heap_free += Vector::heap_size(env, Tag::Indirect(ind)) + 8;
                    }
                }
                _ => (),
            };
        }

        heap_free
    }

    pub fn heap_info(env: &Env) -> (usize, usize) {
        let heap_ref = block_on(env.heap.read());

        (heap_ref.page_size, heap_ref.npages)
    }

    pub fn heap_type(env: &Env, type_: Type) -> HeapTypeInfo {
        let heap_ref = block_on(env.heap.read());
        let alloc_ref = block_on(heap_ref.alloc_map.read());
        let type_ref = block_on(alloc_ref[type_ as usize].read());

        *type_ref
    }
}

pub trait GC {
    fn clear_marks(&mut self);
    fn sweep(&mut self);
    fn set_image_mark(&mut self, _: usize);
    fn get_image_mark(&self, _: usize) -> Option<bool>;
}

impl GC for HeapAllocator {
    fn clear_marks(&mut self) {
        let mut index: usize = 1;

        while let Some(mut info) = self.image_info(index) {
            info.set_mark(false);
            self.write_info(info, index);
            index += (info.len() as usize) / Self::SIZEOF_U64
        }

        let alloc_ref = block_on(self.alloc_map.write());
        for type_map in (*alloc_ref).iter() {
            let mut alloc_type = block_on(type_map.write());

            alloc_type.free = 0
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
            let alloc_ref = block_on(self.alloc_map.read());
            let mut alloc_type = block_on(alloc_ref[type_id].write());

            alloc_type.free += 1;
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
pub struct HeapAllocatorIter<'a> {
    pub heap: &'a HeapAllocator,
    pub index: usize,
}

impl Iterator for HeapAllocatorIter<'_> {
    type Item = (HeapImageInfo, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.heap.image_info(self.index) {
            Some(info) => {
                let type_id = self.index;
                self.index += (info.len() as usize) / std::mem::size_of::<u64>();
                Some((info, type_id))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn env() {
        assert_eq!(2 + 2, 4);
    }
}
