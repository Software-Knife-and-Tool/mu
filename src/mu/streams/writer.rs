//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// stream writer
use {
    crate::{
        core::{
            core_::CORE,
            env::Env,
            exception::{self, Condition, Exception},
            tag::Tag,
            type_::Type,
        },
        types::{
            async_::Async, char::Char, cons::Cons, fixnum::Fixnum, float::Float,
            function::Function, stream::Stream, struct_::Struct, symbol::Symbol, vector::Vector,
        },
    },
    futures_lite::future::block_on,
};

pub struct StreamWriter;

impl StreamWriter {
    pub fn write_str(env: &Env, str: &str, stream_tag: Tag) -> exception::Result<()> {
        assert_eq!(stream_tag.type_of(), Type::Stream);

        for ch in str.chars() {
            StreamWriter::write_char(env, stream_tag, ch)?;
        }

        Ok(())
    }

    pub fn write_char(env: &Env, stream_tag: Tag, ch: char) -> exception::Result<Option<()>> {
        assert_eq!(stream_tag.type_of(), Type::Stream);

        if !Stream::is_open(stream_tag) {
            Err(Exception::err(
                env,
                stream_tag,
                Condition::Open,
                "mu:write-char",
            ))?;
        }

        let core_streams_ref = block_on(CORE.streams.read());

        match core_streams_ref.get(&Stream::stream_id(stream_tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                if stream.direction.eq_(&Symbol::keyword("input")) {
                    drop(core_streams_ref);
                    drop(stream);

                    return Err(Exception::err(
                        env,
                        stream_tag,
                        Condition::Type,
                        "mu:write-char",
                    ));
                }

                StreamWriter::write_byte(env, stream_tag, ch as u8)
            }
            None => panic!(),
        }
    }

    pub fn write_byte(env: &Env, stream_tag: Tag, byte: u8) -> exception::Result<Option<()>> {
        assert_eq!(stream_tag.type_of(), Type::Stream);

        if !Stream::is_open(stream_tag) {
            Err(Exception::err(
                env,
                stream_tag,
                Condition::Open,
                "mu:write-byte",
            ))?;
        }

        let core_streams_ref = block_on(CORE.streams.read());

        match core_streams_ref.get(&Stream::stream_id(stream_tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                if stream.direction.eq_(&Symbol::keyword("input")) {
                    drop(core_streams_ref);
                    drop(stream);

                    return Err(Exception::err(
                        env,
                        stream_tag,
                        Condition::Type,
                        "mu:write-byte",
                    ));
                }

                stream.system.write_byte(env, byte)
            }
            None => panic!(),
        }
    }

    pub fn write(env: &Env, tag: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        assert_eq!(stream.type_of(), Type::Stream);

        match tag.type_of() {
            Type::Async => Async::write(env, tag, escape, stream),
            Type::Char => Char::write(env, tag, escape, stream),
            Type::Cons => Cons::write(env, tag, escape, stream),
            Type::Fixnum => Fixnum::write(env, tag, escape, stream),
            Type::Float => Float::write(env, tag, escape, stream),
            Type::Function => Function::write(env, tag, escape, stream),
            Type::Stream => Stream::write(env, tag, escape, stream),
            Type::Struct => Struct::write(env, tag, escape, stream),
            Type::Symbol | Type::Null | Type::Keyword => Symbol::write(env, tag, escape, stream),
            Type::Vector => Vector::write(env, tag, escape, stream),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
