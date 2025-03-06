//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env config
use {
    crate::{
        core::{env::Env, exception, frame::Frame, types::Tag},
        types::{cons::Cons, fixnum::Fixnum, symbol::Symbol},
    },
    page_size,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub gcmode: GcMode,
    pub npages: usize,
    pub page_size: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GcMode {
    None,
    Auto,
    Demand,
}

impl Config {
    pub fn new(conf_option: Option<String>) -> Option<Config> {
        let mut config = Config {
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

    pub fn as_list(&self, env: &Env) -> Tag {
        let gcmode = if self.gcmode == GcMode::None {
            Symbol::keyword("none")
        } else if self.gcmode == GcMode::Auto {
            Symbol::keyword("auto")
        } else if self.gcmode == GcMode::Demand {
            Symbol::keyword("demand")
        } else {
            panic!()
        };

        Cons::list(
            env,
            &[
                Cons::new(Symbol::keyword("gcmode"), gcmode).evict(env),
                Cons::new(
                    Symbol::keyword("npages"),
                    Fixnum::with_or_panic(env.config.npages),
                )
                .evict(env),
                Cons::new(
                    Symbol::keyword("pagesz"),
                    Fixnum::with_or_panic(env.config.page_size),
                )
                .evict(env),
            ],
        )
    }
}

pub trait CoreFunction {
    fn mu_config(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn mu_config(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = env.config.as_list(env);

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
