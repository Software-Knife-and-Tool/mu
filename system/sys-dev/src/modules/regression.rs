//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::{
        options::{Opt, Options},
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
pub struct Regression {
    module: PathBuf,   // module scripts directory
    core_sys: PathBuf, // core.sys path
    mu_sys: PathBuf,   // mu-sys path
    tests: PathBuf,    // tests directory
}

impl Regression {
    pub fn new(ws: &Workspace) -> Self {
        let module = Options::add_path(&mut ws.modules.clone(), "regression");
        let core_sys = Options::add_path(&mut ws.lib.clone(), "core.sys");
        let mu_sys = Options::add_path(&mut ws.bin.clone(), "mu-sys");
        let tests = Options::add_path(&mut ws.tests.clone(), "regression");

        Self {
            module,
            core_sys,
            mu_sys,
            tests,
        }
    }

    fn test_ns(&self, ns: &str) {
        let output = Command::new("python3")
            .arg(&Options::add_path(&mut self.module.clone(), "test-ns.py"))
            .arg(&self.mu_sys)
            .arg(&self.tests)
            .arg(ns)
            .output()
            .expect("command failed to execute");

        let mut report_tmp_file = NamedTempFile::new().unwrap();
        report_tmp_file.write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        let output = Command::new("python3")
            .arg(&Options::add_path(
                &mut self.module.clone(),
                "summarize-ns.py",
            ))
            .arg(&report_tmp_file.path())
            .output()
            .expect("command failed to execute");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    fn test_module(&self, module: &str) {
        let output = Command::new("python3")
            .arg(&Options::add_path(
                &mut self.module.clone(),
                "test-module.py",
            ))
            .arg(&self.mu_sys)
            .arg(&self.core_sys)
            .arg(module)
            .arg(&self.tests)
            .output()
            .expect("command failed to execute");

        let mut report_tmp_file = NamedTempFile::new().unwrap();
        report_tmp_file.write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        let output = Command::new("python3")
            .arg(&Options::add_path(
                &mut self.module.clone(),
                "summarize-module.py",
            ))
            .arg(&report_tmp_file.path())
            .output()
            .expect("command failed to execute");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    pub fn regression(&self, argv: &Vec<String>) -> io::Result<()> {
        match Options::parse_options(argv, &[], &["verbose", "recipe"]) {
            None => Ok(()),
            Some(options) => {
                if options.modes.len() != 0 {
                    panic!()
                }

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => {
                        println!("[test] --verbose")
                    }
                    None => (),
                }

                match Options::find_opt(&options, &Opt::Recipe) {
                    Some(_) => println!("[tests] --recipe"),
                    None => (),
                };

                self.test_ns("mu");
                self.test_ns("format");
                self.test_module("core");

                Ok(())
            }
        }
    }
}
