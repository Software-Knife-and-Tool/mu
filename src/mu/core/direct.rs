//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env direct tagged types
#![allow(unused_braces)]
#![allow(clippy::identity_op)]
#[rustfmt::skip]
use {
    crate::{
        core::{
            tag::{Tag, TagType},
            type_::Type,
        },
        types::fixnum::Fixnum,
    },
    modular_bitfield::specifiers::{B3, B56},
    num_enum::TryFromPrimitive,
    std::convert::TryInto,
};

// little endian direct tag format
#[derive(Copy, Clone)]
#[bitfield]
#[repr(u64)]
pub struct DirectTag {
    #[bits = 3]
    pub tag: TagType,
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

#[derive(BitfieldSpecifier, Copy, Clone, Eq, PartialEq)]
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
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn to_tag(data: u64, ext: DirectExt, tag: DirectType) -> Tag {
        let ext: u8 = match ext {
            DirectExt::Length(size) => size as u8,
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

    //
    // direct function
    //
    pub fn function(arity: usize, offset: usize) -> Option<Tag> {
        let arity_res: Result<u16, _> = arity.try_into();
        let offset_res: Result<u16, _> = offset.try_into();

        let arity_: u16 = match arity_res {
            Ok(u16_) => u16_,
            Err(_) => None?,
        };

        let offset_: u16 = match offset_res {
            Ok(u16_) => u16_,
            Err(_) => None?,
        };

        Some(Self::to_tag(
            ((arity_ as u64) << 16) | offset_ as u64,
            DirectExt::ExtType(ExtType::Function),
            DirectType::Ext,
        ))
    }

    pub fn function_destruct(tag: Tag) -> (Tag, Tag) {
        match tag {
            Tag::Direct(tag) => (
                Fixnum::with_u64_or_panic(tag.data() >> 16),
                Fixnum::with_u64_or_panic(tag.data() & 0xffff),
            ),
            _ => panic!(),
        }
    }

    //
    // direct cons
    //

    // can tag be sign extended to 64 from 28 bits?
    pub fn sext_from_tag(tag: Tag) -> Option<u32> {
        let u64_ = tag.as_u64();

        let mask_28: u64 = 0x0fffffff;
        let mask_32: u64 = 0xffffffff;
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
                ((car_ as u64) << 28) | cdr_ as u64,
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
                        let mask_32: u64 = 0xffffffff;
                        let mask_28: u64 = 0x0fffffff;

                        let mut u64_car: u64 = dtag.data() >> 28;
                        let mut u64_cdr: u64 = dtag.data() & mask_28;

                        if ((u64_car >> 27) & 1) != 0 {
                            u64_car |= mask_32 << 28;
                        }

                        if ((u64_cdr >> 27) & 1) != 0 {
                            u64_cdr |= mask_32 << 28;
                        }

                        (
                            (&u64_car.to_le_bytes()).into(),
                            (&u64_cdr.to_le_bytes()).into(),
                        )
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn types() {
        assert!(true);
    }
}
