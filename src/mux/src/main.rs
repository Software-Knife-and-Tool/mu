//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
mod counts;
mod crossref;
mod exec;
mod options;
mod profile;
mod reference;
mod test;

#[rustfmt::skip]
use {
    crate::{
        counts::Counts,
        crossref::Crossref,
        exec::Exec,
        options::Options,
        profile::Profile,
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
        "crossref" => Crossref::crossref(&options),
        "profile" => Profile::profile(&options),
        "reference" => Reference::reference(&options),
        "repl" => Exec::repl(&options),
        "symbol-counts" => Counts::counts(&options),
        "test" => Test::test(&options),
        "version" => Options::version(),
        _ => {
            eprintln!("mux: unimplemented command {command}");
            std::process::exit(-1)
        }
    }
}
