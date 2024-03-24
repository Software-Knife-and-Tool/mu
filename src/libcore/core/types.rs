//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu tagged types
#![allow(unused_braces)]
#![allow(clippy::identity_op)]
use {
    crate::{
        core::{
            apply::Core as _,
            direct::{DirectInfo, DirectTag, DirectType, ExtType},
            exception::{self, Condition, Exception},
            frame::Frame,
            indirect::IndirectTag,
            mu::Mu,
        },
        types::{
            char::{Char, Core as _},
            cons::{Cons, Core as _},
            fixnum::{Core as _, Fixnum},
            float::{Core as _, Float},
            function::{Core as _, Function},
            stream::{Core as _, Stream},
            struct_::{Core as _, Struct},
            symbol::{Core as _, Symbol},
            vecimage::{TypedVec, VecType},
            vector::{Core as _, Vector},
        },
    },
    num_enum::TryFromPrimitive,
    std::{convert::From, fmt},
};

use futures::executor::block_on;

// tag storage classes
#[derive(Copy, Clone)]
pub enum Tag {
    Direct(DirectTag),
    Indirect(IndirectTag),
}

// types
#[derive(PartialEq, Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Type {
    AsyncId,
    Byte,
    Char,
    Cons,
    Fixnum,
    Float,
    Function,
    Keyword,
    Null,
    Namespace,
    Stream,
    Struct,
    Symbol,
    Vector,
    // synthetic
    T,
    List,
    String,
}

#[derive(BitfieldSpecifier, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TagType {
    Direct = 0,   // 56 bit direct objects
    Cons = 1,     // cons heap tag
    Function = 2, // function heap tag
    Stream = 3,   // stream heap tag
    Struct = 4,   // struct heap tags
    Symbol = 5,   // symbol heap tag
    Vector = 6,   // vector heap tag
    Namespace = 7,
}

lazy_static! {
    static ref NIL: Tag = DirectTag::to_direct(
        (('l' as u64) << 16) | (('i' as u64) << 8) | ('n' as u64),
        DirectInfo::Length(3),
        DirectType::Keyword
    );
    pub static ref TYPEKEYMAP: Vec::<(Type, Tag)> = vec![
        (Type::AsyncId, Symbol::keyword("asyncid")),
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

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}: ", self.as_u64()).unwrap();
        match self {
            Tag::Direct(direct) => write!(f, "direct: type {:?}", direct.dtype() as u8),
            Tag::Indirect(indirect) => write!(f, "indirect: type {:?}", indirect.tag()),
        }
    }
}

impl From<&[u8; 8]> for Tag {
    fn from(u8_: &[u8; 8]) -> Tag {
        Self::from_slice(u8_)
    }
}

impl Tag {
    pub const NTYPES: u8 = 15;
    pub fn data(&self, mu: &Mu) -> u64 {
        let heap_ref = block_on(mu.heap.read());

        match self {
            Tag::Direct(tag) => tag.data(),
            Tag::Indirect(heap) => match heap_ref.image_info(heap.image_id() as usize) {
                Some(info) => match Type::try_from(info.image_type()) {
                    Ok(etype) => etype as u64,
                    Err(_) => panic!(),
                },
                None => panic!(),
            },
        }
    }

    pub fn as_slice(&self) -> [u8; 8] {
        match self {
            Tag::Direct(tag) => tag.into_bytes(),
            Tag::Indirect(tag) => tag.into_bytes(),
        }
    }

    pub fn eq_(&self, tag: &Tag) -> bool {
        self.as_u64() == tag.as_u64()
    }

    pub fn null_(&self) -> bool {
        self.eq_(&Self::nil())
    }

    pub fn nil() -> Tag {
        *NIL
    }

    pub fn as_u64(&self) -> u64 {
        u64::from_le_bytes(self.as_slice())
    }

    pub fn from_slice(bits: &[u8]) -> Tag {
        let mut data: [u8; 8] = 0u64.to_le_bytes();

        for (src, dst) in bits.iter().zip(data.iter_mut()) {
            *dst = *src
        }

        let tag: u8 = (u64::from_le_bytes(data) & 0x7) as u8;
        let u64_: u64 = u64::from_le_bytes(data);

        match tag {
            tag if tag == TagType::Direct as u8 => Tag::Direct(DirectTag::from(u64_)),
            _ => Tag::Indirect(IndirectTag::from(u64_)),
        }
    }

