//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
mod annotate;
mod bench;
mod build;
mod clean;
mod commit;
mod env;
mod image;
mod init;
mod install;
mod options;
mod profile;
mod repl;
mod symbols;
mod test;

#[rustfmt::skip]
use {
    crate::{
        annotate::Annotate,
        bench::Bench,
        build::Build,
        clean::Clean,
        commit::Commit,
        env::Env,
        image::image::Image,
        init::Init,
        install::Install,
        options::Options,
        profile::Profile,
        repl::Repl,
        symbols::Symbols,
        test::Test,
    },
};

const VERSION: &str = "0.0.15";

pub fn usage() {
    println!("Usage: mux {} command [option...]", VERSION);
    println!("  command:");
    println!("    help                               ; this message");
    println!("    version                            ; mux version");
    println!();
    println!("    init                               ; init");
    println!("    env                                ; print development environment");
    println!("    build     release | profile | debug");
    println!("                                       ; build mu system, release is default");
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
    println!("    repl      mu | core | common | prelude");
    println!("                                       ; repl: mu, core, common with prelude, and prelude repls");
    println!("    test                               ; regression test suite");
    println!("    bench     base | current | footprint [--ntests=number]");
    println!("    profile   --config=path            ; create profile");
    println!("    annotate  --prof=path [--ref=path] ; annotate profile");
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
                        "error: could not find `.mux` in {:?} or any parent directory",
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
                "annotate" => Annotate::annotate(&argv, &home),
                "bench" => Bench::bench(&argv, &home),
                "build" => Build::build(&argv, &home),
                "clean" => Clean::clean(&argv, &home),
                "env" => Env::printenv(&argv, &home),
                "image" => Image::image(&argv, &home),
                "install" => Install::install(&argv, &home),
                "profile" => Profile::profile(&argv, &home),
                "repl" => Repl::repl(&argv, &home),
                "symbols" => Symbols::symbols(&argv, &home),
                "test" => Test::test(&argv, &home),
                _ => {
                    eprintln!("mux: unimplemented command {command}");
                    std::process::exit(-1)
                }
            }
        }
    }
}
