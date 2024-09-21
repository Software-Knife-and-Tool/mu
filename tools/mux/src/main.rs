//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
mod annotate;
mod bench;
mod build;
mod clean;
mod commit;
mod env;
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
        install::Install,
        options::Options,
        profile::Profile,
        repl::Repl,
        symbols::Symbols,
        test::Test,
    },
};

const VERSION: &str = "0.0.8";

pub fn usage() {
    println!("Usage: mux {} command [option...]", VERSION);
    println!("  command:");
    println!("    help                               ; this message");
    println!("    version                            ; mux version");
    println!("    env                                ; mu development environment");
    println!("    build     [--release | --profile | --debug]");
    println!("                                       ; build mu system, debug is default");
    println!("    clean                              ; clean all artifacts");
    println!("    commit                             ; fmt and clippy, pre-commit checking");
    println!("    install                            ; (sudo) install mu system-wide");
    println!("    repl      [--namespace ...]        ; repl");
    println!("    symbols   [--crossref | --counts | --reference | --namespace]");
    println!("                                       ; symbol reports");
    println!("    test                               ; regression test suite");
    println!("    bench     [[--base | --current | --footprint] | --ntests]");
    println!("    profile   [--config]               ; create profile");
    println!("    annotate  [--prof ... | --ref ...] ; annotate profile");
    println!();
    println!("  general options:");
    println!("    --verbose                          ; verbose operation");
    println!("    --output path                      ; output file path");

    std::process::exit(0);
}

pub fn main() {
    let argv = std::env::args().collect::<Vec<String>>();

    if argv.len() == 1 {
        usage();
    }

    let command = argv[1].as_str();
    let options = Options::parse(&argv).unwrap();

    match command {
        "help" => {
            println!();
            println!("    mu implementation explorer");
            println!();
            usage()
        }
        "annotate" => Annotate::annotate(&options),
        "bench" => Bench::bench(&options),
        "build" => Build::build(&options),
        "clean" => Clean::clean(&options),
        "commit" => Commit::commit(&options),
        "env" => Env::printenv(&options),
        "install" => Install::install(&options),
        "profile" => Profile::profile(&options),
        "repl" => Repl::repl(&options),
        "symbols" => Symbols::symbols(&options),
        "test" => Test::test(&options),
        "version" => Options::version(),
        _ => {
            eprintln!("mux: unimplemented command {command}");
            std::process::exit(-1)
        }
    }
}
