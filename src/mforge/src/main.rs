//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
mod modules;
mod options;
mod workspace;

#[rustfmt::skip]
use crate::{
    modules::{
        bench::Bench,
        build::Build,
        clean::Clean,
        commit::Commit,
        install::Install,
        regression::Regression,
        symbols::Symbols,
    },
    options::Options,
    workspace::Workspace,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn usage() {
    println!("Usage: mforge {VERSION} command [option...]");
    println!("  command:");
    println!("    help                               ; this message");
    println!("    version                            ; mforge version");
    println!();

    println!("    workspace init | env               ; manage workspace");
    println!("    build     release | profile | debug");
    println!("                                       ; build mu system, release default");
    println!("    bench     base | current | report | clean [--ntests=number] [--all]");
    println!("                                       ; benchmark test suite");
    println!("    regression                         ; regression test suite");
    println!("    symbols   reference | crossref | metrics [--namespace=name]");
    println!("                                       ; symbol reports, namespace defaults to mu");
    println!("    install                            ; (sudo) install mu system-wide");
    println!("    clean                              ; clean all artifacts");
    println!("    commit                             ; fmt and clippy, pre-commit checking");
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
        "commit" => Commit::commit(&argv),
        "version" => Options::version(),
        "workspace" => Workspace::workspace(&argv),
        _ => {
            let home = match Workspace::env() {
                Some(path) => path,
                None => {
                    let cwd = std::env::current_dir().unwrap();

                    eprintln!(
                        "error: could not find `.mforge` in {:?} or any parent directory",
                        cwd.to_str().unwrap()
                    );
                    std::process::exit(-1)
                }
            };

            let ws = Workspace::new(&home);
            match command {
                "help" => {
                    println!();
                    println!("    mforge: mu packaging and development tool");
                    println!();
                    usage()
                }
                "bench" => Bench::new(&ws).bench(&argv).unwrap(),
                "build" => Build::build(&argv, &home),
                "clean" => Clean::clean(&argv, &home),
                "install" => Install::install(&argv, &home),
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
