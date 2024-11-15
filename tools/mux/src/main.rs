//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
mod annotate;
mod bench;
mod build;
mod clean;
mod commit;
mod env;
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
        install::Install,
        init::Init,
        options::Options,
        profile::Profile,
        repl::Repl,
        symbols::Symbols,
        test::Test,
    },
};

const VERSION: &str = "0.0.10";

pub fn usage() {
    println!("Usage: mux {} command [option...]", VERSION);
    println!("  command:");
    println!("    help                               ; this message");
    println!("    version                            ; mux version");
    println!();
    println!("    init                               ; init");
    println!("    env                                ; mu development environment");
    println!("    build     [--release | --profile | --debug]");
    println!("                                       ; build mu system, debug is default");
    println!("    install                            ; (sudo) install mu system-wide");
    println!("    clean                              ; clean all artifacts");
    println!("    commit                             ; fmt and clippy, pre-commit checking");
    println!("    repl      [--namespace ...]        ; repl");
    println!("    symbols   [--crossref | --counts | --reference | --namespace ...]");
    println!("                                       ; symbol reports");
    println!("    test                               ; regression test suite");
    println!("    bench     [[--base | --current | --footprint] | --ntests ...]");
    println!("    profile   [--config ...]           ; create profile");
    println!("    annotate  [--prof ... | --ref ...] ; annotate profile");
    println!();
    println!("  general options:");
    println!("    --verbose                          ; verbose operation");
    println!("    --output ...                       ; output file path");

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
        "init" => Init::init(&options),
        _ => {
            let home = match Env::mu_home(&options) {
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
                "annotate" => Annotate::annotate(&options, &home),
                "bench" => Bench::bench(&options, &home),
                "build" => Build::build(&options, &home),
                "clean" => Clean::clean(&options, &home),
                "commit" => Commit::commit(&options, &home),
                "env" => Env::printenv(&options, &home),
                "install" => Install::install(&options, &home),
                "profile" => Profile::profile(&options, &home),
                "repl" => Repl::repl(&options, &home),
                "symbols" => Symbols::symbols(&options, &home),
                "test" => Test::test(&options, &home),
                "version" => Options::version(),
                _ => {
                    eprintln!("mux: unimplemented command {command}");
                    std::process::exit(-1)
                }
            }
        }
    }
}
