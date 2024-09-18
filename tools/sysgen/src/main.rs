//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#[rustfmt::skip]
use {
    crate::{
        crate_::Crate,
        binding::Binding,
        options::{Options, Opt},
    },
    std::process::Command,
    git2::Repository,
    std::{
        fs,
        path::{Path},
    },
};

mod binding;
mod crate_;
mod item;
mod options;
mod parsers;

pub fn main() {
    fn fetch(name: &str) -> Option<()> {
        let _output = Command::new("cargo")
            .arg("clone")
            .arg(name)
            .output()
            .expect("failed to execute cargo clone for {path}");

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

    let options = Options::parse(&argv).unwrap();

    let workspace = Path::new("mu");
    let workspace_path = workspace.display().to_string();

    match command {
        "init" => {
            let url = "https://github.com/Software-Knife-and-Tool/mu";

            if workspace.exists() {
                eprintln!("sysgen: workspace {workspace:?} already present");
                std::process::exit(-1)
            }

            if options.is_opt("verbose") {
                println!("sysgen: cloning {url} to {workspace_path}")
            }

            let _repo = match Repository::clone(url, workspace) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to clone: {}", e),
            };

            for opt in &options.options {
                if let Opt::Crate(name) = opt {
                    let crate_sysgen = &format!("{name}.sysgen");

                    if options.is_opt("verbose") {
                        println!("sysgen: cloning crate {name}")
                    }

                    fetch(name);

                    match std::fs::create_dir(Path::new(crate_sysgen)) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("sysgen: couldn't create {crate_sysgen} directory {e:?}");

                            std::process::exit(-1)
                        }
                    }
                }
            }
        }
        "bind" => {
            if !Path::new(workspace).exists() {
                eprintln!("sysgen: no workspace present");
                std::process::exit(-1)
            }

            for path in fs::read_dir(".").unwrap() {
                let name = path.unwrap().path().display().to_string();
                if name != "./mu" && !name.contains(".sysgen") {
                    match Crate::with_options(&options, &name, &format!("{name}.sysgen")) {
                        Ok(crate_) => {
                            if options.is_opt("verbose") {
                                println!("sysgen: generating bindings from {name}")
                            }

                            match Binding::emit(&crate_, &options) {
                                Ok(_) => (),
                                Err(e) => {
                                    eprintln!("sysgen emit: failed to parse {}", e);
                                    std::process::exit(-1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("sysgen crate: failed to parse {}", e);
                            std::process::exit(-1);
                        }
                    }
                }
            }
        }
        "symbols" => (),
        "build" => (),
        _ => {
            eprintln!("sysgen: unimplemented command {command}");
            std::process::exit(-1)
        }
    }
}
