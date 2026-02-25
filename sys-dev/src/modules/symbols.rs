//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::{
        options::{Mode, Opt, Options},
        workspace::Workspace,
    },
    std::{
        io::{self, Write},
        path::PathBuf,
        process::Command,
    },
    tempfile::NamedTempFile,
};

#[derive(Debug)]
pub struct Symbols {
    module: PathBuf,   // module scripts directory
    core_sys: PathBuf, // core.sys path
    mu_sys: PathBuf,   // mu-sys path
}

impl Symbols {
    pub fn new(ws: &Workspace) -> Self {
        let module = Options::add_path(&mut ws.modules.clone(), "symbols");
        let core_sys = Options::add_path(&mut ws.lib.clone(), "core.sys");
        let mu_sys = Options::add_path(&mut ws.bin.clone(), "mu-sys");

        Self {
            module,
            core_sys,
            mu_sys,
        }
    }

    pub fn symbols(&self, argv: &Vec<String>) -> io::Result<()> {
        match Options::parse_options(
            argv,
            &["crossref", "metrics", "reference", "clean"],
            &["namespace", "verbose", "recipe"],
        ) {
            None => Ok(()),
            Some(options) => {
                if options.modes.len() != 1 {
                    panic!()
                }

                let mode = &options.modes[0];

                let ns = match Options::opt_value(&options, &Opt::Namespace("".to_string())) {
                    Some(ns) => ns,
                    None => "mu".to_string(),
                };

                if Options::find_opt(&options, &Opt::Verbose).is_some() {
                    println!("[symbols {:?}] --verbose", mode)
                }

                if Options::find_opt(&options, &Opt::Recipe).is_some() {
                    println!("[symbols {:?}] --verbose", mode)
                }

                match mode {
                    Mode::Crossref => {
                        let tmp_file = NamedTempFile::new().unwrap();

                        let output = Command::new(&self.mu_sys)
                            .args(["-l", &self.core_sys.to_str().unwrap()])
                            .args([
                                "-l",
                                &Options::add_path(&mut self.module.clone(), "crossref.l")
                                    .to_str()
                                    .unwrap(),
                            ])
                            .args(["-q", &format!("(symbols:crossref {:?})", tmp_file.path())])
                            .output()
                            .expect("command failed to execute");

                        io::stderr().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();

                        let output = Command::new("python3")
                            .arg(&Options::add_path(&mut self.module.clone(), "crossref.py"))
                            .arg(&tmp_file.path())
                            .output()
                            .expect("command failed to execute");

                        io::stderr().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    Mode::Metrics => {
                        let tmp_file = NamedTempFile::new().unwrap();

                        let output = Command::new(&self.mu_sys)
                            .args(["-l", &self.core_sys.to_str().unwrap()])
                            .args([
                                "-l",
                                &Options::add_path(&mut self.module.clone(), "metrics.l")
                                    .to_str()
                                    .unwrap(),
                            ])
                            .args([
                                "-q",
                                &format!("(symbols:metrics \"core\" {:?})", tmp_file.path()),
                            ])
                            .output()
                            .expect("command failed to execute");

                        io::stderr().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();

                        let output = Command::new("python3")
                            .arg(&Options::add_path(&mut self.module.clone(), "metrics.py"))
                            .arg(&self.core_sys)
                            .arg(&tmp_file.path())
                            .output()
                            .expect("command failed to execute");

                        io::stderr().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    Mode::Reference => {
                        let tmp_file = NamedTempFile::new().unwrap();

                        let output = Command::new(&self.mu_sys)
                            .args(["-l", &self.core_sys.to_str().unwrap()])
                            .args([
                                "-l",
                                &Options::add_path(&mut self.module.clone(), "reference.l")
                                    .to_str()
                                    .unwrap(),
                            ])
                            .args([
                                "-q",
                                &format!("(symbols:reference \"{ns}\" {:?})", tmp_file.path()),
                            ])
                            .output()
                            .expect("command failed to execute");

                        io::stderr().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();

                        let output = Command::new("python3")
                            .arg(&Options::add_path(&mut self.module.clone(), "reference.py"))
                            .arg(&tmp_file.path())
                            .output()
                            .expect("command failed to execute");

                        io::stderr().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    Mode::Clean => {}
                    _ => panic!(),
                }

                Ok(())
            }
        }
    }
}
