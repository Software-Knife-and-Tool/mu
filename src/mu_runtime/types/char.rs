//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env character class
#![allow(dead_code)]
use crate::{
    mu::{
        direct::{DirectExt, DirectTag, DirectType, ExtType},
        env::Env,
        exception,
        types::Tag,
    },
    streams::write::Write as _,
    types::{stream::Write as _, vector::Vector},
};

#[derive(Copy, Clone)]
pub enum Char {
    Direct(u64),
}

impl From<char> for Tag {
    fn from(ch: char) -> Tag {
        DirectTag::to_tag(
            ch as u64,
            DirectExt::ExtType(ExtType::Char),
            DirectType::Ext,
        )
    }
}

impl Char {
    pub fn as_char(env: &Env, ch: Tag) -> char {
        ((ch.data(env) & 0xff) as u8) as char
    }

    pub fn write(env: &Env, chr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let ch: u8 = (chr.data(env) & 0xff) as u8;

        if escape {
            env.write_string("#\\", stream)?;

            let mut tmp = [0; 4];

            let phrase = match ch {
                0x20 => "space",
                0x09 => "tab",
                0x0a => "linefeed",
                0x0c => "page",
                0x0d => "return",
                _ => (ch as char).encode_utf8(&mut tmp),
            };

            env.write_string(phrase, stream)?;
        } else {
            env.write_char(stream, ch as char)?;
        }

        Ok(())
    }

    pub fn view(env: &Env, chr: Tag) -> Tag {
        Vector::from(vec![chr]).evict(env)
    }
}

#[cfg(test)]
mod tests {
    use crate::mu::types::Tag;

    #[test]
    fn as_tag() {
        match <char as Into<Tag>>::into('a') {
            _ => assert_eq!(true, true),
        }
    }
}
