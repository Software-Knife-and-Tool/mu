//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
use {
    crate::{parser::Item, symbols::Symbol},
    public_api::tokens::Token,
    rust_fsm::*,
    std::{io::Error, result::Result},
};

state_machine! {
    #[derive(Debug)]
    #[repr(C)]
    enumeration(Enumeration)

    Enumeration => {
        Identifier => Parse [Identifier],
    }
}

pub trait Enumeration {
    fn parse(_: Item) -> Result<Symbol, Error>;
}

impl Enumeration for Symbol {
    fn parse(item: Item) -> Result<Symbol, Error> {
        let mut symbol = Symbol {
            item: None,
            name: None,
            qualifiers: None,
        };

        symbol.item = Some(item.clone());

        match item {
            Item::Enumeration((qualifiers, tokens)) => {
                symbol.qualifiers = Some(qualifiers);

                let mut machine = enumeration::StateMachine::new();

                for token in tokens {
                    match token {
                        Token::Identifier(_) => {
                            let output = machine.consume(&enumeration::Input::Identifier);

                            match output {
                                Ok(out) => match out.unwrap() {
                                    enumeration::Output::Identifier => {
                                        symbol.name = Some(token.clone())
                                    }
                                },
                                Err(e) => {
                                    println!("[parse: module syntax error] {e:?} token {token:?} state {:?}", machine.state());

                                    continue;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => panic!(),
        }

        Ok(symbol)
    }
}

#[cfg(test)]
mod tests {}
