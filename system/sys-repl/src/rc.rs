//  SPDX-FileCopyrightText: Copyright 2026 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use {
    lite_json::{json::JsonValue, json_parser, Serialize},
    mu::{Config, Mu},
    std::str,
};

#[derive(Debug, Clone)]
pub struct Rc {
    pub config: Config,
    pub load: Option<Vec<String>>,
    pub loader: Option<String>,
    pub options: Option<Vec<String>>,
    pub reader: Option<String>,
    pub require: Option<Vec<String>>,
    pub sys: Option<Vec<String>>,
}

impl Default for Rc {
    fn default() -> Rc {
        Rc {
            config: Config::default(),
            load: None,
            loader: None,
            options: None,
            reader: None,
            require: None,
            sys: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RcBuilder {
    pub json: JsonValue,
    pub rc: Rc,
}

impl RcBuilder {
    pub fn new(json: &str) -> Self {
        let json = json_parser::parse_json(json).expect("rc: JSON parse failure");

        Self {
            json,
            rc: Rc::default(),
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

        self.rc.config = match config {
            Some(JsonValue::Object(_)) => {
                let json = String::from_utf8(config.unwrap().serialize());

                Mu::config(Some(json.unwrap()))
            }
            None => Mu::config(None),
            _ => panic!("config: config string format"),
        };

        self
    }

    fn loader(&mut self) -> &mut Self {
        let arg = Self::map_json("loader", &self.json);

        self.rc.loader = match arg {
            Some(JsonValue::String(loader)) => Some(loader.iter().collect::<String>()),
            None => None,
            _ => panic!("rc: {arg:?} loader format"),
        };

        self
    }

    fn load(&mut self) -> &mut Self {
        let arg = Self::map_json("load", &self.json);

        self.rc.load = match arg {
            Some(JsonValue::Array(vec)) => Some(
                vec.iter()
                    .map(|s| match s {
                        JsonValue::String(vec) => vec.iter().collect::<String>(),
                        _ => panic!("rc: {s:?} load format"),
                    })
                    .collect::<Vec<String>>(),
            ),
            None => None,
            _ => panic!("rc: {arg:?} load format"),
        };

        self
    }

    fn options(&mut self) -> &mut Self {
        let arg = Self::map_json("options", &self.json);

        self.rc.load = match arg {
            Some(JsonValue::Array(vec)) => Some(
                vec.iter()
                    .map(|s| match s {
                        JsonValue::String(vec) => vec.iter().collect::<String>(),
                        _ => panic!("rc: {s:?} options format"),
                    })
                    .collect::<Vec<String>>(),
            ),
            None => None,
            _ => panic!("rc: {arg:?} options format"),
        };

        self
    }

    fn reader(&mut self) -> &mut Self {
        let arg = Self::map_json("reader", &self.json);

        self.rc.reader = match arg {
            Some(JsonValue::String(reader)) => Some(reader.iter().collect::<String>()),
            None => None,
            _ => panic!("rc: {arg:?} reader format"),
        };

        self
    }

    fn require(&mut self) -> &mut Self {
        let arg = Self::map_json("require", &self.json);

        self.rc.require = match arg {
            Some(JsonValue::Array(vec)) => Some(
                vec.iter()
                    .map(|s| match s {
                        JsonValue::String(vec) => vec.iter().collect::<String>(),
                        _ => panic!("rc: {s:?} require format"),
                    })
                    .collect::<Vec<String>>(),
            ),
            None => None,
            _ => panic!("rc: {arg:?} require format"),
        };

        self
    }

    fn sys(&mut self) -> &mut Self {
        let arg = Self::map_json("sys", &self.json);

        self.rc.sys = match arg {
            Some(JsonValue::Array(vec)) => Some(
                vec.iter()
                    .map(|s| match s {
                        JsonValue::String(vec) => vec.iter().collect::<String>(),
                        _ => panic!("rc: {s:?} sys format"),
                    })
                    .collect::<Vec<String>>(),
            ),
            None => None,
            _ => panic!("rc: {arg:?} sys format"),
        };

        self
    }

    fn build(&self) -> Rc {
        self.rc.clone()
    }
}

impl Rc {
    pub fn new(rc: Option<String>) -> Self {
        match rc {
            None => Rc::default(),
            Some(json) => RcBuilder::new(&json)
                .config()
                .load()
                .loader()
                .options()
                .reader()
                .require()
                .sys()
                .build(),
        }
    }
}
