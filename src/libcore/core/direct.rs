//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu direct tagged types
#![allow(unused_braces)]
#![allow(clippy::identity_op)]
use {
    crate::core::types::{Tag, TagType},
    modular_bitfield::specifiers::{B3, B56},
    num_enum::TryFromPrimitive,
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
    pub info: B3,
    pub data: B56,
}

#[derive(BitfieldSpecifier, Copy, Clone, Eq, PartialEq)]
pub enum DirectType {
    Ext = 0,
    ByteVector = 1,
    Keyword = 2,
    String = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ExtType {
    Fixnum = 0,
    Float = 1,
    AsyncId = 2,
    Cons = 3,
    Stream = 4,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DirectInfo {
    Length(usize),
    ExtType(ExtType),
}

impl DirectTag {
    pub const EXT_TYPE_FIXNUM: u8 = 0; // 56 bits: -36028797018963967 to 36028797018963968
    pub const EXT_TYPE_FLOAT: u8 = 1; // 32 bit IEEE float: -3.40282347E+38 to -1.17549435E-38,
                                      //                    1.17549435E-38 to 3.40282347E+38
    pub const EXT_TYPE_ASYNC: u8 = 2;
    pub const EXT_TYPE_CONS: u8 = 3;
    pub const EXT_TYPE_STREAM: u8 = 4;

    pub const DIRECT_STR_MAX: usize = 7;

    pub fn length(tag: Tag) -> usize {
        match tag {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::String | DirectType::ByteVector | DirectType::Keyword => {
                    dtag.info() as usize
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn to_direct(data: u64, info: DirectInfo, tag: DirectType) -> Tag {
        let info: u8 = match info {
            DirectInfo::Length(size) => size as u8,
            DirectInfo::ExtType(ext_type) => ext_type as u8,
        };

        let dir = DirectTag::new()
            .with_data(data)
            .with_info(info)
            .with_dtype(tag)
            .with_tag(TagType::Direct);

        Tag::Direct(dir)
    }

    //
    // direct cons
    //

    // can tag be sign extended to 64 from 28 bits?
    pub fn sext_from_tag(tag: Tag) -> Option<u32> {
        let u64_ = tag.as_u64();

        let mask_28: u64 = 0xfffffff;
        let mask_32: u64 = 0xffffffff;
        let up_32: u64 = u64_ >> 28;
        let bot_28: u32 = (u64_ & mask_28).try_into().unwrap();
        let msb: u64 = u64_ >> 27 & 1;

        match msb {
            0 if up_32 == 0 && msb == 0 => Some(bot_28),
            1 if up_32 == mask_32 && msb == 1 => Some(bot_28),
            _ => None,
        }
    }

    pub fn cons(car: Tag, cdr: Tag) -> Option<Tag> {
        match Self::sext_from_tag(car) {
            Some(car_) => Self::sext_from_tag(cdr).map(|cdr_| {
                Self::to_direct(
                    (car_ as u64) << 28 | cdr_ as u64,
                    DirectInfo::ExtType(ExtType::Cons),
                    DirectType::Ext,
                )
            }),
            None => None,
        }
    }

    pub fn car(cons: Tag) -> Tag {
        match cons {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::Ext => match dtag.info() {
                    Self::EXT_TYPE_CONS => {
                        let mask_32: u64 = 0xffffffff;
                        let mut u64_: u64 = dtag.data() >> 28;
                        let sign = (u64_ >> 27) & 1;

                        if sign != 0 {
                            u64_ |= mask_32 << 28;
                        }

                        Tag::from(&u64_.to_le_bytes())
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn cdr(cons: Tag) -> Tag {
        match cons {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::Ext => match dtag.info() {
                    Self::EXT_TYPE_CONS => {
                        let mask_28: u64 = 0xfffffff;
                        let mask_32: u64 = 0xffffffff;

                        let mut u64_: u64 = dtag.data() & mask_28;
                        let sign = (u64_ >> 27) & 1;

                        if sign != 0 {
                            u64_ |= mask_32 << 28;
                        }

                        Tag::from(&u64_.to_le_bytes())
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
        assert_eq!(2 + 2, 4);
    }
}
