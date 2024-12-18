#[allow(unused_imports)]
use {
    crate::image::heap_info::{HeapInfo, HeapInfoBuilder},
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
    pub image: HeapInfo,
}

impl Writer {
    pub fn with_writer(path: &str, allocator: HeapInfo) -> Result<Self> {
        Ok(Writer {
            path: path.to_string(),
            image: allocator,
        })
    }

    fn image(&self, elf: &mut Object) {
        let image_id = elf.add_section(
            elf.segment_name(StandardSegment::Data).to_vec(),
            ".image".as_bytes().to_vec(),
            SectionKind::Data,
        );

        elf.set_section_data(image_id, self.image.image.clone(), 8);

        let allocator_data: Vec<u8> = self.image.to_json().unwrap().into_bytes();

        let allocator_id = elf.add_section(
            elf.segment_name(StandardSegment::Data).to_vec(),
            ".allocator".as_bytes().to_vec(),
            SectionKind::Data,
        );

        elf.set_section_data(allocator_id, allocator_data, 8);
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
