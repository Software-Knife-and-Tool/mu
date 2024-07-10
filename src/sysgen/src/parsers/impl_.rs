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
pub struct Impl {
    body: Vec<PublicItem>,
}

state_machine! {
    #[derive(Debug)]
    #[repr(C)]
    implementation(Implementation)

    Implementation(Identifier) => Members [Identifier],
    Members => {
        Members => Members [Parse],
        Else => End [Push],
    },
}

impl Impl {
    pub fn parse(item: Item) -> Result<Binding, Error> {
        match item {
            Item::Implementation(ItemState {
                // ref crate_,
                crate_: _,
                ref qualifiers,
                ref tokens,
            }) => {
                let mut machine = implementation::StateMachine::new();
                let _ = machine.consume(&implementation::Input::Members);

                let mut name: Option<String> = None;

                for token in tokens.clone() {
                    match token {
                        Token::Identifier(ident) => {
                            name = Some(ident);
                            let _ = machine.consume(&implementation::Input::Identifier);
                        }
                        _ => continue,
                    }
                }

                let item = BindingItem::Implementation(Impl { body: Vec::new() });

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
