//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// stream writer
use {
    crate::{
        core::{
            core::CORE,
            env::Env,
            exception::{self, Condition, Exception},
            tag::Tag,
            type_::Type,
        },
        types::{stream::Stream, symbol::Symbol},
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
            Err(Exception::new(
                env,
                Condition::Open,
                "mu:write-char",
                stream_tag,
            ))?
        }

        let streams_ref = block_on(CORE.streams.read());

        match streams_ref.get(&Stream::stream_id(stream_tag).unwrap()) {
            Some(stream_ref) => {
                let stream_ = block_on(stream_ref.read());

                if stream_.direction.eq_(&Symbol::keyword("input")) {
                    drop(streams_ref);
                    drop(stream_);

                    return Err(Exception::new(
                        env,
                        Condition::Type,
                        "mu:write-char",
                        stream_tag,
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
            Err(Exception::new(
                env,
                Condition::Open,
                "mu:write-byte",
                stream_tag,
            ))?
        }

        let streams_ref = block_on(CORE.streams.read());

        match streams_ref.get(&Stream::stream_id(stream_tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                if stream.direction.eq_(&Symbol::keyword("input")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(
                        env,
                        Condition::Type,
                        "mu:write-byte",
                        stream_tag,
                    ));
                }

                stream.system.write_byte(env, byte)
            }
            None => panic!(),
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
