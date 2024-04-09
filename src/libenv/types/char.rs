//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env character class
use crate::{
    core::{
        direct::{DirectInfo, DirectTag, DirectType},
        env::Env,
        exception,
        types::Tag,
    },
    streams::write::Core as _,
    types::{
        stream::{Core as _, Stream},
        vecimage::{TypedVec, VecType},
        vector::Core as _,
    },
};

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Char {
    Direct(u64),
}

impl From<char> for Tag {
    fn from(ch: char) -> Tag {
        DirectTag::to_direct(ch as u64, DirectInfo::Length(1), DirectType::String)
    }
}

impl Char {
    pub fn as_char(env: &Env, ch: Tag) -> char {
        ((ch.data(env) & 0xff) as u8) as char
    }
}

pub trait Core {
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn view(_: &Env, _: Tag) -> Tag;
}

impl Core for Char {
    fn write(env: &Env, chr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let ch: u8 = (chr.data(env) & 0xff) as u8;

        if escape {
            match env.write_string("#\\", stream) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            let mut tmp = [0; 4];

            let phrase = match ch {
                0x20 => "space",
                0x9 => "tab",
                0xa => "linefeed",
                0xc => "page",
                0xd => "return",
                _ => (ch as char).encode_utf8(&mut tmp),
            };

            match env.write_string(phrase, stream) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            match Stream::write_char(env, stream, ch as char) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
    }

    fn view(env: &Env, chr: Tag) -> Tag {
        let vec = vec![chr];

        TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::types::Tag;

    #[test]
    fn as_tag() {
        match Tag::from('a') {
            _ => assert_eq!(true, true),
        }
    }
}
