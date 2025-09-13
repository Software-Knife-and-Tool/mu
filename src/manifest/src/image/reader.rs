#[allow(unused_imports)]
use {
    crate::image::heap_info::{HeapInfo, HeapInfoBuilder},
    object::{
        build::elf,
        elf::{FileHeader64, SectionHeader64, Sym64, SHT_SYMTAB},
        read::elf::{FileHeader, SectionHeader, Sym},
        read::{self, Export, StringTable, Symbol},
        Architecture, BinaryFormat, Endianness,
    },
    std::{
        env,
        fs::{self, File},
        io::{self, Error, ErrorKind, Read, Result, Write},
    },
};

pub struct Reader {
    pub elf: FileHeader64<Endianness>,
    pub data: Vec<u8>,
    pub sections: Vec<SectionHeader64<Endianness>>,
}

impl Reader {
    pub fn with_reader(path: &str) -> Result<Self> {
        let data = fs::read(path)?;

        match FileHeader64::<Endianness>::parse(&*data) {
            Ok(elf) => {
                let sections = elf.section_headers(Endianness::Little, &*data).unwrap();

                Ok(Self {
                    elf: *elf,
                    data: data.to_vec(),
                    sections: sections.to_vec(),
                })
            }
            Err(_) => Err(Error::new(ErrorKind::InvalidData, path)),
        }
    }

    /*
        pub fn symbols(&self) -> Option<Vec<&Sym64<Endianness>>> {
            let mut sym_vec = Vec::new();

            let endian = self.elf.endian().unwrap();
            let sections = self.elf.sections(endian, &*self.data).unwrap();
            let symbols = sections.symbols(endian, &*self.data, SHT_SYMTAB).unwrap();
            for symbol in symbols.iter() {
                match symbol.name(endian, symbols.strings()) {
                    Ok(_) => sym_vec.push(symbol),
                    Err(_) => return None,
                }
            }

            Some(sym_vec)
    }
        */

    pub fn symbol_names(&self) -> Option<Vec<String>> {
        let mut name_vec = Vec::new();

        let endian = self.elf.endian().unwrap();
        let sections = self.elf.sections(endian, &*self.data).unwrap();
        let symbols = sections.symbols(endian, &*self.data, SHT_SYMTAB).unwrap();
        for symbol in symbols.iter() {
            match symbol.name(endian, symbols.strings()) {
                Ok(name) => name_vec.push(String::from_utf8(name.to_vec()).unwrap()),
                Err(_) => return None,
            }
        }

        Some(name_vec)
    }

    fn string_table(&self) -> StringTable<'_> {
        self.elf
            .section_strings(Endianness::Little, &*self.data, &self.sections)
            .unwrap()
    }

    pub fn section_names(&self) -> Vec<String> {
        self.sections
            .iter()
            .map(|header| {
                String::from_utf8(
                    header
                        .name(self.elf.endian().unwrap(), self.string_table())
                        .unwrap()
                        .to_vec(),
                )
                .unwrap()
            })
            .collect()
    }

    pub fn section_by_name(&self, section_name: &str) -> Option<&SectionHeader64<Endianness>> {
        let section = self.sections.iter().find(|section| {
            let section_name_data = section
                .name(self.elf.endian().unwrap(), self.string_table())
                .unwrap();
            let header_name = String::from_utf8(section_name_data.to_vec()).unwrap();

            section_name == header_name
        });

        section
    }

    pub fn section_data(&self, section: &SectionHeader64<Endianness>) -> Option<Vec<u8>> {
        let endian = self.elf.endian().unwrap();

        Some(section.data(endian, &*self.data).unwrap().to_vec())
    }
}
