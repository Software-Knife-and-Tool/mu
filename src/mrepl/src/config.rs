//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
use {
    json::{self, JsonValue},
    std::fs,
    std::path::PathBuf,
};

#[derive(Debug)]
pub enum Config {
    Json(JsonValue),
    None,
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Array(Vec<Value>),
    Number(i64),
    Bool(bool),
    Null,
}

impl Config {
    // if there's an .mrepl in the current directory, use it.
    // otherwise, if there's one in the home directory, use it.
    pub fn new() -> Self {
        let mut home_path = PathBuf::new();

        home_path.push(std::env::home_dir().unwrap().as_path());
        home_path.push(".mrepl");

        let mut cwd_path = PathBuf::new();

        cwd_path.push(std::env::current_dir().unwrap().as_path());
        cwd_path.push(".mrepl");

        match fs::read_to_string(cwd_path) {
            Ok(json) => match json::parse(&json) {
                Ok(opts) => Config::Json(opts),
                Err(_) => Config::None,
            },
            Err(_) => match fs::read_to_string(home_path) {
                Ok(json) => match json::parse(&json) {
                    Ok(opts) => Config::Json(opts),
                    Err(_) => {
                        eprintln!("repl: failed to parse config JSON, using null config");
                        Config::None
                    }
                },
                Err(_) => Config::None,
            },
        }
    }

    fn map_value(value: &JsonValue) -> Value {
        match value {
            JsonValue::Short(str) => Value::String(str.as_str().to_string()),
            JsonValue::Object(obj) => Value::String(obj.dump()),
            JsonValue::Array(vec) => Value::Array(
                vec.iter()
                    .map(|value| Self::map_value(value))
                    .collect::<Vec<Value>>(),
            ),
            _ => panic!(),
        }
    }

    pub fn map(&self, key: &str) -> Option<Value> {
        match self {
            Config::Json(opts) => Some(Self::map_value(&opts[key])),
            Config::None => None,
        }
    }
}
