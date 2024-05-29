//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image writer
#[allow(unused_imports)]
use {
    crate::image::Image,
    object::{
        build::elf,
        elf::{FileHeader64, SectionHeader64},
        read::elf::{FileHeader, SectionHeader, Sym},
        write::{elf::Writer as ElfWriter, Object, StandardSegment},
        Architecture, BinaryFormat, Endianness, SectionKind,
    },
    std::{
        env,
        fs::{self, File},
        io::{self, Error, ErrorKind, Read, Result, Write},
    },
};

pub struct Writer {
    pub path: String,
    pub ident: Vec<u8>,
    pub image: Vec<u8>,
}

impl Writer {
    pub fn with_writer(path: &str, ident: Vec<u8>, image: Vec<u8>) -> Result<Self> {
        Ok(Writer {
            path: path.to_string(),
            ident: ident.clone(),
            image,
        })
    }

    fn image(&self, elf: &mut Object) {
        let image_id = elf.add_section(
            elf.segment_name(StandardSegment::Data).to_vec(),
            ".ident".as_bytes().to_vec(),
            SectionKind::Data,
        );

        elf.set_section_data(image_id, self.ident.clone(), 8);

        let image_id = elf.add_section(
            elf.segment_name(StandardSegment::Data).to_vec(),
            ".image".as_bytes().to_vec(),
            SectionKind::Data,
        );

        elf.set_section_data(image_id, self.image.clone(), 8);
    }

    pub fn write(&self) -> Result<()> {
        let mut elf_out = File::create(&self.path)?;
        let mut elf_object =
            Object::new(BinaryFormat::Elf, Architecture::Aarch64, Endianness::Little);

        self.image(&mut elf_object);

        match elf_object.write() {
            Ok(format) => {
                let _ = elf_out.write(&format)?;

                Ok(())
            }
            Err(_) => Err(Error::new(ErrorKind::WriteZero, &*self.path)),
        }
    }
}
