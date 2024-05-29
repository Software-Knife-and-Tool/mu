//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env config
use crate::{
    allocators::bump_allocator::BumpAllocator,
    core::{gc::GcMode, types::Tag},
};

// config builder
pub struct ConfigBuilder {
    pub npages: Option<usize>,
    pub gcmode: Option<GcMode>,
    pub image: Option<Vec<u8>>,
    pub heap: Option<BumpAllocator>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            npages: Some(1024),
            gcmode: Some(GcMode::Auto),
            image: None,
            heap: None,
        }
    }

    pub fn npages(&mut self, n: usize) -> &mut Self {
        self.npages = Some(n);
        self
    }

    pub fn gcmode(&mut self, mode: GcMode) -> &mut Self {
        self.gcmode = Some(mode);
        self
    }

    pub fn image(&mut self, image: Vec<u8>) -> &mut Self {
        self.image = Some(image);
        self
    }

    pub fn build(&self) -> Option<Config> {
        Some(Config {
            npages: 1024,
            gcmode: GcMode::Auto,
            heap: Some(BumpAllocator::new(self.npages.unwrap(), Tag::NTYPES)),
        })
    }
}

#[derive(Debug)]
pub struct Config {
    pub npages: usize,
    pub gcmode: GcMode,
    pub heap: Option<BumpAllocator>,
}

impl Config {
    pub fn new(conf_option: Option<String>) -> Option<Config> {
        let mut config = Config {
            npages: 1024,
            gcmode: GcMode::Auto,
            heap: None,
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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
    }
}
