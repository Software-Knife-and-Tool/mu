#![allow(dead_code)]
#[allow(unused_imports)]
use {
    crate::syntax::Syntax,
    json::{self, object::Object, JsonValue},
    std::cell::RefCell,
    std::{
        env,
        fs::{self, File, OpenOptions},
        io::{self, Error, ErrorKind, Read, Result, Write},
    },
    syn::{self, Item},
};

pub struct Config {
    pub bindmap_path: RefCell<Option<String>>,
    pub namespace: RefCell<Option<String>>,
    pub output_path: RefCell<Option<String>>,
    pub symbols_path: RefCell<Option<String>>,
    pub verbose: RefCell<bool>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            bindmap_path: RefCell::new(None),
            namespace: RefCell::new(None),
            output_path: RefCell::new(None),
            symbols_path: RefCell::new(None),
            verbose: RefCell::new(false),
        }
    }

    pub fn namespace(&self) -> Option<String> {
        self.namespace.borrow().clone()
    }

    pub fn bindmap_path(&self) -> Option<String> {
        self.bindmap_path.borrow().clone()
    }

    pub fn output_path(&self) -> Option<String> {
        self.output_path.borrow().clone()
    }

    pub fn symbols_path(&self) -> Option<String> {
        self.symbols_path.borrow().clone()
    }

    pub fn verbose(&self) -> bool {
        *self.verbose.borrow()
    }
}

#[cfg(test)]
mod tests {}
