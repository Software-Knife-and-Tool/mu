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
    pub namespace: RefCell<Option<String>>,
    pub bindmap_path: RefCell<Option<String>>,
    pub output_path: RefCell<Option<String>>,
    pub verbose: RefCell<bool>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            namespace: RefCell::new(None),
            bindmap_path: RefCell::new(None),
            output_path: RefCell::new(None),
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

    pub fn verbose(&self) -> bool {
        *self.verbose.borrow()
    }
}

pub struct BindGen<'a> {
    bindmap: Option<JsonValue>,
    config: &'a Config,
    syntax: syn::File,
}

impl BindGen<'_> {
    fn parse(path: &str) -> std::result::Result<syn::File, Error> {
        let source = fs::read(path)?;

        match syn::parse_file(&String::from_utf8(source).unwrap()) {
            Ok(syntax) => Ok(syntax),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    pub fn with_config<'a>(
        config: &'a Config,
        src_path: &str,
    ) -> std::result::Result<BindGen<'a>, Error> {
        let bindmap = match config.bindmap_path() {
            Some(path) => {
                let bindmap = fs::read(&path)?;

                match json::parse(&String::from_utf8(bindmap).unwrap()) {
                    Ok(bindmap) => Some(bindmap),
                    Err(_) => return Err(Error::new(ErrorKind::Other, path.to_string())),
                }
            }
            None => None,
        };

        let syntax = Self::parse(src_path)?;

        Ok(BindGen {
            config,
            bindmap,
            syntax,
        })
    }

    pub fn print_items(&self, path: &str) {
        println!("{path}/");
        for item in &self.syntax.items {
            self.print_item(item, 1)
        }
    }

    pub fn write(&self, path: &str) -> std::result::Result<(), Error> {
        let mut out = OpenOptions::new().create(true).append(true).open(path)?;

        out.write_all(format!("{:#?}", self.syntax).as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
