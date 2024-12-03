//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
use crate::VERSION;

#[derive(Debug, PartialEq, Clone)]
pub struct Options {
    pub modes: Vec<Mode>,
    pub options: Vec<Opt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Opt {
    Config(String),
    Module(String),
    Namespace(String),
    Ntests(String),
    Prof(String),
    Ref(String),
    Verbose,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Mode {
    Base,
    Core,
    Crossref,
    Current,
    Debug,
    Footprint,
    Metrics,
    Mu,
    Prelude,
    Profile,
    Reference,
    Release,
}

impl Options {
    pub fn version() {
        println!("{}", VERSION);
        std::process::exit(0)
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
                Opt::Config(str)
                | Opt::Module(str)
                | Opt::Namespace(str)
                | Opt::Prof(str)
                | Opt::Ref(str)
                | Opt::Ntests(str) => Some(str.to_string()),
                _ => panic!(),
            },
            None => None,
        }
    }

    pub fn opt_name(opt: Opt) -> String {
        match opt {
            Opt::Config(_) => "config",
            Opt::Module(_) => "module",
            Opt::Namespace(_) => "namespace",
            Opt::Ntests(_) => "ntests",
            Opt::Prof(_) => "prof",
            Opt::Ref(_) => "ref",
            Opt::Verbose => "verbose",
        }
        .to_string()
    }

    pub fn parse_options(
        argv: &Vec<String>,
        mode_list: &[&str],
        opt_list: &[&str],
    ) -> Option<Options> {
        let mut opts = getopts::Options::new();

        opts.optflag("", "verbose", "");
        opts.optopt("", "config", "", "VALUE");
        opts.optopt("", "module", "", "VALUE");
        opts.optopt("", "namespace", "", "VALUE");
        opts.optopt("", "ntests", "", "VALUE");
        opts.optopt("", "prof", "", "VALUE");
        opts.optopt("", "ref", "", "VALUE");

        let mode_args = argv[2..]
            .iter()
            .filter(|mode| mode.chars().next().unwrap() != '-')
            .map(|string| string.as_str())
            .collect::<Vec<&str>>();

        for mode in &mode_args {
            if !mode_list.iter().any(|el| el == mode) {
                eprintln!("mux: unknown mode {mode:?}");
                return None;
            }
        }

        let modes = mode_args
            .iter()
            .map(|mode| match *mode {
                "base" => Mode::Base,
                "core" => Mode::Core,
                "crossref" => Mode::Crossref,
                "current" => Mode::Current,
                "debug" => Mode::Debug,
                "footprint" => Mode::Footprint,
                "metrics" => Mode::Metrics,
                "mu" => Mode::Mu,
                "prelude" => Mode::Prelude,
                "profile" => Mode::Profile,
                "reference" => Mode::Reference,
                "release" => Mode::Release,
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
                eprintln!("mux: unknown option {opt:?}");
                return None;
            }
        }

        let options = match opts.parse(opt_args) {
            Ok(opts) => opt_list
                .iter()
                .filter(|opt| opts.opt_present(opt))
                .map(|opt| match *opt {
                    "verbose" => Opt::Verbose,
                    "config" => Opt::Config(opts.opt_str("config").unwrap()),
                    "module" => Opt::Module(opts.opt_str("module").unwrap()),
                    "namespace" => Opt::Namespace(opts.opt_str("namespace").unwrap()),
                    "ntests" => Opt::Ntests(opts.opt_str("ntests").unwrap()),
                    "prof" => Opt::Prof(opts.opt_str("prof").unwrap()),
                    "ref" => Opt::Ref(opts.opt_str("ref").unwrap()),
                    _ => panic!(),
                })
                .collect::<Vec<Opt>>(),
            Err(error) => {
                eprintln!("mux options: {error:?}");
                std::process::exit(-1);
            }
        };

        Some(Options { modes, options })
    }
}
