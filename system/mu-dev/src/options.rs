//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
use {crate::VERSION, std::path::PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub struct Options {
    pub modes: Vec<Mode>,
    pub options: Vec<Opt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Opt {
    All,
    Namespace(String),
    Ntests(String),
    Prof(String),
    Recipe,
    Ref(String),
    Verbose,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Mode {
    Base,
    Build,
    Clean,
    Common,
    Core,
    Crossref,
    Current,
    Debug,
    Env,
    Init,
    Metrics,
    Mu,
    Prelude,
    Profile,
    Show,
    Reference,
    Release,
    Report,
    View,
}

impl Mode {
    pub fn name(&self) -> &str {
        match self {
            Mode::Base => "base",
            Mode::Build => "build",
            Mode::Clean => "clean",
            Mode::Core => "core",
            Mode::Common => "common",
            Mode::Crossref => "crossref",
            Mode::Current => "current",
            Mode::Debug => "debug",
            Mode::Env => "env",
            Mode::Init => "init",
            Mode::Metrics => "metrics",
            Mode::Mu => "mu",
            Mode::Prelude => "prelude",
            Mode::Profile => "profile",
            Mode::Reference => "reference",
            Mode::Release => "release",
            Mode::Report => "report",
            Mode::View => "view",
            _ => panic!(),
        }
    }
}

impl Options {
    pub fn version() {
        println!("{}", VERSION);
        std::process::exit(0)
    }

    pub fn add_path(path: &mut PathBuf, component: &str) -> PathBuf {
        path.push(component);

        (&path).into()
    }

    pub fn find_opt(options: &Options, opt: &Opt) -> Option<Opt> {
        match options
            .options
            .iter()
            .find(|next| std::mem::discriminant(*next) == std::mem::discriminant(opt))
        {
            None => None,
            Some(opt) => Some(opt.clone()),
        }
    }

    pub fn opt_value(options: &Options, opt: &Opt) -> Option<String> {
        match Self::find_opt(&options, opt) {
            Some(opt) => match opt {
                Opt::Namespace(str) | Opt::Prof(str) | Opt::Ref(str) | Opt::Ntests(str) => {
                    Some(str.to_string())
                }
                _ => panic!(),
            },
            None => None,
        }
    }

    pub fn opt_name(opt: Opt) -> String {
        match opt {
            Opt::All => "all",
            Opt::Namespace(_) => "namespace",
            Opt::Ntests(_) => "ntests",
            Opt::Prof(_) => "prof",
            Opt::Ref(_) => "ref",
            Opt::Verbose => "verbose",
            Opt::Recipe => "recipe",
        }
        .to_string()
    }

    pub fn parse_options(argv: &Vec<String>, modes: &[&str], opt_list: &[&str]) -> Option<Options> {
        let mut opts = getopts::Options::new();

        opts.optflag("", "all", "");
        opts.optflag("", "verbose", "");
        opts.optflag("", "recipe", "");
        opts.optopt("", "module", "", "NAME");
        opts.optopt("", "namespace", "", "NAME");
        opts.optopt("", "ntests", "", "NUMBER");
        opts.optopt("", "prof", "", "VALUE");
        opts.optopt("", "ref", "", "VALUE");

        let mode_args = argv[2..]
            .iter()
            .filter(|mode| mode.chars().next().unwrap() != '-')
            .map(|string| string.as_str())
            .collect::<Vec<&str>>();

        for mode in &mode_args {
            if !modes.iter().any(|el| el == mode) {
                eprintln!("unknown mode {mode:?}");
                return None;
            }
        }

        let modes = mode_args
            .iter()
            .map(|mode| match *mode {
                "base" => Mode::Base,
                "build" => Mode::Build,
                "core" => Mode::Core,
                "clean" => Mode::Clean,
                "common" => Mode::Common,
                "crossref" => Mode::Crossref,
                "current" => Mode::Current,
                "debug" => Mode::Debug,
                "env" => Mode::Env,
                "init" => Mode::Init,
                "metrics" => Mode::Metrics,
                "mu" => Mode::Mu,
                "prelude" => Mode::Prelude,
                "profile" => Mode::Profile,
                "reference" => Mode::Reference,
                "release" => Mode::Release,
                "report" => Mode::Report,
                "view" => Mode::View,
                _ => panic!(),
            })
            .collect();

        let mut opt_args = argv[2..]
            .iter()
            .filter(|opt| opt.chars().next().unwrap() == '-')
            .collect::<Vec<&String>>();

        for opt in &mut opt_args {
            let expr = opt.clone().split_off(2);

            let base = match expr.find('=') {
                Some(index) => {
                    let mut clone = expr.clone();
                    clone.truncate(index);
                    clone
                }
                None => expr,
            };

            if !opt_list.iter().any(|el| el == &base) {
                eprintln!("unknown option {opt:?}");
                return None;
            }
        }

        let options = match opts.parse(opt_args) {
            Ok(opts) => opt_list
                .iter()
                .filter(|opt| opts.opt_present(opt))
                .map(|opt| match *opt {
                    "namespace" => Opt::Namespace(opts.opt_str("namespace").unwrap()),
                    "ntests" => Opt::Ntests(opts.opt_str("ntests").unwrap()),
                    "all" => Opt::All,
                    "verbose" => Opt::Verbose,
                    "recipe" => Opt::Recipe,
                    _ => panic!(),
                })
                .collect::<Vec<Opt>>(),
            Err(error) => {
                eprintln!("options: {error:?}");
                std::process::exit(-1);
            }
        };

        Some(Options { modes, options })
    }
}
