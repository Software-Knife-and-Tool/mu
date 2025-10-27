//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::{
        options::{Mode, Opt, Options},
        workspace::Workspace,
    },
    std::{
        fs::File,
        io::{self, Write},
        path::PathBuf,
        process::{Command, Stdio},
    },
    tempfile::NamedTempFile,
};

#[derive(Debug)]
pub struct Bench {
    mu_sys: PathBuf,      // mu-sys path
    core: PathBuf,        // core.sys path
    performance: PathBuf, // bench
    footprint: PathBuf,   // footprint
    module: PathBuf,      // module scripts directory
    tests: PathBuf,       // tests directory
}

impl Bench {
    pub fn new(ws: &Workspace) -> Self {
        let home = ws.workspace.as_path().to_str().unwrap();
        let tests: PathBuf = [home, "tests", "performance"].iter().collect();
        let dist_path: PathBuf = [home, "dist"].iter().collect::<PathBuf>();
        let test_path: PathBuf = Self::push_path(&mut ws.forge.clone(), "tests");

        let core: PathBuf = Self::push_path(&mut dist_path.clone(), "core.sys");
        let footprint: PathBuf = Self::push_path(&mut test_path.clone(), "footprint");
        let module: PathBuf = Self::push_path(&mut ws.module.clone(), "bench");
        let mu_sys: PathBuf = Self::push_path(&mut dist_path.clone(), "mu-sys");
        let performance: PathBuf = Self::push_path(&mut test_path.clone(), "performance");

        Self {
            core,
            footprint,
            module,
            mu_sys,
            performance,
            tests,
        }
    }

    fn push_path(path: &mut PathBuf, component: &str) -> PathBuf {
        path.push(component);

        (&path).into()
    }

    fn run_perf(&self, script: &str, ns: &str, to: &str, ntests: usize) {
        let script_path = Self::push_path(&mut self.module.clone(), script);
        let json_path = Self::push_path(&mut self.performance.clone(), to);
        let mut json_file = File::create(&json_path).unwrap();

        let output = Command::new("python3")
            .arg(&script_path)
            .arg(&self.mu_sys)
            .arg(&self.core)
            .arg(&self.module)
            .arg(ns)
            .arg(&self.tests)
            .arg(ntests.to_string())
            .output()
            .expect("command failed to execute");

        json_file.write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    pub fn bench(&self, argv: &Vec<String>) {
        match Options::parse_options(
            argv,
            &["base", "current", "footprint", "report"],
            &["namespace", "ntests", "verbose", "recipe"],
        ) {
            None => (),
            Some(options) => {
                if options.modes.len() != 1 {
                    panic!()
                }

                let mode = &options.modes[0];

                let ntests = match Options::opt_value(&options, &Opt::Ntests("".to_string())) {
                    Some(n) => n.parse().unwrap(),
                    None => 20usize,
                };

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("[bench {:?}] --ntests {ntests} --verbose", mode),
                    None => (),
                };

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("[bench {:?}] --ntests {ntests} --verbose", mode),
                    None => (),
                };

                let _ns = Options::opt_value(&options, &Opt::Namespace("".to_string()));

                match mode {
                    Mode::Base => {
                        self.run_perf("perf-ns.py", "mu", "base.mu.json", ntests);
                        self.run_perf("perf-ns.py", "core", "base.core.json", ntests);
                        self.run_perf("perf-ns.py", "frequent", "base.frequent.json", ntests);
                    }
                    Mode::Current => {
                        self.run_perf("perf-ns.py", "mu", "current.mu.json", ntests);
                        self.run_perf("perf-ns.py", "core", "current.core.json", ntests);
                        self.run_perf("perf-ns.py", "frequent", "current.frequent.json", ntests);
                    }
                    Mode::Report => self.report(),
                    //  Mode::Footprint => self.footprint(ntests, home),
                    _ => panic!(),
                }
            }
        }
    }

    pub fn report(&self) {
        let json_script_path = Self::push_path(&mut self.module.clone(), "report-ns.py");
        let report_script_path = Self::push_path(&mut self.module.clone(), "report.py");
        let base_report_path = Self::push_path(&mut self.performance.clone(), "base.report");
        let mut base_report_file = File::create(&base_report_path).unwrap();
        let current_report_path = Self::push_path(&mut self.performance.clone(), "current.report");
        let mut current_report_file = File::create(&current_report_path).unwrap();

        for ns in ["mu", "frequent", "core"] {
            let path = Self::push_path(&mut self.performance.clone(), &format!("base.{ns}.json"));

            let output = Command::new("python3")
                .arg(&json_script_path)
                .arg(&path)
                .output()
                .expect("command failed to execute");

            base_report_file.write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }

        for ns in ["mu", "frequent", "core"] {
            let path =
                Self::push_path(&mut self.performance.clone(), &format!("current.{ns}.json"));

            let output = Command::new("python3")
                .arg(&json_script_path)
                .arg(&path)
                .output()
                .expect("command failed to execute");

            current_report_file.write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }

        let sed_child = Command::new("sed")
            .args(["-e", "1,$s/^.. .[^ ]*.[ ]*//"])
            .arg(&current_report_path)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let paste_child = Command::new("paste")
            .arg(&base_report_path)
            .arg("-")
            .stdin(Stdio::from(sed_child.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let pipe_child = Command::new("sed")
            .args(["-e", "1,$s/^.. //"])
            .stdin(Stdio::from(paste_child.stdout.unwrap()))
            .stdout(Stdio::piped())
            .output()
            .unwrap();

        let mut report_tmp_file = NamedTempFile::new().unwrap();
        report_tmp_file.write_all(&pipe_child.stdout).unwrap();

        let output = Command::new("python3")
            .arg(&report_script_path)
            .arg(report_tmp_file.path())
            .output()
            .expect("command failed to execute");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    /*
        pub fn footprint(&self, _ntests: usize, home: &str) {
            println!("footprint: {home}");
            let output = Command::new("make")
                .current_dir(home)
                .args(["-C", "tests/footprint"])
                .arg("current")
                .arg("--no-print-directory")
                .output()
                .expect("command failed to execute");

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();

            let output = Command::new("make")
                .current_dir(home)
                .args(["-C", "tests/footprint"])
                .arg("report")
                .arg("--no-print-directory")
                .output()
                .expect("command failed to execute");

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
    }
        */
}
