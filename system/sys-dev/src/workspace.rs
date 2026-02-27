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
    pub bin: PathBuf,     // workspace bin
    pub lib: PathBuf,     // workspace lib
    pub sysdev: PathBuf,  // sys-dev module directory
    pub modules: PathBuf, // workspace modules
    pub tests: PathBuf,   // workspace tests
}

impl Workspace {
    pub fn new(path: &str) -> Self {
        let modules: PathBuf = [path, "system", "sys-dev", "modules"].iter().collect();
        let bin: PathBuf = [path, "target", "release"].iter().collect();
        let lib: PathBuf = [path, "dist"].iter().collect();
        let tests: PathBuf = [path, "tests"].iter().collect();
        let sysdev: PathBuf = [path, ".sys-dev"].iter().collect();

        Self {
            modules,
            bin,
            lib,
            sysdev,
            tests,
        }
    }

    pub fn env() -> Option<String> {
        let mut cwd: PathBuf = std::env::current_dir().unwrap();
        loop {
            match Path::read_dir(&cwd) {
                Ok(mut dir) => match dir.find(|entry| match entry {
                    Ok(entry) => entry.file_name() == ".sys-dev",
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

    fn sysdev_dir_exists(home: &str) -> bool {
        let path = PathBuf::from(home.to_owned() + "/.sys-dev");

        Path::exists(&path)
    }

    fn make_module_dirs(home: &str) -> io::Result<()> {
        let module_paths = [
            [home, ".sys-dev", "bench"],
            [home, ".sys-dev", "regression"],
            [home, ".sys-dev", "symbols"],
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

                        if Self::sysdev_dir_exists("./") {
                            eprintln!("workspace already initted.");
                        }

                        Self::make_module_dirs("./").unwrap()
                    }
                    Mode::Env => match Self::env() {
                        Some(env) => println!("{env}"),
                        None => eprintln!("not in a sys-dev workspace`"),
                    },
                    _ => panic!(),
                }
            }
        }
    }
}
