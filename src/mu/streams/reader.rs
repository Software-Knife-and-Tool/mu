//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// stream reader
use {
    crate::{
        core_::{
            core::CORE,
            env::Env,
            exception::{self, Condition, Exception},
            tag::Tag,
            type_::Type,
        },
        streams::system::SystemStream,
        types::{char::Char, stream::Stream, symbol::Symbol},
    },
    futures_lite::future::block_on,
};

pub struct StreamReader;

impl StreamReader {
    pub fn read_char(env: &Env, stream_tag: Tag) -> exception::Result<Option<char>> {
        assert_eq!(stream_tag.type_of(), Type::Stream);

        if !Stream::is_open(stream_tag) {
            Err(Exception::new(
                env,
                Condition::Open,
                "mu:read-char",
                stream_tag,
            ))?
        }

        let streams_ref = block_on(CORE.streams.read());

        match streams_ref.get(&Stream::stream_id(stream_tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = block_on(stream_ref.write());

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(
                        env,
                        Condition::Stream,
                        "mu:read-char",
                        stream_tag,
                    ));
                }

                if stream.unch.null_() {
                    match stream.system.read_byte(env)? {
                        Some(byte) => Ok(Some(byte as char)),
                        None => Ok(None),
                    }
                } else {
                    let unch = stream.unch;

                    stream.unch = Tag::nil();
                    Ok(Some(Char::as_char(env, unch)))
                }
            }
            None => panic!(),
        }
    }

    pub fn read_byte(env: &Env, stream_tag: Tag) -> exception::Result<Option<u8>> {
        assert_eq!(stream_tag.type_of(), Type::Stream);

        if !Stream::is_open(stream_tag) {
            Err(Exception::new(
                env,
                Condition::Open,
                "mu:read-byte",
                stream_tag,
            ))?
        }

        let streams_ref = block_on(CORE.streams.read());

        match streams_ref.get(&Stream::stream_id(stream_tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = block_on(stream_ref.write());

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(
                        env,
                        Condition::Stream,
                        "mu:read-byte",
                        stream_tag,
                    ));
                }

                if stream.unch.null_() {
                    match stream.system.read_byte(env)? {
                        Some(byte) => Ok(Some(byte)),
                        None => Ok(None),
                    }
                } else {
                    let unch = stream.unch;

                    stream.unch = Tag::nil();

                    Ok(Some(Char::as_char(env, unch) as u8))
                }
            }
            None => panic!(),
        }
    }

    pub fn unread_char(env: &Env, stream_tag: Tag, ch: char) -> exception::Result<Option<()>> {
        assert_eq!(stream_tag.type_of(), Type::Stream);

        if !Stream::is_open(stream_tag) {
            Err(Exception::new(
                env,
                Condition::Open,
                "mu:unread-char",
                stream_tag,
            ))?
        }

        let streams_ref = block_on(CORE.streams.read());

        match streams_ref.get(&Stream::stream_id(stream_tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = block_on(stream_ref.write());

                SystemStream::close(&stream.system);

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(
                        env,
                        Condition::Type,
                        "mu:unread-char",
                        stream_tag,
                    ));
                }

                if stream.unch.null_() {
                    stream.unch = ch.into();

                    Ok(None)
                } else {
                    Err(Exception::new(
                        env,
                        Condition::Stream,
                        "mu:unread-char",
                        stream.unch,
                    ))
                }
            }
            None => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reader_test() {
        assert!(true);
    }
}
