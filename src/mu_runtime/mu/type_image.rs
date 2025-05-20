//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env tagged types
#![allow(dead_code)]
use crate::{
    mu::{
        direct::{DirectExt, DirectTag, DirectType, ExtType},
        dynamic::Dynamic,
        env::Env,
        types::{Tag, Type},
    },
    types::{cons::Cons, function::Function, struct_::Struct, symbol::Symbol, vector::Vector},
    vectors::image::VectorImageType,
};

#[derive(Clone)]
pub enum TypeImage {
    Cons(Cons),
    Function(Function),
    Struct(Struct),
    Symbol(Symbol),
    Vector((Vector, VectorImageType)),
}

impl TypeImage {
    pub fn to_tag(&self, env: &Env, type_id: u8) -> Tag {
        let offset = Dynamic::images_push(env, self.clone());
        let data = ((offset << 4) as u64) | ((type_id & 0xf) as u64);

        DirectTag::to_tag(data, DirectExt::ExtType(ExtType::Image), DirectType::Ext)
    }

    pub fn type_of(&self) -> Type {
        match self {
            TypeImage::Cons(_) => Type::Cons,
            TypeImage::Function(_) => Type::Function,
            TypeImage::Struct(_) => Type::Struct,
            TypeImage::Symbol(_) => Type::Symbol,
            TypeImage::Vector(_) => Type::Vector,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn types() {
        assert_eq!(2 + 2, 4);
    }
}
