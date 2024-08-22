//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
mod counts;
mod crossref;
mod exec;
mod options;
mod reference;
mod test;

#[rustfmt::skip]
use {
    crate::{
        counts::Counts,
        crossref::Crossref,
        exec::Exec,
        options::Options,
        reference::Reference,
        test::Test,
    },
};

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
        "reference" => Reference::reference(&options),
        "symbol-counts" => Counts::counts(&options),
        "test" => Test::test(&options),
        _ => {
            eprintln!("mux: unimplemented command {command}");
            std::process::exit(-1)
        }
    }
}
