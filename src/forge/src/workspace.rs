//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::options::{Mode, Opt, Options},
    std::{
        env, fs, io,
        path::{Path, PathBuf},
    },
};

#[derive(Debug)]
pub struct Workspace {
    pub forge: PathBuf,   // forge module directory
    pub bin: PathBuf,     // workspace bin
    pub lib: PathBuf,     // workspace lib
    pub tests: PathBuf,   // workspace tests
    pub modules: PathBuf, // workspace modules
}

impl Workspace {
    pub fn new(path: &str) -> Self {
        let modules: PathBuf = [path, "src", "forge", "modules"].iter().collect();
        let bin: PathBuf = [path, "dist"].iter().collect();
        let lib: PathBuf = [path, "dist"].iter().collect();
        let tests: PathBuf = [path, "tests"].iter().collect();
        let forge: PathBuf = [path, ".forge"].iter().collect();

        Self {
            modules,
            bin,
            lib,
            forge,
            tests,
        }
    }

    pub fn env() -> Option<String> {
        let mut cwd: PathBuf = std::env::current_dir().unwrap();
        loop {
            match Path::read_dir(&cwd) {
                Ok(mut dir) => match dir.find(|entry| match entry {
                    Ok(entry) => entry.file_name() == ".forge",
                    _ => false,
                }) {
                    Some(_) => return Some(cwd.to_str().unwrap().to_string()),
                    None => (),
                },
                _ => return None,
            }

            cwd = match cwd.parent() {
                Some(path) => path.to_path_buf(),
                None => return None,
            };
        }
    }

    fn forge_dir_exists(forge_home: &str) -> bool {
        let forge_path = PathBuf::from(forge_home.to_owned() + "/.forge");

        Path::exists(&forge_path)
    }

    fn make_module_dirs(forge_home: &str) -> io::Result<()> {
        let module_paths = [
            [forge_home, ".forge", "bench"],
            [forge_home, ".forge", "regression"],
        ];

        for path in module_paths {
            let dir: PathBuf = path.iter().collect();

            fs::create_dir_all(&dir)?;
        }

        Ok(())
    }

    pub fn workspace(argv: &Vec<String>) {
        match Options::parse_options(argv, &["init", "env"], &["verbose", "recipe"]) {
            None => (),
            Some(options) => {
                if options.modes.len() != 1 {
                    panic!()
                }

                let mode = &options.modes[0];

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => {
                        println!(
                            "verbose [workspace {}]:{}",
                            mode.name(),
                            options
                                .options
                                .iter()
                                .map(|option| format!(
                                    " --{}",
                                    Options::opt_name((*option).clone())
                                ))
                                .collect::<String>()
                        )
                    }
                    None => (),
                };

                match mode {
                    Mode::Init => {
                        match Options::find_opt(&options, &Opt::Recipe) {
                            Some(_) => {
                                println!(
                                    "recipe [workspace init]: mkdir {}/.forge",
                                    env::current_dir().unwrap().to_str().unwrap()
                                );
                                return ();
                            }
                            None => (),
                        };

                        if Self::forge_dir_exists("./") {
                            eprintln!("workspace already initted.");
                        }

                        Self::make_module_dirs("./").unwrap()
                    }
                    Mode::Env => println!("{}", Self::env().unwrap()),
                    _ => panic!(),
                }
            }
        }
    }
}
