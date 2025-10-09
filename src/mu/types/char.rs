//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// character class
use crate::{
    core_::{
        direct::{DirectExt, DirectTag, DirectType, ExtType},
        env::Env,
        exception,
        tag::Tag,
    },
    streams::writer::StreamWriter,
    types::vector::Vector,
};

pub struct Char;

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
            StreamWriter::write_str(env, "#\\", stream)?;

            let mut tmp = [0; 4];

            let phrase = match ch {
                0x20 => "space",
                0x09 => "tab",
                0x0a => "linefeed",
                0x0c => "page",
                0x0d => "return",
                _ => (ch as char).encode_utf8(&mut tmp),
            };

            StreamWriter::write_str(env, phrase, stream)?;
        } else {
            StreamWriter::write_char(env, stream, ch as char)?;
        }

        Ok(())
    }

    pub fn view(env: &Env, chr: Tag) -> Tag {
        Vector::from(vec![chr]).with_heap(env)
    }
}

#[cfg(test)]
mod tests {
    use crate::core_::tag::Tag;

    #[test]
    fn as_tag() {
        match <char as Into<Tag>>::into('a') {
            _ => assert!(true),
        }
    }
}
