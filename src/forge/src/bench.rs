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
    bench: PathBuf,    // module scripts directory
    core_sys: PathBuf, // core.sys path
    mu_sys: PathBuf,   // mu-sys path
    report: PathBuf,   // module report path
    tests: PathBuf,    // tests directory
}

impl Bench {
    pub fn new(ws: &Workspace) -> Self {
        let bench = Self::push_path(&mut ws.modules.clone(), "bench");
        let core_sys = Self::push_path(&mut ws.lib.clone(), "core.sys");
        let mu_sys = Self::push_path(&mut ws.bin.clone(), "mu-sys");
        let report = Self::push_path(&mut ws.forge.clone(), "bench");
        let tests = Self::push_path(&mut ws.tests.clone(), "performance");

        Self {
            bench,
            core_sys,
            mu_sys,
            report,
            tests,
        }
    }

    fn push_path(path: &mut PathBuf, component: &str) -> PathBuf {
        path.push(component);

        (&path).into()
    }

    fn run_perf(&self, script: &str, group: &str, to: &str, ntests: usize) {
        let script_path = Self::push_path(&mut self.bench.clone(), script);
        let json_path = Self::push_path(&mut self.report.clone(), to);

        let mut json_file = File::create(&json_path).expect(&format!("{json_path:?}"));

        let output = Command::new("python3")
            .arg(&script_path)
            .arg(&self.mu_sys)
            .arg(&self.core_sys)
            .arg(&self.bench)
            .arg(group)
            .arg(&self.tests)
            .arg(ntests.to_string())
            .output()
            .expect("command failed to execute");

        json_file.write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    fn run_footprint(&self, script: &str, to: &str, ntests: usize) {
        let script_path = Self::push_path(&mut self.bench.clone(), script);
        let json_path = Self::push_path(&mut self.report.clone(), to);

        let mut json_file = File::create(&json_path).expect(&format!("{json_path:?}"));

        let output = Command::new("python3")
            .arg(&script_path)
            .arg(&self.mu_sys)
            .arg(&self.core_sys)
            .arg(ntests.to_string())
            .output()
            .expect("command failed to execute");

        json_file.write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    pub fn bench(&self, argv: &Vec<String>) {
        match Options::parse_options(
            argv,
            &["base", "current", "report", "clean"],
            &["all", "ntests", "verbose", "recipe"],
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
                    Some(_) => {
                        println!("[bench {:?}] --ntests {ntests} --verbose", mode)
                    }
                    None => (),
                }

                let all = match Options::find_opt(&options, &Opt::All) {
                    Some(_) => true,
                    None => false,
                };

                match Options::find_opt(&options, &Opt::Recipe) {
                    Some(_) => println!("[bench {:?}] --ntests {ntests} --verbose", mode),
                    None => (),
                };

                match mode {
                    Mode::Base => {
                        self.run_perf("perf-group.py", "mu", "base.mu.json", ntests);
                        self.run_perf("perf-group.py", "core", "base.core.json", ntests);
                        self.run_perf("perf-group.py", "frequent", "base.frequent.json", ntests);
                        self.run_footprint("perf-footprint.py", "base.footprint.json", ntests);
                    }
                    Mode::Current => {
                        self.run_perf("perf-group.py", "mu", "current.mu.json", ntests);
                        self.run_perf("perf-group.py", "core", "current.core.json", ntests);
                        self.run_perf("perf-group.py", "frequent", "current.frequent.json", ntests);
                        self.run_footprint("perf-footprint.py", "current.footprint.json", ntests);
                    }
                    Mode::Report => {
                        self.bench_report();
                        if all {
                            self.footprint_report()
                        }
                    }
                    Mode::Clean => {}
                    _ => panic!(),
                }
            }
        }
    }

    pub fn bench_report(&self) {
        let json_script_path = Self::push_path(&mut self.bench.clone(), "report-group.py");
        let report_script_path = Self::push_path(&mut self.bench.clone(), "report.py");
        let base_report_path = Self::push_path(&mut self.report.clone(), "base.report");
        let mut base_report_file = File::create(&base_report_path).unwrap();
        let current_report_path = Self::push_path(&mut self.report.clone(), "current.report");
        let mut current_report_file = File::create(&current_report_path).unwrap();

        for group in ["mu", "frequent", "core"] {
            let path = Self::push_path(&mut self.report.clone(), &format!("base.{group}.json"));

            if !path.exists() {
                eprintln!(
                    "{}",
                    &format!(
                        "bench report: {} not found, run bench base",
                        path.to_str().unwrap()
                    )
                );
                std::process::exit(-1)
            }

            let output = Command::new("python3")
                .arg(&json_script_path)
                .arg(&path)
                .output()
                .expect("command failed to execute");

            base_report_file.write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }

        for group in ["mu", "frequent", "core"] {
            let path = Self::push_path(&mut self.report.clone(), &format!("current.{group}.json"));

            if !path.exists() {
                eprintln!(
                    "{}",
                    &format!(
                        "bench report: {} not found, run bench current",
                        path.to_str().unwrap()
                    )
                );
                std::process::exit(-1)
            }

            if !path.exists() {
                panic!()
            }

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

    pub fn footprint_report(&self) {
        let json_script_path = Self::push_path(&mut self.bench.clone(), "report-footprint.py");

        let base_json_path = Self::push_path(&mut self.report.clone(), "base.footprint.json");
        let current_json_path = Self::push_path(&mut self.report.clone(), "current.footprint.json");

        let base_report_path = Self::push_path(&mut self.report.clone(), "base.footprint.report");
        let current_report_path =
            Self::push_path(&mut self.report.clone(), "current.footprint.report");

        let mut base_report_file = File::create(&base_report_path).unwrap();
        let mut current_report_file = File::create(&current_report_path).unwrap();

        if !base_json_path.exists() {
            eprintln!(
                "{}",
                &format!(
                    "bench report: {} not found, run bench base",
                    base_json_path.to_str().unwrap()
                )
            );
            std::process::exit(-1)
        }

        if !current_json_path.exists() {
            eprintln!(
                "{}",
                &format!(
                    "bench report: {} not found, run bench current",
                    current_json_path.to_str().unwrap()
                )
            );
            std::process::exit(-1)
        }

        let output = Command::new("python3")
            .arg(&json_script_path)
            .arg(&base_json_path)
            .output()
            .expect("command failed to execute");

        base_report_file.write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        let output = Command::new("python3")
            .arg(&json_script_path)
            .arg(&current_json_path)
            .output()
            .expect("command failed to execute");

        current_report_file.write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        let output = Command::new("paste")
            .arg(&base_report_path)
            .arg(&current_report_path)
            .output()
            .expect("command failed to execute");

        io::stderr().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