    pub fn type_of(&self) -> Type {
        if self.null_() {
            Type::Null
        } else {
            match self {
                Tag::Direct(direct) => match direct.dtype() {
                    DirectType::ByteVector => Type::Vector,
                    DirectType::String => Type::Char,
                    DirectType::Keyword => Type::Keyword,
                    DirectType::Ext => match ExtType::try_from(direct.info()) {
                        Ok(ExtType::AsyncId) => Type::AsyncId,
                        Ok(ExtType::Cons) => Type::Cons,
                        Ok(ExtType::Fixnum) => Type::Fixnum,
                        Ok(ExtType::Float) => Type::Float,
                        Ok(ExtType::Stream) => Type::Stream,
                        _ => panic!("direct type botch {:x}", self.as_u64()),
                    },
                },
                Tag::Indirect(indirect) => match indirect.tag() {
                    TagType::Cons => Type::Cons,
                    TagType::Function => Type::Function,
                    TagType::Struct => Type::Struct,
                    TagType::Symbol => Type::Symbol,
                    TagType::Vector => Type::Vector,
                    _ => panic!("indirect type botch {:x}", self.as_u64()),
                },
            }
        }
    }

    pub fn type_key(type_: Type) -> Option<Tag> {
        TYPEKEYMAP
            .iter()
            .copied()
            .find(|map| type_ == map.0)
            .map(|map| map.1)
    }

    pub fn key_type(tag: Tag) -> Option<Type> {
        TYPEKEYMAP
            .iter()
            .copied()
            .find(|map| tag.eq_(&map.1))
            .map(|map| map.0)
    }
}

pub trait MuFunction {
    fn core_eq(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn core_typeof(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn core_repr(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn core_view(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Tag {
    fn core_repr(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let type_ = fp.argv[0];
        let arg = fp.argv[1];

        fp.value = match mu.fp_argv_check("repr", &[Type::Keyword, Type::T], fp) {
            Ok(_) => {
                if type_.eq_(&Symbol::keyword("vector")) {
                    let slice = arg.as_slice();

                    TypedVec::<Vec<u8>> {
                        vec: slice.to_vec(),
                    }
                    .vec
                    .to_vector()
                    .evict(mu)
                } else if type_.eq_(&Symbol::keyword("t")) {
                    if Vector::type_of(mu, arg) == Type::Byte && Vector::length(mu, arg) == 8 {
                        let mut u64_: u64 = 0;

                        for index in (0..8).rev() {
                            u64_ <<= 8;
                            u64_ |= match Vector::ref_(mu, arg, index as usize) {
                                Some(byte) => Fixnum::as_i64(byte) as u64,
                                None => panic!(),
                            }
                        }

                        Tag::from(&u64_.to_le_bytes())
                    } else {
                        return Err(Exception::new(Condition::Type, "repr", arg));
                    }
                } else {
                    return Err(Exception::new(Condition::Type, "repr", type_));
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_eq(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = if fp.argv[0].eq_(&fp.argv[1]) {
            Symbol::keyword("t")
        } else {
            Tag::nil()
        };

        Ok(())
    }

    fn core_typeof(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match Tag::type_key(fp.argv[0].type_of()) {
            Some(type_key) => type_key,
            None => panic!(),
        };

        Ok(())
    }

    fn core_view(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        fp.value = match tag.type_of() {
            Type::Char => Char::view(mu, tag),
            Type::Cons => Cons::view(mu, tag),
            Type::Fixnum => Fixnum::view(mu, tag),
            Type::Float => Float::view(mu, tag),
            Type::Function => Function::view(mu, tag),
            Type::Stream => Stream::view(mu, tag),
            Type::Struct => Struct::view(mu, tag),
            Type::Vector => Vector::view(mu, tag),
            _ => Symbol::view(mu, tag),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn types() {
        assert_eq!(2 + 2, 4);
    }
}
