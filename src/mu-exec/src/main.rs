//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader/repl
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[allow(unused_imports)]
use {
    elf::{
        endian::AnyEndian,
        note::{Note, NoteGnuBuildId},
        section::SectionHeader,
        ElfBytes,
    },
    getopt::Opt,
    mu::{Condition, Env, Result, Tag},
    std::{error::Error, fs, io::Write},
};

#[derive(Debug, PartialEq)]
enum ExecOpt {
    Check,
    Toc,
    Path(String),
}

fn options(mut argv: Vec<String>) -> Option<Vec<ExecOpt>> {
    let mut opts = getopt::Parser::new(&argv, "h?vVd");
    let mut optv = Vec::new();

    loop {
        let opt = opts.next().transpose();
        match opt {
            Err(_) => {
                if let Err(error) = opt {
                    eprintln!("env-exec: option {error:?}")
                };
                std::process::exit(-1);
            }
            Ok(None) => {
                break;
            }
            Ok(clause) => match clause {
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => usage(),
                    Opt('c', None) => optv.push(ExecOpt::Check),
                    Opt('t', None) => optv.push(ExecOpt::Toc),
                    Opt('v', None) => {
                        print!("env-exec: {} ", Env::VERSION);
                        return None;
                    }
                    _ => panic!(),
                },
                None => panic!(),
            },
        }
    }

    for path in argv.split_off(opts.index()) {
        optv.push(ExecOpt::Path(path));
    }

    Some(optv)
}

fn usage() {
    println!("env-exec: {}: [-h?vd] file", Env::VERSION);
    println!("?: usage message");
    println!("h: usage message");
    println!("dump: load path");
    println!("V: verbose");
    println!("v: print version and exit");

    std::process::exit(0);
}

pub fn main() {
    let mut _check = false;
    let mut _toc = false;
    let mut _path = String::new();

    let _env = match Env::config(None) {
        Some(config) => Env::new(config, None),
        None => {
            eprintln!("option: configuration error");
            std::process::exit(-1)
        }
    };

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                match opt {
                    ExecOpt::Check => _check = true,
                    ExecOpt::Path(path) => _path = path,
                    ExecOpt::Toc => _toc = true,
                }
            }
        }
        None => std::process::exit(0),
    };

    let path = std::path::PathBuf::from(_path);
    let file_data = std::fs::read(path).expect("Could not read file.");
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Open test1");

    if _check {
        // Get the ELF file's build-id
        let abi_shdr: SectionHeader = file
            .section_header_by_name(".note.gnu.build-id")
            .expect("section table should be parseable")
            .expect("file should have a .note.ABI-tag section");

        let _notes: Vec<Note> = file
            .section_data_as_notes(&abi_shdr)
            .expect("Should be able to get note section data")
            .collect();

        let text_shdr: SectionHeader = file
            .section_header_by_name(".text")
            .expect("section table should be parseable")
            .expect("file should have a .text section");

        println!("text section size is {}", text_shdr.sh_size);
        println!("text section offset is {}", text_shdr.sh_offset);

        let text_off = text_shdr.sh_offset as usize;
        let _foo: &[u8] = &slice[text_off..text_off + 8];

        println!("{:x?}", _foo);
    }
}
