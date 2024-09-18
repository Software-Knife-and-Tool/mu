//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use crate::VERSION;

#[derive(Debug)]
pub struct Options {
    pub options: Vec<Opt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Opt {
    Base,
    Config(String),
    Counts,
    Crossref,
    Current,
    Debug,
    Eval(String),
    Footprint,
    Load(String),
    Namespace(String),
    Ntests(u32),
    Output(String),
    Profile,
    Prof(String),
    Reference,
    Ref(String),
    Release,
    Verbose,
}

impl Options {
    pub fn version() {
        println!("{}", VERSION);
        std::process::exit(0)
    }

    pub fn option_name(opt: Opt) -> String {
        match opt {
            Opt::Base => "base",
            Opt::Config(_) => "config",
            Opt::Counts => "counts",
            Opt::Crossref => "crossref",
            Opt::Current => "current",
            Opt::Debug => "debug",
            Opt::Eval(_) => "eval",
            Opt::Footprint => "footprint",
            Opt::Load(_) => "load",
            Opt::Namespace(_) => "namespace",
            Opt::Ntests(_) => "ntests",
            Opt::Output(_) => "output",
            Opt::Prof(_) => "prof",
            Opt::Profile => "profile",
            Opt::Ref(_) => "ref",
            Opt::Reference => "reference",
            Opt::Release => "release",
            Opt::Verbose => "verbose",
        }
        .to_string()
    }

    pub fn parse(argv: &[String]) -> Option<Options> {
        let mut opts = getopts::Options::new();
        let mut options = Vec::new();

        opts.optflag("", "base", "");
        opts.optflag("", "counts", "");
        opts.optflag("", "crossref", "");
        opts.optflag("", "current", "");
        opts.optflag("", "debug", "");
        opts.optflag("", "footprint", "");
        opts.optflag("", "profile", "");
        opts.optflag("", "reference", "");
        opts.optflag("", "release", "");
        opts.optflag("", "verbose", "");

        opts.optopt("", "config", "", "VALUE");
        opts.optopt("", "eval", "", "VALUE");
        opts.optopt("", "load", "", "VALUE");
        opts.optopt("", "namespace", "", "VALUE");
        opts.optopt("", "ntests", "", "VALUE");
        opts.optopt("", "output", "", "VALUE");
        opts.optopt("", "prof", "", "VALUE");
        opts.optopt("", "ref", "", "VALUE");

        let opt_names = vec![
            "base",
            "config",
            "counts",
            "crossref",
            "current",
            "eval",
            "footprint",
            "load",
            "namespace",
            "output",
            "prof",
            "profile",
            "ref",
            "reference",
            "release",
            "verbose",
        ];

        let opts = match opts.parse(&argv[2..]) {
            Ok(opts) => opts,
            Err(error) => {
                eprintln!("mux options: {error:?}");
                std::process::exit(-1);
            }
        };

        for name in opt_names {
            if opts.opt_present(name) {
                match opts.opt_get::<String>(name) {
                    Ok(_) => match name {
                        "base" => options.push(Opt::Base),
                        "config" => options.push(Opt::Config(opts.opt_str(name).unwrap())),
                        "counts" => options.push(Opt::Counts),
                        "crossref" => options.push(Opt::Crossref),
                        "current" => options.push(Opt::Current),
                        "debug" => options.push(Opt::Debug),
                        "eval" => options.push(Opt::Eval(opts.opt_str(name).unwrap())),
                        "footprint" => options.push(Opt::Footprint),
                        "load" => options.push(Opt::Load(opts.opt_str(name).unwrap())),
                        "namespace" => options.push(Opt::Namespace(opts.opt_str(name).unwrap())),
                        "ntests" => {
                            options.push(Opt::Ntests(opts.opt_str(name).unwrap().parse().unwrap()))
                        }
                        "output" => options.push(Opt::Output(opts.opt_str(name).unwrap())),
                        "prof" => options.push(Opt::Prof(opts.opt_str(name).unwrap())),
                        "profile" => options.push(Opt::Profile),
                        "ref" => options.push(Opt::Ref(opts.opt_str(name).unwrap())),
                        "reference" => options.push(Opt::Reference),
                        "release" => options.push(Opt::Release),
                        "verbose" => options.push(Opt::Verbose),
                        _ => panic!(),
                    },
                    Err(_) => panic!(),
                }
            }
        }

        Some(Options { options })
    }
}
