//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use {
    crate::{
        crates::Crate,
        parser::{Item, Parser},
    },
    public_api::{tokens::Token, PublicItem},
    std::{
        cell::RefCell,
        fs::File,
        io::{Error, ErrorKind, Write},
        result::Result,
    },
};

pub struct Symbols {
    pub symbols: RefCell<Vec<Symbol>>,
}

#[derive(Debug)]
pub struct Symbol {
    pub qualifiers: Option<Vec<Token>>,
    pub name: Option<Token>,
    pub item: Option<Item>,
}

impl Symbols {
    pub fn new(crate_: &Crate) -> Self {
        let symbol_table = Symbols {
            symbols: RefCell::new(Vec::<Symbol>::new()),
        };

        for item in crate_.symbols.items() {
            let item = Self::parse_item(item);

            match item {
                Err(e) => {
                    eprintln!("parse error: {e:?}")
                }
                Ok(item) => symbol_table.symbols.borrow_mut().push(item),
            }
        }

        symbol_table
    }

    fn parse_item(item: &PublicItem) -> Result<Symbol, Error> {
        <Symbol as Parser>::parse(item)
    }

    pub fn write(&self, path: &str) -> Result<(), Error> {
        let mut out = File::create(path)?;

        for symbol in (*self.symbols.borrow()).iter() {
            out.write_all(format!("{:?}\n", symbol).as_bytes())?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
