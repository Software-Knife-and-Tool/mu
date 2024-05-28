//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader
#[allow(unused_imports)]
use crate::{
    crux::{Condition, Env, Result, Tag},
    reader::Reader,
    writer::Writer,
};

use json::{self, object};

pub struct Image {
    pub image: Vec<u8>,
    pub alloc_map: Vec<u8>,
    pub free_map: Vec<u8>,
    pub write_barrier: Vec<u8>,
}

pub struct ImageBuilder {
    image: Option<Vec<u8>>,
    alloc_map: Option<Vec<u8>>,
    free_map: Option<Vec<u8>>,
    write_barrier: Option<Vec<u8>>,
}

impl ImageBuilder {
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

    pub fn build(&self) -> Image {
        Image {
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

impl Image {
    pub fn to_json(&self) -> Option<String> {
        let image = object! {
            image: self.image.clone(),
            alloc_map: self.alloc_map.clone(),
            free_map: self.free_map.clone(),
            write_barrier: self.write_barrier.clone(),
        };

        Some(image.dump())
    }

    /*
    pub fn from_json(_json: String) -> Option<Self> {
    None
    }
     */

    pub fn dump(path: &str) {
        let reader = Reader::with_reader(path).unwrap();

        println!("{path:}/");
        println!("  sections/");

        let section_names = reader
            .section_names()
            .into_iter()
            .filter(|name| !name.is_empty())
            .collect::<Vec<String>>();

        for name in &section_names {
            println!("    {name}")
        }

        println!("  symbols/");

        let symbols = reader.symbol_names();

        for name in &symbols {
            println!("    {name:?}")
        }

        println!("  .allocator/");
        match reader.section_by_name(".allocator") {
            Some(section) => {
                let json = String::from_utf8(reader.section_data(section).unwrap()).unwrap();

                println!("   json: {json:?}");
            }
            None => println!("     ! .allocator section not found"),
        }

        println!("  .image/");
        match reader.section_by_name(".image") {
            Some(section) => println!("    data: {:?}", reader.section_data(section).unwrap()),
            None => println!("     ! .image section not found"),
        }
    }

    pub fn output(path: &str) {
        let image = ImageBuilder::new()
            .image(vec![1, 2, 3])
            .alloc_map(vec![vec![4], vec![5], vec![6]])
            .free_map(vec![vec![7], vec![8], vec![9]])
            .write_barrier(0)
            .build();

        let writer = Writer::with_writer(path, image).unwrap();

        writer.write().unwrap()
    }
}
