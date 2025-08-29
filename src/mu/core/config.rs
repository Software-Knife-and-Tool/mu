//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

/// Config struct
use {
    crate::{
        core::{env::Env, tag::Tag},
        types::{cons::Cons, fixnum::Fixnum, vector::Vector},
    },
    page_size,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub gc_mode: GcMode,
    pub npages: usize,
    pub page_size: usize,
    pub heap_type: HeapType,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            npages: 1024,
            gc_mode: GcMode::None,
            heap_type: HeapType::Bump,
            page_size: page_size::get(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GcMode {
    None,
    Auto,
    Demand,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HeapType {
    Bump,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ConfigBuilder {
    pub gc_mode: Option<GcMode>,
    pub heap_type: Option<HeapType>,
    pub npages: Option<usize>,
    pub page_size: Option<usize>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            gc_mode: None,
            heap_type: None,
            npages: None,
            page_size: None,
        }
    }

    pub fn gc_mode(&mut self, phrases: &mut Vec<(&str, &str)>) -> &mut Self {
        if let Some((index, (_, mode))) = phrases
            .iter_mut()
            .enumerate()
            .find(|(_, (key, _))| key == &"gc-mode")
        {
            self.gc_mode = Some(match *mode {
                "auto" => GcMode::Auto,
                "none" => GcMode::None,
                "demand" => GcMode::Demand,
                _ => panic!(),
            });
            phrases.swap_remove(index);
        }

        self
    }

    pub fn heap_type(&mut self, phrases: &mut Vec<(&str, &str)>) -> &mut Self {
        if let Some((index, (_, mode))) = phrases
            .iter_mut()
            .enumerate()
            .find(|(_, (key, _))| key == &"heap-type")
        {
            self.heap_type = Some(match *mode {
                "bump" => HeapType::Bump,
                _ => panic!(),
            });
            phrases.swap_remove(index);
        }

        self
    }

    pub fn npages(&mut self, phrases: &mut Vec<(&str, &str)>) -> &mut Self {
        if let Some((index, (_, n))) = phrases
            .iter_mut()
            .enumerate()
            .find(|(_, (key, _))| key == &"npages")
        {
            self.npages = Some(n.parse::<usize>().unwrap());
            phrases.swap_remove(index);
        }

        self
    }

    pub fn page_size(&mut self, phrases: &mut Vec<(&str, &str)>) -> &mut Self {
        if let Some((index, (_, n))) = phrases
            .iter_mut()
            .enumerate()
            .find(|(_, (key, _))| key == &"page-size")
        {
            self.page_size = Some(n.parse::<usize>().unwrap());
            phrases.swap_remove(index);
        }

        self
    }

    pub fn build(&self, phrases: Vec<(&str, &str)>) -> Option<Config> {
        assert!(phrases.is_empty());

        let default: Config = Default::default();
        let config = Config {
            npages: if self.npages.is_some() {
                self.npages.unwrap()
            } else {
                default.npages
            },
            gc_mode: if self.gc_mode.is_some() {
                self.gc_mode.unwrap()
            } else {
                default.gc_mode
            },
            heap_type: if self.heap_type.is_some() {
                self.heap_type.unwrap()
            } else {
                default.heap_type
            },
            page_size: if self.page_size.is_some() {
                self.page_size.unwrap()
            } else {
                default.page_size
            },
        };

        Some(config)
    }
}

impl Config {
    pub fn new(conf_option: Option<String>) -> Option<Config> {
        match conf_option {
            None => Some(Default::default()),
            Some(conf) => {
                let terms = conf.split(',').collect::<Vec<&str>>();
                let mut phrases = terms
                    .into_iter()
                    .map(|term| {
                        let term = term.split(':').collect::<Vec<&str>>();
                        assert!(term.len() == 2);

                        (term[0].trim(), term[1].trim())
                    })
                    .collect::<Vec<(&str, &str)>>();

                let config = ConfigBuilder::new()
                    .gc_mode(&mut phrases)
                    .heap_type(&mut phrases)
                    .npages(&mut phrases)
                    .page_size(&mut phrases)
                    .build(phrases);

                config
            }
        }
    }

    pub fn as_list(&self, env: &Env) -> Tag {
        let gc_mode = match self.gc_mode {
            GcMode::None => "none",
            GcMode::Auto => "auto",
            GcMode::Demand => "demand",
        };

        let heap_type = match self.heap_type {
            HeapType::Bump => "bump",
        };

        Cons::list(
            env,
            &[
                Cons::new(
                    Vector::from("gc-mode").evict(env),
                    Vector::from(gc_mode).evict(env),
                )
                .evict(env),
                Cons::new(
                    Vector::from("heap-type").evict(env),
                    Vector::from(heap_type).evict(env),
                )
                .evict(env),
                Cons::new(
                    Vector::from("npages").evict(env),
                    Fixnum::with_or_panic(env.config.npages),
                )
                .evict(env),
                Cons::new(
                    Vector::from("page-size").evict(env),
                    Fixnum::with_or_panic(env.config.page_size),
                )
                .evict(env),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
