//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env heap
use {
    crate::core::allocator::{AllocTypeInfo, AllocatorImageInfo},
    memmap, page_size,
    std::{
        fs::{remove_file, OpenOptions},
        io::{Seek, SeekFrom, Write},
    },
};

use {futures::executor::block_on, futures_locks::RwLock};

#[derive(Debug)]
pub struct BumpAllocator {
    pub mmap: Box<memmap::MmapMut>,
    pub alloc_map: RwLock<Vec<RwLock<AllocTypeInfo>>>,
    pub free_map: Vec<Vec<usize>>,
    pub page_size: usize,
    pub npages: usize,
    pub size: usize,
    pub write_barrier: usize, // byte offset
}

impl BumpAllocator {
    const SIZEOF_U64: usize = std::mem::size_of::<u64>();

    pub fn new(pages: usize, ntypes: u8) -> Self {
        let path = &format!("/var/tmp/mu.{}.heap", std::process::id());

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("unable to open heap mmap file");

        remove_file(path).unwrap();

        f.seek(SeekFrom::Start((pages * 4096) as u64)).unwrap();
        f.write_all(&[0]).unwrap();
        f.rewind().unwrap();

        // unavoidable
        let data = unsafe {
            memmap::MmapOptions::new()
                .map_mut(&f)
                .expect("Could not access data from memory mapped file")
        };

        let mut heap = BumpAllocator {
            mmap: Box::new(data),
            page_size: page_size::get(),
            npages: pages,
            size: pages * page_size::get(),
            alloc_map: RwLock::new(Vec::new()),
            free_map: Vec::new(),
            write_barrier: 0,
        };

        for _ in 0..ntypes {
            heap.free_map.push(Vec::<usize>::new())
        }

        let mut alloc_ref = block_on(heap.alloc_map.write());

        for _ in 0..ntypes {
            alloc_ref.push(RwLock::new(AllocTypeInfo {
                size: 0,
                total: 0,
                free: 0,
            }))
        }

        heap
    }

    pub fn iter(&self) -> BumpAllocatorIter {
        BumpAllocatorIter {
            heap: self,
            index: 1,
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
    pub fn alloc(&mut self, image: &[[u8; 8]], vdata: Option<&[u8]>, id: u8) -> Option<usize> {
        let image_len = image.len();
        let base = self.write_barrier;

        let vdata_size: usize = match vdata {
            None => 0,
            Some(vdata) => (vdata.len() + Self::SIZEOF_U64 - 1) & !(Self::SIZEOF_U64 - 1),
        };

        if base + ((image_len + 1) * Self::SIZEOF_U64 + vdata_size) > self.size {
            return None;
        }

        if let Some(index) = self.alloc_free(id, (image_len * Self::SIZEOF_U64) + vdata_size) {
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

            Some(index)
        } else {
            let hinfo = AllocatorImageInfo::new()
                .with_reloc(0)
                .with_len((((image_len + 1) * Self::SIZEOF_U64) + vdata_size) as u16)
                .with_mark(false)
                .with_image_type(id)
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

            self.alloc_map(id, (image_len * Self::SIZEOF_U64) + vdata_size);

            Some(index)
        }
    }

    // try first fit
    fn alloc_free(&mut self, id: u8, size: usize) -> Option<usize> {
        for (index, off) in self.free_map[id as usize].iter().enumerate() {
            match self.image_info(*off) {
                Some(info) => {
                    if info.len() >= size as u16 {
                        let alloc_ref = block_on(self.alloc_map.read());
                        let mut alloc_type = block_on(alloc_ref[id as usize].write());

                        alloc_type.free -= 1;

                        return Some(self.free_map[id as usize].remove(index));
                    }
                }
                None => panic!(),
            }
        }

        None
    }

    // rewrite info header
    pub fn write_info(&mut self, info: AllocatorImageInfo, index: usize) {
        let off = index * Self::SIZEOF_U64;

        self.mmap[(off - 8)..off].copy_from_slice(&(info.into_bytes()))
    }

    // info header from heap tag
    pub fn image_info(&self, index: usize) -> Option<AllocatorImageInfo> {
        let off = index * Self::SIZEOF_U64;

        if off == 0 || off > self.write_barrier {
            None
        } else {
            let data = &self.mmap;
            let mut info = 0u64.to_le_bytes();

            info.copy_from_slice(&data[(off - 8)..off]);
            Some(AllocatorImageInfo::from_bytes(info))
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

    // gc
    pub fn clear_marks(&mut self) {
        let mut index: usize = 1;
        let alloc_ref = block_on(self.alloc_map.read());

        while let Some(mut info) = self.image_info(index) {
            info.set_mark(false);
            self.write_info(info, index);
            index += (info.len() as usize) / Self::SIZEOF_U64
        }

        for type_map in (*alloc_ref).iter() {
            let mut alloc_type = block_on(type_map.write());
            alloc_type.free = 0
        }

        for free_map in self.free_map.iter_mut() {
            free_map.clear()
        }
    }

    pub fn sweep(&mut self) {
        let alloc_ref = block_on(self.alloc_map.write());

        let free_list = self
            .iter()
            .filter(|(info, _)| !info.mark())
            .collect::<Vec<(AllocatorImageInfo, usize)>>();

        for (info, index) in free_list {
            let id = info.image_type() as usize;
            let mut alloc_type = block_on(alloc_ref[id].write());

            alloc_type.free += 1;
            self.free_map[id].push(index);
        }
    }

    pub fn set_image_mark(&mut self, index: usize) {
        match self.image_info(index) {
            Some(mut info) => {
                info.set_mark(true);
                self.write_info(info, index)
            }
            None => panic!(),
        }
    }

    pub fn get_image_mark(&self, index: usize) -> Option<bool> {
        let _ = block_on(self.alloc_map.read());

        self.image_info(index).map(|info| info.mark())
    }
}

// iterator
pub struct BumpAllocatorIter<'a> {
    pub heap: &'a BumpAllocator,
    pub index: usize,
}

impl<'a> Iterator for BumpAllocatorIter<'a> {
    type Item = (AllocatorImageInfo, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.heap.image_info(self.index) {
            Some(info) => {
                let id = self.index;
                self.index += (info.len() as usize) / std::mem::size_of::<u64>();
                Some((info, id))
            }
            None => None,
        }
    }
}
