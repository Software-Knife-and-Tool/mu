//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// tagged types
#[rustfmt::skip]
use {
    crate::{
        core::{
            tag::Tag,
        },
        types::{
            symbol::Symbol,
        },
    },
    num_enum::TryFromPrimitive,
};

// types
#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Type {
    Async,
    Bit,
    Byte,
    Char,
    Cons,
    Fixnum,
    Float,
    Function,
    Keyword,
    Null,
    Stream,
    Struct,
    Symbol,
    Vector,
    // synthetic
    T,
    List,
    String,
}

lazy_static! {
    pub static ref TYPEKEYMAP: Vec::<(Type, Tag)> = vec![
        (Type::Async, Symbol::keyword("async")),
        (Type::Bit, Symbol::keyword("bit")),
        (Type::Byte, Symbol::keyword("byte")),
        (Type::Char, Symbol::keyword("char")),
        (Type::Cons, Symbol::keyword("cons")),
        (Type::Fixnum, Symbol::keyword("fixnum")),
        (Type::Float, Symbol::keyword("float")),
        (Type::Function, Symbol::keyword("func")),
        (Type::Keyword, Symbol::keyword("keyword")),
        (Type::Null, Symbol::keyword("null")),
        (Type::Stream, Symbol::keyword("stream")),
        (Type::Struct, Symbol::keyword("struct")),
        (Type::Symbol, Symbol::keyword("symbol")),
        (Type::T, Symbol::keyword("t")),
        (Type::Vector, Symbol::keyword("vector")),
    ];
}

impl Type {
    pub fn to_key(self) -> Tag {
        TYPEKEYMAP
            .iter()
            .copied()
            .find(|map| self == map.0)
            .map(|map| map.1)
            .unwrap()
    }

    pub fn to_type_id(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn types() {
        assert!(true)
    }
}
