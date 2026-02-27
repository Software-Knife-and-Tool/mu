//  SPDX-FileCopyrightText: Copyright 2026 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use {
    lite_json::{json::JsonValue, json_parser, Serialize},
    mu::{Config as MuConfig, Mu},
    std::str,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub config: MuConfig,
    pub ns: Option<String>,
    pub load: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            config: MuConfig::default(),
            ns: None,
            load: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    pub json: JsonValue,
    pub config: Config,
}

impl ConfigBuilder {
    pub fn new(conf: &str) -> Self {
        let json = json_parser::parse_json(conf).expect("config: JSON parse failure");

        Self {
            json,
            config: Config::default(),
        }
    }

    fn map_json(term: &str, json: &JsonValue) -> Option<JsonValue> {
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

    fn config(&mut self) -> &mut Self {
        let config = Self::map_json("config", &self.json);

        self.config.config = match config {
            Some(JsonValue::Object(_)) => {
                let json = String::from_utf8(config.unwrap().serialize());

                Mu::config(Some(json.unwrap()))
            }
            None => Mu::config(None),
            _ => panic!("config: config string format"),
        };

        self
    }

    fn ns(&mut self) -> &mut Self {
        let ns = Self::map_json("namespace", &self.json);

        self.config.ns = match ns {
            Some(JsonValue::String(ns)) => Some(ns.iter().collect::<String>()),
            None => None,
            _ => panic!("ns: config string format"),
        };

        self
    }

    fn load(&mut self) -> &mut Self {
        let load = Self::map_json("load", &self.json);

        self.config.load = match load {
            Some(JsonValue::String(sys)) => Some(vec![sys.iter().collect::<String>()]),
            Some(JsonValue::Array(vec)) => Some(
                vec.iter()
                    .map(|s| match s {
                        JsonValue::String(vec) => vec.iter().collect::<String>(),
                        _ => panic!("load: config string format"),
                    })
                    .collect::<Vec<String>>(),
            ),
            None => None,
            _ => panic!("load: config string format"),
        };

        self
    }

    fn build(&self) -> Config {
        Config {
            config: self.config.config.clone(),
            ns: self.config.ns.clone(),
            load: self.config.load.clone(),
        }
    }
}

impl Config {
    pub fn new(conf_option: Option<String>) -> Self {
        match conf_option {
            None => Config::default(),
            Some(conf) => ConfigBuilder::new(&conf).config().ns().load().build(),
        }
    }
}
