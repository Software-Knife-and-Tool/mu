//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#[allow(unused_imports)]
use {
    crate::{
        image::{heap_info::HeapInfoBuilder, reader::Reader, writer::Writer},
        options::{Mode, Opt, Options},
    },
    mu::{Condition, Env, Mu, Result, Tag},
    object::{Object, ObjectSection},
    std::{error::Error, fs},
};

pub struct Image {}

impl Image {
    fn dump(path: &str) {
        let reader = Reader::with_reader(path).unwrap();

        println!("{path:}/");
        println!("  sections/");

        let section_names = reader
            .section_names()
            .into_iter()
            .filter(|name| !name.is_empty())
            .collect::<Vec<String>>();

        for name in &section_names {
            println!("    {name}")
        }

        println!("  symbols/");

        let symbols = reader.symbol_names();

        while let Some(name) = &symbols {
            println!("    {name:?}")
        }

        println!("  .allocator/");
        match reader.section_by_name(".allocator") {
            Some(section) => {
                let json = String::from_utf8(reader.section_data(section).unwrap()).unwrap();

                println!("   json: {json:?}");
            }
            None => println!("     ! .allocator section not found"),
        }

        println!("  .image/");
        match reader.section_by_name(".image") {
            Some(section) => println!("    data: {:?}", reader.section_data(section).unwrap()),
            None => println!("     ! .image section not found"),
        }
    }

    pub fn image(argv: &Vec<String>, home: &str) {
        match Options::parse_options(
            argv,
            &["build", "init", "view"],
            &["eval", "image", "config", "load", "out"],
        ) {
            None => (),
            Some(options) => {
                if options.modes.len() != 1 {
                    eprintln!("illegal options: {argv:?}");
                    std::process::exit(-1)
                }

                let mode = &options.modes[0];

                match Options::find_opt(&options, &Opt::Verbose) {
                    Some(_) => println!("mux image: {:?}", mode),
                    None => (),
                };

                match options.modes[0] {
                    Mode::Build => Self::build(&options, home),
                    Mode::View => Self::view(&options, home),
                    _ => panic!(),
                }
                .unwrap()
            }
        }
    }

    fn build(options: &Options, _home: &str) -> std::result::Result<(), Box<dyn Error>> {
        let mut _config: Option<String> = None;
        let out_opt = options.options.iter().find(|opt| match opt {
            Opt::Out(_) => true,
            _ => false,
        });

        let config_opt = options.options.iter().find(|opt| match opt {
            Opt::Config(_) => true,
            _ => false,
        });

        match out_opt {
            Some(opt) => match opt {
                Opt::Out(path) => {
                    let config = match config_opt {
                        Some(opt) => match opt {
                            Opt::Config(cfg) => Some(cfg.clone()),
                            _ => None,
                        },
                        None => None,
                    };

                    let env = match Mu::config(config) {
                        Some(config) => Mu::make_env(&config),
                        None => {
                            eprintln!("build: configuration problem");
                            std::process::exit(-1)
                        }
                    };

                    for opt in options.options.iter() {
                        match opt {
                            Opt::Load(path) => match Mu::load(&env, &path) {
                                Ok(_) => (),
                                Err(e) => {
                                    eprintln!(
                                        "build load: failed to load {}, {}",
                                        &path,
                                        Mu::exception_string(&env, e)
                                    );
                                    std::process::exit(-1);
                                }
                            },
                            Opt::Eval(expr) => match Mu::eval_str(&env, &expr) {
                                Ok(_) => (),
                                Err(e) => {
                                    eprintln!(
                                        "build eval: error {}, {}",
                                        expr,
                                        Mu::exception_string(&env, e)
                                    );
                                    std::process::exit(-1);
                                }
                            },
                            _ => (),
                        }
                    }

                    let heap_info = HeapInfoBuilder::new()
                        .config("config".to_string())
                        .image(vec![])
                        .meta(vec![])
                        .build();

                    let writer = Writer::with_writer(&path, heap_info).unwrap();

                    writer.write().unwrap()
                }
                _ => panic!(),
            },
            None => {
                eprintln!("image build requires --out");
                std::process::exit(-1)
            }
        }

        Ok(())
    }

    fn view(options: &Options, _home: &str) -> std::result::Result<(), Box<dyn Error>> {
        let image_opt = options.options.iter().find(|opt| match opt {
            Opt::Image(_) => true,
            _ => false,
        });

        match image_opt {
            Some(opt) => match opt {
                Opt::Image(path) => Self::dump(path),
                _ => panic!(),
            },
            None => {
                eprintln!("image view requires --image");
                std::process::exit(-1)
            }
        }

        Ok(())
    }
}
