//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader
use crate::{bump_allocator::BumpAllocatorImageBuilder, reader::Reader, writer::Writer};

pub struct Image {}

impl Image {
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
        let bump_allocator = BumpAllocatorImageBuilder::new()
            .image(vec![1, 2, 3])
            .alloc_map(vec![vec![4], vec![5], vec![6]])
            .free_map(vec![vec![7], vec![8], vec![9]])
            .write_barrier(0)
            .build();

        let writer = Writer::with_writer(path, bump_allocator).unwrap();

        writer.write().unwrap()
    }
}
