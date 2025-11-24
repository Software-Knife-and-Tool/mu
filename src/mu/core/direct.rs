//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// direct tagged types
#![allow(unused_braces)]
#![allow(clippy::identity_op)]
#[rustfmt::skip]
use {
    crate::{
        core::{
            type_::Type,
            env::Env,
            tag::{Tag, TagType},
        },
        namespaces::cache::Cache,
        types::{
            async_::Async,
            cons::Cons,
            function::Function,
            symbol::SymbolImage,
        },
    },
    modular_bitfield::specifiers::{B3, B56},
    num_enum::TryFromPrimitive,
};

// little endian direct tag format
#[derive(Copy, Clone)]
#[bitfield]
#[repr(u64)]
pub struct DirectTag {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[bits = 3]
    pub tag: TagType,
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[bits = 2]
    pub dtype: DirectType,
    pub ext: B3,
    pub data: B56,
}

impl Default for DirectTag {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Specifier, Copy, Clone, Eq, PartialEq)]
pub enum DirectType {
    Ext = 0,
    ByteVec = 1,
    Keyword = 2,
    String = 3,
}

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ExtType {
    Char = 0,
    Cons = 1,
    Fixnum = 2,
    Float = 3,
    Function = 4,
    Image = 5,
    Stream = 6,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DirectExt {
    Length(usize),
    ExtType(ExtType),
}

#[derive(Clone, Copy)]
pub enum DirectImage {
    Async(Async),
    Cons(Cons),
    Function(Function),
    Symbol(SymbolImage),
}

impl DirectImage {
    pub fn type_of(self) -> Type {
        match self {
            DirectImage::Async(_) => Type::Async,
            DirectImage::Cons(_) => Type::Cons,
            DirectImage::Function(_) => Type::Function,
            DirectImage::Symbol(_) => Type::Symbol,
        }
    }

    pub fn with_cache(self, env: &Env, type_id: u8) -> Tag {
        let tag_id = Cache::add(env, self);

        Tag::Direct(
            DirectTag::new()
                .with_data((tag_id << 8) | u64::from(type_id & 0xf))
                .with_ext(ExtType::Image as u8)
                .with_dtype(DirectType::Ext)
                .with_tag(TagType::Direct),
        )
    }
}

impl DirectTag {
    // 56 bit fixnum:     -36028797018963967 to 36028797018963968
    // 32 bit IEEE float: -3.40282347E+38    to -1.17549435E-38,
    //                     1.17549435E-38    to  3.40282347E+38
    pub const DIRECT_STR_MAX: usize = 7;

    pub fn length(tag: Tag) -> usize {
        match tag {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::String | DirectType::ByteVec | DirectType::Keyword => {
                    dtag.ext() as usize
                }
                DirectType::Ext => panic!(),
            },
            Tag::Indirect(_) => panic!(),
        }
    }

    pub fn to_tag(data: u64, ext: DirectExt, tag: DirectType) -> Tag {
        let ext: u8 = match ext {
            DirectExt::Length(size) => u8::try_from(size).unwrap(),
            DirectExt::ExtType(ext_type) => ext_type as u8,
        };

        Tag::Direct(
            DirectTag::new()
                .with_data(data)
                .with_ext(ext)
                .with_dtype(tag)
                .with_tag(TagType::Direct),
        )
    }

    pub fn type_of(self) -> Type {
        match self.dtype() {
            DirectType::ByteVec | DirectType::String => Type::Vector,
            DirectType::Keyword => Type::Keyword,
            DirectType::Ext => match ExtType::try_from(self.ext()).unwrap() {
                ExtType::Char => Type::Char,
                ExtType::Cons => Type::Cons,
                ExtType::Fixnum => Type::Fixnum,
                ExtType::Float => Type::Float,
                ExtType::Function => Type::Function,
                #[allow(clippy::cast_possible_truncation)]
                ExtType::Image => Type::try_from(self.data() as u8).unwrap(),
                ExtType::Stream => Type::Stream,
            },
        }
    }

    //
    // image cache
    //
    pub fn is_cached(tag: Tag) -> bool {
        match tag {
            Tag::Direct(fn_) => matches!(ExtType::try_from(fn_.ext()).unwrap(), ExtType::Image),
            Tag::Indirect(_) => panic!(),
        }
    }

    pub fn cache_ref(tag: Tag) -> (usize, u8) {
        match tag {
            Tag::Direct(fn_) => {
                let data = usize::try_from(fn_.data()).unwrap();

                #[allow(clippy::cast_possible_truncation)]
                (data >> 8, data as u8)
            }
            Tag::Indirect(_) => panic!(),
        }
    }

    pub fn image_detag(tag: Tag) -> (usize, u8) {
        match tag {
            Tag::Direct(direct) => match direct.ext() {
                val if val == ExtType::Image as u8 => {
                    let data = usize::try_from(direct.data()).unwrap();

                    #[allow(clippy::cast_possible_truncation)]
                    (data >> 8, data as u8)
                }
                _ => panic!(),
            },
            Tag::Indirect(_) => panic!(),
        }
    }

    //
    // direct function
    //
    pub fn function(index: usize) -> Tag {
        Self::to_tag(
            index as u64,
            DirectExt::ExtType(ExtType::Function),
            DirectType::Ext,
        )
    }

    pub fn function_destruct(tag: Tag) -> usize {
        match tag {
            Tag::Direct(tag) if tag.dtype() == DirectType::Ext && tag.ext() == 4 => {
                let data: u64 = tag.data();

                usize::try_from(data).unwrap()
            }
            _ => panic!(),
        }
    }

    //
    // direct cons
    //

    // can tag be sign extended to 64 from 28 bits?
    fn sext_from_tag(tag: Tag) -> Option<u32> {
        let u64_ = tag.as_u64();

        let mask_28: u64 = 0x0fff_ffff;
        let mask_32: u64 = 0xffff_ffff;
        let up_32: u64 = u64_ >> 28;
        let bot_28: u32 = (u64_ & mask_28).try_into().unwrap();
        let msb: u64 = (u64_ >> 27) & 1;

        match msb {
            0 if up_32 == 0 && msb == 0 => Some(bot_28),
            1 if up_32 == mask_32 && msb == 1 => Some(bot_28),
            _ => None,
        }
    }

    pub fn cons(car: Tag, cdr: Tag) -> Option<Tag> {
        let car_ = Self::sext_from_tag(car)?;

        Self::sext_from_tag(cdr).map(|cdr_| {
            Self::to_tag(
                (u64::from(car_) << 28) | u64::from(cdr_),
                DirectExt::ExtType(ExtType::Cons),
                DirectType::Ext,
            )
        })
    }

    pub fn cons_destruct(cons: Tag) -> (Tag, Tag) {
        assert!(cons.type_of() == Type::Cons || cons.null_());

        if cons.null_() {
            return (Tag::nil(), Tag::nil());
        }

        match cons {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::Ext => match dtag.ext().try_into() {
                    Ok(ExtType::Cons) => {
                        let mask_32: u64 = 0xffff_ffff;
                        let mask_28: u64 = 0x0fff_ffff;

                        let mut cons_u64: (u64, u64) = (dtag.data() >> 28, dtag.data() & mask_28);

                        if ((cons_u64.0 >> 27) & 1) != 0 {
                            cons_u64.0 |= mask_32 << 28;
                        }

                        if ((cons_u64.1 >> 27) & 1) != 0 {
                            cons_u64.1 |= mask_32 << 28;
                        }

                        (
                            (&cons_u64.0.to_le_bytes()).into(),
                            (&cons_u64.1.to_le_bytes()).into(),
                        )
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            Tag::Indirect(_) => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn direct_test() {
        assert!(true);
    }
}
