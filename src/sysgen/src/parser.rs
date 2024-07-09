//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
#![allow(unused_imports)]
use {
    crate::{
        binding::Binding,
        crate_::Crate,
        item::Item,
        parsers::{enum_::Enum, fn_::Function, impl_::Impl, mod_::Mod, type_::TypeAlias},
    },
    public_api::{tokens::Token, PublicItem},
    std::{
        io::{Error, ErrorKind},
        result::Result,
    },
};

pub trait Parser {
    fn parse(_: Item) -> Result<Binding, Error>;
}

impl Parser for Item<'_> {
    fn parse(item: Item) -> Result<Binding, Error> {
        let symbol = match item {
            Item::Module(_) => <Mod as Parser>::parse(item)?,
            Item::Enumeration(_) => <Enum as Parser>::parse(item)?,
            Item::Implementation(_) => <Impl as Parser>::parse(item)?,
            Item::Function(_) => <Function as Parser>::parse(item)?,
            Item::TypeAlias(_) => <TypeAlias as Parser>::parse(item)?,
            _ => return Err(Error::new(ErrorKind::Other, "unparsed item")),
        };

        Ok(symbol)
    }
}

/*
    Token::Annotation(annotation)
    Token::Function(_)
    Token::Generic(generic)
    Token::Identifier(_)
    Token::Keyword(keyword)
    Token::Kind(_)
    Token::Lifetime(_)
    Token::Primitive(_)
    Token::Qualifier(_)
    Token::Self_(_)
    Token::Symbol(_)
    Token::Type(_)
    Token::Whitespace
*/

#[cfg(test)]
mod tests {}
