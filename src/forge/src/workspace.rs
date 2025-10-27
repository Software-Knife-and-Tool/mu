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
    pub workspace: PathBuf, // workspace directory
    pub forge: PathBuf,     // workspace forge directory
    pub module: PathBuf,    // module scripts directory
}

impl Workspace {
    pub fn new(home: &str) -> Self {
        let workspace: PathBuf = home.into();
        let forge: PathBuf = [home, ".forge"].iter().collect();
        let module: PathBuf = [home, "src", "forge", "modules"].iter().collect();

        Self {
            workspace,
            forge,
            module,
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

    fn make_forge_dirs(forge_home: &str) -> io::Result<()> {
        let forge_paths = [
            [forge_home, ".forge", "tests", "performance"],
            [forge_home, ".forge", "tests", "regression"],
            [forge_home, ".forge", "tests", "footprint"],
        ];

        for path in forge_paths {
            let dir: PathBuf = path.iter().collect();

            fs::create_dir_all(&dir)?
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

                        Self::make_forge_dirs("./").unwrap()
                    }
                    Mode::Env => println!("{}", Self::env().unwrap()),
                    _ => panic!(),
                }
            }
        }
    }
}
