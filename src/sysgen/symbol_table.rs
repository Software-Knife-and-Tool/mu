//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

#![allow(dead_code)]
use {
    crate::crate_::Crate,
    public_api::{tokens::Token, PublicItem},
    std::{
        cell::RefCell,
        fs::File,
        io::{Error, Write},
        result::Result,
    },
};

pub struct SymbolTable {
    pub symbols: RefCell<Vec<SymbolDescription>>,
}

#[derive(Debug)]
pub struct SymbolDescription {
    pub type_: String,
    pub name: String,
    pub value: String,
    pub qualifiers: String,
}

impl SymbolTable {
    pub fn new(crate_: &Crate) -> Self {
        let symbol_table = SymbolTable {
            symbols: RefCell::new(Vec::<SymbolDescription>::new()),
        };

        for item in crate_.symbols.items() {
            symbol_table
                .symbols
                .borrow_mut()
                .push(Self::parse_item(item))
        }

        symbol_table
    }

    fn parse_item(item: &PublicItem) -> SymbolDescription {
        let mut symbol = SymbolDescription {
            type_: "".to_string(),
            name: "".to_string(),
            value: "".to_string(),
            qualifiers: "".to_string(),
        };

        for qualifier in item.tokens() {
            if let Token::Qualifier(qualifier) = qualifier {
                symbol.qualifiers = format!("{} {}", symbol.qualifiers, qualifier)
            }
        }

        for token in item.tokens() {
            match token {
                Token::Symbol(_symbol) => (),
                Token::Qualifier(_qualifier) => (),
                Token::Kind(_kind) => (),
                Token::Whitespace => (),
                Token::Identifier(identifier) => symbol.name = identifier.to_string(),
                Token::Annotation(_annotation) => (),
                Token::Self_(_self) => (),
                Token::Function(function) => symbol.value = function.to_string(),
                Token::Lifetime(_lifetime) => (),
                Token::Keyword(_keyword) => (),
                Token::Generic(_generic) => (),
                Token::Primitive(_primitive) => (),
                Token::Type(_type_) => (),
            }
        }

        symbol
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
