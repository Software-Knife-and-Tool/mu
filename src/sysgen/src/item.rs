//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
#![allow(unused_imports)]
use {
    crate::crate_::Crate,
    public_api::{tokens::Token, PublicItem},
    std::{
        cell::RefCell,
        fs::File,
        io::{Error, ErrorKind, Write},
        result::Result,
    },
};

#[derive(Clone, Debug)]
pub struct ItemState<'a> {
    pub crate_: &'a Crate,
    pub qualifiers: Vec<Token>,
    pub tokens: Vec<Token>,
}

#[derive(Clone, Debug)]
pub enum Item<'a> {
    ConstantItem(ItemState<'a>),
    Enumeration(ItemState<'a>),
    ExternBlock(ItemState<'a>),
    ExternCrate(ItemState<'a>),
    Function(ItemState<'a>),
    Implementation(ItemState<'a>),
    Module(ItemState<'a>),
    StaticItem(ItemState<'a>),
    Struct(ItemState<'a>),
    Symbol(ItemState<'a>),
    Trait(ItemState<'a>),
    TypeAlias(ItemState<'a>),
    Union(ItemState<'a>),
    UseDeclaration(ItemState<'a>),
}

impl Item<'_> {
    pub fn with_public_item<'a>(crate_: &'a Crate, item: &PublicItem) -> Result<Item<'a>, Error> {
        let mut qualifiers = Vec::new();
        let token_vec = item.tokens().cloned().collect::<Vec<Token>>();

        for (index, token) in token_vec.iter().enumerate() {
            match token {
                Token::Qualifier(_) => {
                    qualifiers.push(token.clone());
                }
                Token::Identifier(_sym) | Token::Symbol(_sym) => {
                    return Ok(Item::Symbol(ItemState {
                        crate_,
                        qualifiers,
                        tokens: Vec::from_iter(token_vec[index..].iter().cloned()),
                    }))
                }
                Token::Whitespace => (),
                Token::Kind(key) => match key.as_str() {
                    "impl" => {
                        return Ok(Item::Implementation(ItemState {
                            crate_,
                            qualifiers,
                            tokens: Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        }))
                    }
                    "enum" => {
                        return Ok(Item::Enumeration(ItemState {
                            crate_,
                            qualifiers,
                            tokens: Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        }))
                    }
                    "fn" => {
                        return Ok(Item::Function(ItemState {
                            crate_,
                            qualifiers,
                            tokens: Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        }))
                    }
                    "mod" => {
                        return Ok(Item::Module(ItemState {
                            crate_,
                            qualifiers,
                            tokens: Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        }))
                    }
                    "type" => {
                        return Ok(Item::TypeAlias(ItemState {
                            crate_,
                            qualifiers,
                            tokens: Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        }))
                    }
                    _ => return Err(Error::new(ErrorKind::Other, "unparsed kind")),
                },
                Token::Keyword(key) => match key.as_str() {
                    "impl" => {
                        return Ok(Item::Implementation(ItemState {
                            crate_,
                            qualifiers,
                            tokens: Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        }))
                    }
                    "enum" => {
                        return Ok(Item::Enumeration(ItemState {
                            crate_,
                            qualifiers,
                            tokens: Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        }))
                    }
                    "fn" => {
                        return Ok(Item::Function(ItemState {
                            crate_,
                            qualifiers,
                            tokens: Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        }))
                    }
                    "mod" => {
                        return Ok(Item::Module(ItemState {
                            crate_,
                            qualifiers,
                            tokens: Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        }))
                    }
                    "type" => {
                        return Ok(Item::TypeAlias(ItemState {
                            crate_,
                            qualifiers,
                            tokens: Vec::from_iter(token_vec[index + 1..].iter().cloned()),
                        }))
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
}

#[cfg(test)]
mod tests {}
