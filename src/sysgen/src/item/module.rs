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
    module(Module)

    Module => {
        Identifier => Parse [Identifier],
    }
}

pub trait Module {
    fn parse(_: Item) -> Result<Symbol, Error>;
}

impl Module for Symbol {
    fn parse(item: Item) -> Result<Symbol, Error> {
        let mut symbol = Symbol {
            item: None,
            name: None,
            qualifiers: None,
        };

        symbol.item = Some(item.clone());

        match item {
            Item::Module((qualifiers, tokens)) => {
                symbol.qualifiers = Some(qualifiers);

                let mut machine = module::StateMachine::new();

                for token in tokens {
                    match token {
                        Token::Identifier(_) => {
                            let output = machine.consume(&module::Input::Identifier);

                            match output {
                                Ok(out) => match out.unwrap() {
                                    module::Output::Identifier => symbol.name = Some(token.clone()),
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
