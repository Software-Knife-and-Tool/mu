//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
mod modules;
mod options;
mod workspace;

#[rustfmt::skip]
use crate::{
    modules::{
        bench::Bench,
        check::Check,
        clean::Clean,
     //   install::Install,
        commit::Commit,
        regression::Regression,
        symbols::Symbols,
    },
    options::Options,
    workspace::Workspace,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn usage() {
    println!("Usage: sys-dev {VERSION} command [option...]");
    println!("  command:");
    println!("    help                               ; this message");
    println!("    version                            ; sys-dev version");
    println!();
    println!("    workspace init | env               ; manage workspace");
    println!();
    println!("    dist                               ; build distribution");
    println!("    check     --config=string          ; check config string");
    println!("    bench     base | current | report | clean [--ntests=number] [--all]");
    println!("                                       ; benchmark test suite");
    println!("    regression                         ; regression test suite");
    println!("    symbols   reference | crossref | metrics [--namespace=name]");
    println!("                                       ; symbol reports, namespace defaults to mu");
    println!("    precommit                          ; fmt and clippy, pre-commit checking");
    println!();
    println!("  general options:");
    println!("    --verbose                          ; verbose operation");
    println!("    --recipe                           ; show recipe");

    std::process::exit(0);
}

pub fn main() {
    let argv = std::env::args().collect::<Vec<String>>();

    if argv.len() == 1 {
        usage();
    }

    let command = argv[1].as_str();

    match command {
        "precommit" => Commit::commit(&argv),
        "check" => Check::check(&argv, ""),
        "version" => Options::version(),
        "workspace" => Workspace::workspace(&argv),
        _ => {
            let home = match Workspace::env() {
                Some(path) => path,
                None => {
                    let cwd = std::env::current_dir().unwrap();

                    eprintln!(
                        "error: could not find `.sys-dev` in {:?} or any parent directory",
                        cwd.to_str().unwrap()
                    );
                    std::process::exit(-1)
                }
            };

            let ws = Workspace::new(&home);
            match command {
                "help" => {
                    println!();
                    println!("    sys-dev: mu packaging and development tool");
                    println!();
                    usage()
                }
                "bench" => Bench::new(&ws).bench(&argv).unwrap(),
                "dist" => Clean::clean(&argv, &home),
                "symbols" => Symbols::new(&ws).symbols(&argv).unwrap(),
                "regression" => Regression::new(&ws).regression(&argv).unwrap(),
                _ => {
                    eprintln!("unimplemented command {command}");
                    std::process::exit(-1)
                }
            }
        }
    }
}
