//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env tagged types
#![allow(unused_braces)]
#![allow(clippy::identity_op)]
use {
    crate::{
        core::{
            apply::Core as _,
            direct::{DirectInfo, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            indirect::IndirectTag,
        },
        types::{
            char::{Char, Core as _},
            cons::{Cons, Core as _},
            core_stream::{Core as _, Stream},
            fixnum::{Core as _, Fixnum},
            float::{Core as _, Float},
            function::{Core as _, Function},
            indirect_vector::{TypedVector, VecType},
            struct_::{Core as _, Struct},
            symbol::{Core as _, Symbol},
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
#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Type {
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
#[bits = 3]
pub enum TagType {
    Direct = 0,   // 56 bit direct objects
    Cons = 1,     // cons heap tag
    Function = 2, // function heap tag
    Stream = 3,   // stream heap tag
    Struct = 4,   // struct heap tags
    Symbol = 5,   // symbol heap tag
    Vector = 6,   // vector heap tag
                  // and room for a pony
}

lazy_static! {
    static ref NIL: Tag = DirectTag::to_direct(
        (('l' as u64) << 16) | (('i' as u64) << 8) | ('n' as u64),
        DirectInfo::Length(3),
        DirectType::Keyword
    );
    pub static ref TYPEKEYMAP: Vec::<(Type, Tag)> = vec![
        (Type::Byte, Symbol::keyword("byte")),
        (Type::Char, Symbol::keyword("char")),
        (Type::Cons, Symbol::keyword("cons")),
        (Type::Fixnum, Symbol::keyword("fixnum")),
        (Type::Float, Symbol::keyword("float")),
        (Type::Function, Symbol::keyword("func")),
        (Type::Keyword, Symbol::keyword("keyword")),
        (Type::Namespace, Symbol::keyword("ns")),
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

    pub fn data(&self, env: &Env) -> u64 {
        match self {
            Tag::Direct(tag) => tag.data(),
            Tag::Indirect(heap) => {
                let heap_ref = block_on(env.heap.read());

                match heap_ref.image_info(heap.image_id() as usize) {
                    Some(info) => match Type::try_from(info.image_type()) {
                        Ok(etype) => etype as u64,
                        Err(_) => panic!(),
                    },
                    None => panic!(),
                }
            }
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
                        Ok(ExtType::Cons) => Type::Cons,
                        Ok(ExtType::Fixnum) => Type::Fixnum,
                        Ok(ExtType::Float) => Type::Float,
                        Ok(ExtType::Namespace) => Type::Namespace,
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

pub trait CoreFunction {
    fn crux_eq(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_typeof(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_repr(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_view(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Tag {
    fn crux_repr(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let type_ = fp.argv[0];
        let arg = fp.argv[1];

        env.fp_argv_check("crux:repr", &[Type::Keyword, Type::T], fp)?;

        fp.value = if type_.eq_(&Symbol::keyword("vector")) {
            let slice = arg.as_slice();

            TypedVector::<Vec<u8>> {
                vec: slice.to_vec(),
            }
            .vec
            .to_vector()
            .evict(env)
        } else if type_.eq_(&Symbol::keyword("t")) {
            if Vector::type_of(env, arg) == Type::Byte && Vector::length(env, arg) == 8 {
                let mut u64_: u64 = 0;

                for index in (0..8).rev() {
                    u64_ <<= 8;
                    u64_ |= match Vector::ref_heap(env, arg, index as usize) {
                        Some(byte) => Fixnum::as_i64(byte) as u64,
                        None => panic!(),
                    }
                }

                Tag::from(&u64_.to_le_bytes())
            } else {
                return Err(Exception::new(env, Condition::Type, "crux:repr", arg));
            }
        } else {
            return Err(Exception::new(env, Condition::Type, "crux:repr", type_));
        };

        Ok(())
    }

    fn crux_eq(_: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = if fp.argv[0].eq_(&fp.argv[1]) {
            Symbol::keyword("t")
        } else {
            Tag::nil()
        };

        Ok(())
    }

    fn crux_typeof(_: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match Tag::type_key(fp.argv[0].type_of()) {
            Some(type_key) => type_key,
            None => panic!(),
        };

        Ok(())
    }

    fn crux_view(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        fp.value = match tag.type_of() {
            Type::Char => Char::view(env, tag),
            Type::Cons => Cons::view(env, tag),
            Type::Fixnum => Fixnum::view(env, tag),
            Type::Float => Float::view(env, tag),
            Type::Function => Function::view(env, tag),
            Type::Stream => Stream::view(env, tag),
            Type::Struct => Struct::view(env, tag),
            Type::Vector => Vector::view(env, tag),
            _ => Symbol::view(env, tag),
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
