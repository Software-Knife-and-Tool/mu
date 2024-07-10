//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
use {
    crate::{
        binding::{Binding, BindingItem},
        item::{Item, ItemState},
    },
    public_api::{tokens::Token, PublicItem},
    rust_fsm::*,
    std::{io::Error, result::Result},
};

#[derive(Debug)]
pub struct Function {
    body: Vec<PublicItem>,
}

state_machine! {
    #[derive(Debug)]
    #[repr(C)]
    function(Function)

    Identifier(Symbol) => Symbol,
    Symbol => {
        Symbol => Symbol,
        Else => Members,
    },
    Members => {
        Members => Members [Parse],
        Else => End [Push],
    },
}

impl Function {
    pub fn parse(item: Item) -> Result<Binding, Error> {
        match item {
            Item::Function(ItemState {
                // ref crate_,
                crate_: _,
                ref qualifiers,
                ref tokens,
            }) => {
                let mut machine = function::StateMachine::new();
                let _ = machine.consume(&function::Input::Members);

                let mut name: String = String::new();

                for token in tokens.clone() {
                    match token {
                        Token::Identifier(str)
                        | Token::Function(str)
                        | Token::Symbol(str)
                        | Token::Keyword(str)
                        | Token::Type(str) => {
                            name.push_str(&str);
                            let _ = machine.consume(&function::Input::Symbol);
                        }
                        _ => continue,
                    }
                }

                let item = BindingItem::Function(Function { body: Vec::new() });

                Ok(Binding {
                    qualifiers: Some(qualifiers.to_vec()),
                    item: Some(item),
                    name: Some(name),
                })
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {}
