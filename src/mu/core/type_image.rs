//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env tagged types
#![allow(dead_code)]
use crate::{
    core::{
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::{cons::Cons, function::Function, struct_::Struct, symbol::Symbol, vector::Vector},
};

#[derive(Clone)]
pub enum TypeImage {
    Char(Tag),
    Cons(Cons),
    Fixnum(Tag),
    Float(Tag),
    Function(Function),
    Keyword(Tag),
    Namespace(Namespace),
    Struct(Struct),
    Symbol(Symbol),
    Vector(Vector),
}

impl TypeImage {
    pub fn null_(&self) -> bool {
        self.type_of() == Type::Null
    }

    pub fn type_of(&self) -> Type {
        match self {
            TypeImage::Char(_) => Type::Char,
            TypeImage::Cons(_) => Type::Cons,
            TypeImage::Fixnum(_) => Type::Fixnum,
            TypeImage::Float(_) => Type::Float,
            TypeImage::Function(_) => Type::Function,
            TypeImage::Keyword(keyword) => {
                if keyword.null_() {
                    Type::Null
                } else {
                    Type::Keyword
                }
            }
            TypeImage::Namespace(_) => Type::Namespace,
            TypeImage::Struct(_) => Type::Struct,
            TypeImage::Symbol(_) => Type::Symbol,
            TypeImage::Vector(_) => Type::Vector,
        }
    }
}

pub trait CoreFunction {}

impl CoreFunction for TypeImage {}

#[cfg(test)]
mod tests {
    #[test]
    fn types() {
        assert_eq!(2 + 2, 4);
    }
}
