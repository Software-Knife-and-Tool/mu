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
    Eval(String),
    Image(String),
    Load(String),
    Module(String),
    Namespace(String),
    Ntests(String),
    Out(String),
    Prof(String),
    Ref(String),
    Verbose,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Mode {
    Base,
    Build,
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
    View,
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
                | Opt::Image(str)
                | Opt::Load(str)
                | Opt::Out(str)
                | Opt::Eval(str)
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
            Opt::Eval(_) => "exec",
            Opt::Image(_) => "image",
            Opt::Load(_) => "load",
            Opt::Module(_) => "module",
            Opt::Namespace(_) => "namespace",
            Opt::Ntests(_) => "ntests",
            Opt::Out(_) => "out",
            Opt::Prof(_) => "prof",
            Opt::Ref(_) => "ref",
            Opt::Verbose => "verbose",
        }
        .to_string()
    }

    pub fn parse_options(argv: &Vec<String>, modes: &[&str], opt_list: &[&str]) -> Option<Options> {
        let mut opts = getopts::Options::new();

        opts.optflag("", "verbose", "");
        opts.optopt("", "config", "", "VALUE");
        opts.optopt("", "eval", "", "VALUE");
        opts.optopt("", "image", "", "VALUE");
        opts.optopt("", "load", "", "VALUE");
        opts.optopt("", "module", "", "VALUE");
        opts.optopt("", "namespace", "", "VALUE");
        opts.optopt("", "ntests", "", "VALUE");
        opts.optopt("", "out", "", "VALUE");
        opts.optopt("", "prof", "", "VALUE");
        opts.optopt("", "ref", "", "VALUE");

        let mode_args = argv[2..]
            .iter()
            .filter(|mode| mode.chars().next().unwrap() != '-')
            .map(|string| string.as_str())
            .collect::<Vec<&str>>();

        for mode in &mode_args {
            if !modes.iter().any(|el| el == mode) {
                eprintln!("mux: unknown mode {mode:?}");
                return None;
            }
        }

        let modes = mode_args
            .iter()
            .map(|mode| match *mode {
                "base" => Mode::Base,
                "build" => Mode::Build,
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
                eprintln!("mux: unknown option {opt:?}");
                return None;
            }
        }

        let options = match opts.parse(opt_args) {
            Ok(opts) => opt_list
                .iter()
                .filter(|opt| opts.opt_present(opt))
                .map(|opt| match *opt {
                    "config" => Opt::Config(opts.opt_str("config").unwrap()),
                    "eval" => Opt::Eval(opts.opt_str("eval").unwrap()),
                    "image" => Opt::Image(opts.opt_str("image").unwrap()),
                    "load" => Opt::Load(opts.opt_str("load").unwrap()),
                    "module" => Opt::Module(opts.opt_str("module").unwrap()),
                    "namespace" => Opt::Namespace(opts.opt_str("namespace").unwrap()),
                    "ntests" => Opt::Ntests(opts.opt_str("ntests").unwrap()),
                    "out" => Opt::Out(opts.opt_str("out").unwrap()),
                    "prof" => Opt::Prof(opts.opt_str("prof").unwrap()),
                    "ref" => Opt::Ref(opts.opt_str("ref").unwrap()),
                    "verbose" => Opt::Verbose,
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
