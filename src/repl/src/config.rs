//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
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

impl Config {
    // if we have a .repl in the current directory, use it.
    // otherwise, see if there's one in the home directory
    pub fn new() -> Self {
        let mut home_path = PathBuf::new();

        home_path.push(std::env::home_dir().unwrap().as_path());
        home_path.push(".repl");

        let mut cwd_path = PathBuf::new();

        cwd_path.push(std::env::current_dir().unwrap().as_path());
        cwd_path.push(".repl");

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

    pub fn map(&self, key: &str) -> Option<String> {
        match self {
            Config::Json(opts) => match &opts[key] {
                JsonValue::Short(str) => Some(str.as_str().to_string()),
                JsonValue::Object(obj) => Some(obj.dump()),
                _ => None,
            },
            Config::None => None,
        }
    }
}
