//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
mod bench;
mod build;
mod clean;
mod commit;
mod env;
mod image;
mod init;
mod install;
mod options;
mod symbols;
mod test;

#[rustfmt::skip]
use {
    crate::{
        bench::Bench,
        build::Build,
        clean::Clean,
        commit::Commit,
        env::Env,
        image::image::Image,
        init::Init,
        install::Install,
        options::Options,
        symbols::Symbols,
        test::Test,
    },
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn usage() {
    println!("Usage: lade {VERSION} command [option...]");
    println!("  command:");
    println!("    help                               ; this message");
    println!("    version                            ; lade version");
    println!();
    println!("    init                               ; init");
    println!("    env                                ; print development environment");
    println!("    build     release | profile | debug");
    println!("                                       ; build mu system, release default");
    println!("    image     build --out=path | [--image=path | -config=config] *[--load=path | --eval=sexpr]] |");
    println!("              view --image=path");
    println!("                                       ; manage heap images");
    println!("    symbols   reference [--module=name] |");
    println!("              crossref [--module=name]  |");
    println!("              metrics [--module=name]");
    println!("                                       ; symbol reports, default to mu");
    println!("    install                            ; (sudo) install mu system-wide");
    println!("    clean                              ; clean all artifacts");
    println!("    commit                             ; fmt and clippy, pre-commit checking");
    println!("    test                               ; regression test suite");
    println!("    bench     base | current | footprint [--ntests=number]");
    println!();
    println!("  general options:");
    println!("    --verbose                          ; verbose operation");

    std::process::exit(0);
}

pub fn main() {
    let argv = std::env::args().collect::<Vec<String>>();

    if argv.len() == 1 {
        usage();
    }

    let command = argv[1].as_str();

    match command {
        "init" => Init::init(&argv),
        "commit" => Commit::commit(&argv),
        "version" => Options::version(),
        _ => {
            let home = match Env::mu_home() {
                Some(path) => path,
                None => {
                    let cwd = std::env::current_dir().unwrap();

                    eprintln!(
                        "error: could not find `.lade` in {:?} or any parent directory",
                        cwd.to_str().unwrap()
                    );
                    std::process::exit(-1)
                }
            };

            match command {
                "help" => {
                    println!();
                    println!("    mu implementation explorer");
                    println!();
                    usage()
                }
                "bench" => Bench::bench(&argv, &home),
                "build" => Build::build(&argv, &home),
                "clean" => Clean::clean(&argv, &home),
                "env" => Env::printenv(&argv, &home),
                "image" => Image::image(&argv, &home),
                "install" => Install::install(&argv, &home),
                "symbols" => Symbols::symbols(&argv, &home),
                "test" => Test::test(&argv, &home),
                _ => {
                    eprintln!("lade: unimplemented command {command}");
                    std::process::exit(-1)
                }
            }
        }
    }
}
