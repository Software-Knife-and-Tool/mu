//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

#[rustfmt::skip]
use {
    crate::{
        crate_::Crate,
        options::{Options, Opt},
        sysgen::Sysgen,
    },
    std::process::Command,
    git2::Repository,
    std::{
        fs,
        path::Path,
    },
};

mod crate_;
mod options;
mod symbol_table;
mod sysgen;

pub fn main() {
    fn fetch(name: &str, path: &str) -> Option<()> {
        let _output = Command::new("cargo")
            .current_dir(path)
            .arg("clone")
            .arg(name)
            .output()
            .expect("failed to execute process");

        Some(())
    }

    let argv = std::env::args().collect::<Vec<String>>();

    if argv.len() == 1 {
        Options::usage();
    }

    let command = argv[1].as_str();

    match command {
        "--help" => Options::usage(),
        "--version" => Options::version(),
        _ => (),
    }

    let sysgen = match Options::with_options(&argv) {
        Some(opts) => {
            let workspace = match opts.opt_value("workspace") {
                Some(path) => path,
                None => "mu".to_string(),
            };

            Sysgen::new(opts, workspace)
        }
        None => std::process::exit(0),
    };

    let workspace = &sysgen.workspace;

    match command {
        "clone" => {
            let url = "https://github.com/Software-Knife-and-Tool/mu";

            if Path::new(workspace).exists() {
                eprintln!("sysgen: workspace {workspace} already present");
                std::process::exit(-1)
            }

            if sysgen.options.is_opt("verbose") {
                println!("sysgen: cloning {url} to {workspace}")
            }

            let _repo = match Repository::clone(url, workspace) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to clone: {}", e),
            };

            let sysgen_path = &format!("{workspace}/{}", Sysgen::BINDINGS);

            match std::fs::create_dir(Path::new(sysgen_path)) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("sysgen: couldn't create bindings directory {e:?}");

                    std::process::exit(-1)
                }
            }
        }
        "generate" => {
            let options = &sysgen.options;

            if !Path::new(workspace).exists() {
                eprintln!("sysgen: no workspace present");
                std::process::exit(-1)
            }

            let sysgen_path = &format!("{workspace}/{}", Sysgen::BINDINGS);

            if !Path::new(sysgen_path).exists() {
                eprintln!("sysgen: no workspace bindings directory present");
                std::process::exit(-1)
            }

            for opt in &options.options {
                if let Opt::Crate(name) = opt {
                    fetch(name, sysgen_path);

                    let crate_src = &format!("{sysgen_path}/{name}");
                    let crate_sysgen = &format!("{sysgen_path}/{name}.sysgen");

                    match std::fs::create_dir(Path::new(crate_sysgen)) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("sysgen: couldn't create {name}.sysgen directory {e:?}");

                            std::process::exit(-1)
                        }
                    }

                    match Crate::with_options(options, name, crate_src, crate_sysgen) {
                        Ok(crate_) => {
                            if options.is_opt("verbose") {
                                println!("sysgen: generating from {name}")
                            }

                            sysgen.generate(&crate_)
                        }
                        Err(e) => {
                            eprintln!("sysgen: failed to parse {}", e);
                            std::process::exit(-1);
                        }
                    }
                }
            }
        }
        "image" => (),
        "toc" => {
            let options = &sysgen.options;

            if !Path::new(workspace).exists() {
                eprintln!("sysgen: no workspace present");
                std::process::exit(-1)
            }

            let sysgen_path = &format!("{workspace}/{}", Sysgen::BINDINGS);

            if !Path::new(sysgen_path).exists() {
                eprintln!("sysgen: no workspace bindings directory present");
                std::process::exit(-1)
            }

            for opt in &options.options {
                if let Opt::Crate(name) = opt {
                    let dir = format!("{sysgen_path}/{name}.sysgen");
                    println!("{name} contents: {dir}");
                    for entry in fs::read_dir(dir).unwrap() {
                        match entry {
                            Ok(dir) => println!("  {:?}", dir.path()),
                            Err(_) => {
                                println!("can't read bindings directory");
                                std::process::exit(-1);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            eprintln!("sysgen: unimplemented command {command}");
            std::process::exit(-1)
        }
    }
}
