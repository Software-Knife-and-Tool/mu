//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#[rustfmt::skip]
use {
    crate::{
        crossref::Crossref,
        options::Options,
        exec::Exec,
        test::Test,
        symbols::Symbols,
    },
};

mod crossref;
mod exec;
mod options;
mod symbols;
mod test;

pub fn main() {
    let argv = std::env::args().collect::<Vec<String>>();

    if argv.len() == 1 {
        Options::usage();
    }

    let command = argv[1].as_str();
    let options = Options::parse(&argv).unwrap();

    match command {
        "help" => {
            println!();
            println!("    mu implementation explorer");
            println!();
            Options::usage()
        }
        "version" => Options::version(),
        "crossref" => Crossref::crossref(&options),
        "repl" => Exec::repl(&options),
        "symbols" => Symbols::symbols(&options),
        "test" => Test::test(&options),
        _ => {
            eprintln!("mux: unimplemented command {command}");
            std::process::exit(-1)
        }
    }
}
