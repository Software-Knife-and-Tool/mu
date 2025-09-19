//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

/// Config struct
use {
    crate::{
        core_::{env::Env, tag::Tag},
        types::{cons::Cons, fixnum::Fixnum, vector::Vector},
    },
    lite_json::{json::JsonValue, json_parser},
};

#[derive(Debug, Clone)]
pub struct Config {
    pub gc_mode: GcMode,
    pub npages: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            npages: 1024,
            gc_mode: GcMode::None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GcMode {
    None,
    Auto,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigBuilder {
    pub json: JsonValue,
    pub gc_mode: Option<GcMode>,
    pub npages: Option<usize>,
}

impl ConfigBuilder {
    pub fn new(conf: &str) -> Self {
        let json = json_parser::parse_json(conf).expect("config: JSON parse failure");

        Self {
            json,
            gc_mode: None,
            npages: None,
        }
    }

    pub fn map_json(term: &str, json: &JsonValue) -> Option<JsonValue> {
        match json {
            JsonValue::Object(object) => object
                .iter()
                .find(|(cvec, _)| {
                    let name = &*(cvec.iter().collect::<String>());
                    term == name
                })
                .map(|(_, value)| value.clone()),
            _ => None,
        }
    }

    pub fn gc_mode(&mut self) -> &mut Self {
        let mode = Self::map_json("gc-mode", &self.json);

        self.gc_mode = match mode {
            Some(JsonValue::String(mode)) => match &*(mode.iter().collect::<String>()) {
                "auto" => Some(GcMode::Auto),
                "none" => Some(GcMode::None),
                _ => panic!("gc-mode: config string format"),
            },
            Some(_) => panic!("gc-mode: config string format"),
            None => None,
        };

        self
    }

    pub fn npages(&mut self) -> &mut Self {
        let npages = Self::map_json("pages", &self.json);

        self.npages = match npages.unwrap() {
            JsonValue::Number(n) => Some(n.integer as usize),
            JsonValue::String(nstr) => Some(
                (*(nstr.iter().collect::<String>()))
                    .parse::<usize>()
                    .unwrap(),
            ),
            _ => panic!("pages: config string format"),
        };

        self
    }

    pub fn build(&self) -> Option<Config> {
        let mut config: Config = Default::default();

        if self.npages.is_some() {
            config.npages = self.npages.unwrap()
        }

        if self.gc_mode.is_some() {
            config.gc_mode = self.gc_mode.unwrap()
        }

        Some(config)
    }
}

impl Config {
    pub fn new(conf_option: Option<String>) -> Option<Config> {
        match conf_option {
            None => Some(Default::default()),
            Some(conf) => ConfigBuilder::new(&conf).gc_mode().npages().build(),
        }
    }

    pub fn as_list(&self, env: &Env) -> Tag {
        let gc_mode = match self.gc_mode {
            GcMode::None => "none",
            GcMode::Auto => "auto",
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
                    Vector::from("npages").evict(env),
                    Fixnum::with_or_panic(env.config.npages),
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
