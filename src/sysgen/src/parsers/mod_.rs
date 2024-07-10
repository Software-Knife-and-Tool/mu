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
pub struct Mod {
    body: Vec<PublicItem>,
}

state_machine! {
    #[derive(Debug)]
    #[repr(C)]
    module(Identifier)

    Identifier(Identifier) => Members [Identifier],
    Members => {
        Members => Members [Parse],
        Else => End [Push],
    },
}

impl Mod {
    pub fn parse(item: Item) -> Result<Binding, Error> {
        match item {
            Item::Module(ItemState {
                // ref crate_,
                crate_: _,
                ref qualifiers,
                ref tokens,
            }) => {
                let mut machine = module::StateMachine::new();
                let _ = machine.consume(&module::Input::Members);

                let mut name: Option<String> = None;

                for token in tokens.clone() {
                    match token {
                        Token::Identifier(ident) => {
                            name = Some(ident);
                            let _ = machine.consume(&module::Input::Identifier);
                        }
                        _ => continue,
                    }
                }

                let item = BindingItem::Module(Mod { body: Vec::new() });

                Ok(Binding {
                    qualifiers: Some(qualifiers.to_vec()),
                    item: Some(item),
                    name,
                })
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {}
