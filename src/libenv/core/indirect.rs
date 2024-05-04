//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env indirect
#![allow(unused_braces)]
#![allow(clippy::identity_op)]
use crate::{
    core::types::{Tag, TagType, Type},
    modular_bitfield::specifiers::{B2, B59},
    types::symbol::{Core as _, Symbol},
};

// little-endian tag format
#[derive(Copy, Clone)]
#[bitfield]
#[repr(u64)]
pub struct IndirectTag {
    #[bits = 3]
    pub tag: TagType,
    pub heap_id: B2,
    pub image_id: B59,
}

impl Default for IndirectTag {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static! {
    static ref TYPEMAP: Vec<(Tag, Type)> = vec![
        (Symbol::keyword("cons"), Type::Cons),
        (Symbol::keyword("func"), Type::Function),
        (Symbol::keyword("nil"), Type::Null),
        (Symbol::keyword("stream"), Type::Stream),
        (Symbol::keyword("struct"), Type::Struct),
        (Symbol::keyword("symbol"), Type::Symbol),
        (Symbol::keyword("t"), Type::T),
        (Symbol::keyword("vector"), Type::Vector),
    ];
}

pub trait Core {
    fn to_indirect_type(_: Tag) -> Option<Type>;
}

impl Core for IndirectTag {
    fn to_indirect_type(keyword: Tag) -> Option<Type> {
        TYPEMAP
            .iter()
            .copied()
            .find(|tab| keyword.eq_(&tab.0))
            .map(|tab| tab.1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn image() {
        assert_eq!(2 + 2, 4);
    }
}
