//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
use {
    crate::{
        binding::{Binding, BindingItem},
        item::{Item, ItemState},
    },
    public_api::tokens::Token,
    rust_fsm::state_machine,
    std::{io::Error, result::Result},
};

#[derive(Debug)]
pub struct Enum<'a> {
    members: Vec<ItemState<'a>>,
}

state_machine! {
    #[derive(Debug, PartialEq)]
    #[repr(C)]
    enumeration(Identifier)

    Identifier(Symbol) => Symbol,
    Symbol => {
        Symbol => Symbol,
        Else => Symbol,
        End => Member,
    },
    Member => {
        Member => Member,
    },
}

impl Enum<'_> {
    pub fn parse(item: Item) -> Result<Binding, Error> {
        match item {
            Item::Enumeration(ItemState {
                ref crate_,
                ref qualifiers,
                ref tokens,
            }) => {
                let mut machine = enumeration::StateMachine::new();
                let mut name = String::new();
                let mut tokens_iter = tokens.iter();
                let mut members = Vec::new();

                loop {
                    if machine.state() == &enumeration::State::Member {
                        let item = crate_.parse_next_item().unwrap();
                        match item {
                            Some((Item::Symbol(item), _)) => {
                                members.push(item);

                                let _ = machine.consume(&enumeration::Input::Member);
                            }
                            _ => {
                                match item {
                                    Some((_, public_item)) => crate_.push_item(public_item),
                                    None => (),
                                }

                                break;
                            }
                        }
                    } else {
                        match tokens_iter.next() {
                            None => {
                                let _ = machine.consume(&enumeration::Input::End);
                            }
                            Some(token) => {
                                let _action = match token {
                                    Token::Identifier(str)
                                    | Token::Symbol(str)
                                    | Token::Type(str) => {
                                        name.push_str(&str);

                                        machine.consume(&enumeration::Input::Symbol)
                                    }
                                    Token::Whitespace => machine.consume(&enumeration::Input::Else),
                                    _ => machine.consume(&enumeration::Input::Else),
                                };

                                ()

                                /*
                                    match action.as_ref().unwrap() {
                                        Some(_action) => {
                                            println!("enum: {name} state: {:?}", machine.state());
                                        }
                                        None => {
                                            println!("enum: {name} None state: {:?}", machine.state())
                                        }
                                }
                                    */
                            }
                        }
                    }
                }

                let item = BindingItem::Enumeration(Enum { members });

                Ok(Binding {
                    qualifiers: Some(qualifiers.to_vec()),
                    item: Some(item),
                    name: if name.is_empty() { None } else { Some(name) },
                })
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {}
