//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
use {
    crate::{
        item::{enumeration, module},
        symbols::Symbol,
    },
    public_api::{tokens::Token, PublicItem},
    std::{
        io::{Error, ErrorKind},
        result::Result,
    },
};

#[derive(Clone, Debug)]
pub enum Item {
    Module((Vec<Token>, Vec<Token>)),
    ExternCrate((Vec<Token>, Vec<Token>)),
    UseDeclaration((Vec<Token>, Vec<Token>)),
    Function((Vec<Token>, Vec<Token>)),
    TypeAlias((Vec<Token>, Vec<Token>)),
    Struct((Vec<Token>, Vec<Token>)),
    Enumeration((Vec<Token>, Vec<Token>)),
    Union((Vec<Token>, Vec<Token>)),
    ConstantItem((Vec<Token>, Vec<Token>)),
    StaticItem((Vec<Token>, Vec<Token>)),
    Trait((Vec<Token>, Vec<Token>)),
    Implementation((Vec<Token>, Vec<Token>)),
    ExternBlock((Vec<Token>, Vec<Token>)),
}

pub trait Parser {
    fn item(_: &PublicItem) -> Result<Item, Error>;
    fn parse(_: &PublicItem) -> Result<Symbol, Error>;
}

impl Parser for Symbol {
    /*
    Key: impl  Ident: core
    Key: impl  Sym: !
    Key: impl  Sym: <
    Qual: pub  Ident: ctrlc
    Qual: pub  Kind: enum
    Qual: pub  Kind: fn
    Qual: pub  Kind: mod
    Qual: pub  Kind: type
     */

    fn item(item: &PublicItem) -> Result<Item, Error> {
        let mut qualifiers = Vec::new();
        let token_vec = item.tokens().cloned().collect::<Vec<Token>>();

        println!("item: {item:?}");

        for (index, token) in token_vec.iter().enumerate() {
            //  println!("token: {token:?}");

            match token {
                Token::Qualifier(_) => {
                    qualifiers.push(token.clone());
                }
                Token::Identifier(_) => {
                    return Err(Error::new(ErrorKind::Other, "unrecognized item: ident"))
                }
                Token::Whitespace => (),
                Token::Kind(key) => match key.as_str() {
                    "impl" => {
                        return Ok(Item::Trait((
                            qualifiers,
                            Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        )))
                    }
                    "enum" => {
                        return Ok(Item::Enumeration((
                            qualifiers,
                            Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        )))
                    }
                    "fn" => {
                        return Ok(Item::Function((
                            qualifiers,
                            Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        )))
                    }
                    "mod" => {
                        return Ok(Item::Module((
                            qualifiers,
                            Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        )))
                    }
                    "type" => {
                        return Ok(Item::TypeAlias((
                            qualifiers,
                            Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        )))
                    }
                    _ => return Err(Error::new(ErrorKind::Other, "unparsed kind")),
                },
                Token::Keyword(key) => match key.as_str() {
                    "impl" => {
                        return Ok(Item::Trait((
                            qualifiers,
                            Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        )))
                    }
                    "enum" => {
                        return Ok(Item::Enumeration((
                            qualifiers,
                            Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        )))
                    }
                    "fn" => {
                        return Ok(Item::Function((
                            qualifiers,
                            Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        )))
                    }
                    "mod" => {
                        return Ok(Item::Module((
                            qualifiers,
                            Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        )))
                    }
                    "type" => {
                        return Ok(Item::TypeAlias((
                            qualifiers,
                            Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        )))
                    }
                    _ => return Err(Error::new(ErrorKind::Other, "unparsed keyword")),
                },
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("unrecognized item type: {token:?}"),
                    ))
                }
            }
        }

        panic!()
    }

    fn parse(public_item: &PublicItem) -> Result<Symbol, Error> {
        let item = Self::item(public_item)?;

        let symbol = match item {
            Item::Module(_) => <crate::symbols::Symbol as module::Module>::parse(item)?,
            Item::Enumeration(_) => {
                <crate::symbols::Symbol as enumeration::Enumeration>::parse(item)?
            }
            _ => return Err(Error::new(ErrorKind::Other, "unparsed item")),
        };

        Ok(symbol)
    }
}

/*
let output = match token {
    Token::Annotation(annotation) => {
        println!("[parse: stop ignoring anotation items] {annotation:?}");
        return Ok(symbol);
    }
    Token::Function(_) => machine.consume(&parser::Input::Function),
    Token::Generic(generic) => {
        println!("[parse: stop ignoring generics] {generic:?}");
        return Ok(symbol);
    },
    Token::Identifier(_) => machine.consume(&parser::Input::Identifier),
    Token::Keyword(keyword) => {
        println!("[parse: stop ignoring keywords] {keyword:?}");
        return Ok(symbol);
    }
    Token::Kind(_) => {
        symbol.kind_ = Some(token.clone());

        continue
    }
    Token::Lifetime(_) => machine.consume(&parser::Input::Qualifier),
    Token::Primitive(_) => machine.consume(&parser::Input::Qualifier),
    Token::Qualifier(_) => machine.consume(&parser::Input::Qualifier),
    Token::Self_(_) => machine.consume(&parser::Input::Self_),
    Token::Symbol(_) => machine.consume(&parser::Input::Symbol),
    Token::Type(_) => machine.consume(&parser::Input::Type),
    Token::Whitespace => continue,
};

match output {
    Ok(out) => match out.unwrap() {
        _ => (),
    },
    Err(e) => {
        println!("[parse: transition error] {e:?} token {token:?} state {:?}", machine.state());

        continue;
    }
}
*/

#[cfg(test)]
mod tests {}
