//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env config
use crate::{
    core::{
        env::Env,
        frame::Frame,
        exception,
        core::CORE
    },
    types::{
        cons::Cons,
        fixnum::Fixnum,
        vector::Vector,
    },
};
use page_size;

#[derive(Debug, Clone)]
pub struct Config {
    pub gcmode: GcMode,
    pub npages: usize,
    pub page_size: usize,
    pub version: String,
}

#[derive(Debug, Copy, Clone)]
pub enum GcMode {
    None,
    Auto,
    Demand,
}

impl Config {
    pub fn new(conf_option: Option<String>) -> Option<Config> {
        let mut config = Config {
            version: CORE.version.to_string(),
            npages: 1024,
            gcmode: GcMode::Auto,
            page_size: page_size::get(),
        };

        match conf_option {
            None => Some(config),
            Some(conf) => {
                for phrase in conf.split(',').collect::<Vec<&str>>() {
                    let parse = phrase.split(':').collect::<Vec<&str>>();
                    if parse.len() != 2 {
                        return None;
                    } else {
                        let [name, arg] = parse[..] else { panic!() };
                        match name {
                            "npages" => match arg.parse::<usize>() {
                                Ok(n) => config.npages = n,
                                Err(_) => return None,
                            },
                            "page_size" => match arg.parse::<usize>() {
                                Ok(n) => config.page_size = n,
                                Err(_) => return None,
                            },
                            "gcmode" => {
                                config.gcmode = match arg {
                                    "auto" => GcMode::Auto,
                                    "none" => GcMode::None,
                                    "demand" => GcMode::Demand,
                                    _ => return None,
                                }
                            }
                            _ => return None,
                        }
                    }
                }

                Some(config)
            }
        }
    }
}

pub trait CoreFunction {
    fn mu_config(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn mu_config(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let mut alist = vec![];

        alist.push(Cons::cons(env,
                              Vector::from("gcmode").evict(env),
                              match env.config.gcmode {
                                  GcMode::None => Vector::from("none").evict(env),
                                  GcMode::Auto => Vector::from("auto").evict(env),
                                  GcMode::Demand => Vector::from("demand").evict(env),
                              }));

        alist.push(Cons::cons(env, Vector::from("npages").evict(env), Fixnum::with_or_panic(env.config.npages)));
        alist.push(Cons::cons(env, Vector::from("page_size").evict(env), Fixnum::with_or_panic(env.config.page_size)));
        alist.push(Cons::cons(env, Vector::from("version").evict(env), Vector::from(env.config.version.as_str()).evict(env)));

        fp.value = Cons::list(env, &alist);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
    }
}
