//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env stream type
#![allow(unused_braces)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::identity_op)]

use crate::{
    core::{
        direct::{DirectInfo, DirectTag, DirectType, ExtType},
        env::Env,
        exception::{self, Condition, Exception},
        lib::LIB,
        types::{Tag, Type},
    },
    streams::{
        system::{Core as _, SystemStream},
        write::Core as _,
    },
    types::{
        char::Char,
        fixnum::{Core as _, Fixnum},
        indirect_vector::{TypedVector, VecType},
        symbol::{Core as _, Symbol},
        vector::Core as _,
    },
};

use futures::executor::block_on;

// stream struct
pub struct Stream {
    pub system: SystemStream, // system stream
    pub index: usize,         // stream table index
    pub open: bool,           // stream open
    pub direction: Tag,       // :input | :output | :bidir (keyword)
    pub unch: Tag,            // pushbask for input streams
}

impl From<Stream> for Tag {
    fn from(stream: Stream) -> Tag {
        DirectTag::to_direct(
            stream.index as u64,
            DirectInfo::ExtType(ExtType::Stream),
            DirectType::Ext,
        )
    }
}

pub trait Core {
    fn to_stream_index(_: &Env, _: Tag) -> exception::Result<usize>;
    fn close(_: &Env, _: Tag);
    fn get_string(_: &Env, _: Tag) -> exception::Result<String>;
    fn is_open(_: &Env, _: Tag) -> bool;
    fn read_byte(_: &Env, _: Tag) -> exception::Result<Option<u8>>;
    fn read_char(_: &Env, _: Tag) -> exception::Result<Option<char>>;
    fn unread_char(_: &Env, _: Tag, _: char) -> exception::Result<Option<()>>;
    fn view(_: &Env, _: Tag) -> Tag;
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn write_byte(_: &Env, _: Tag, _: u8) -> exception::Result<Option<()>>;
    fn write_char(_: &Env, _: Tag, _: char) -> exception::Result<Option<()>>;
}

impl Core for Stream {
    fn to_stream_index(env: &Env, tag: Tag) -> exception::Result<usize> {
        match tag {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::Ext => match dtag.info().try_into() {
                    Ok(ExtType::Stream) => Ok(dtag.data() as usize),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn view(env: &Env, tag: Tag) -> Tag {
        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());
                let vec = vec![
                    Fixnum::with_or_panic(stream.index),
                    stream.direction,
                    stream.unch,
                ];

                TypedVector::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
            }
            None => panic!(),
        }
    }

    fn is_open(env: &Env, tag: Tag) -> bool {
        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                stream.open
            }
            None => panic!(),
        }
    }

    fn close(env: &Env, tag: Tag) {
        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                SystemStream::close(&stream.system);
            }
            None => panic!(),
        }
    }

    fn get_string(env: &Env, tag: Tag) -> exception::Result<String> {
        if !Self::is_open(env, tag) {
            return Err(Exception::new(env, Condition::Open, "crux:get-string", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                Ok(SystemStream::get_string(&stream.system).unwrap())
            }
            None => panic!(),
        }
    }

    fn write(env: &Env, tag: Tag, _: bool, stream_tag: Tag) -> exception::Result<()> {
        match tag.type_of() {
            Type::Stream => {
                let streams_ref = block_on(LIB.streams.read());

                match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
                    Some(stream_ref) => {
                        let stream = block_on(stream_ref.read());

                        env.write_string(
                            format!(
                                "#<stream: {} {} {} {}>",
                                stream.index,
                                match stream.system {
                                    SystemStream::Reader(_) | SystemStream::Writer(_) => ":file",
                                    SystemStream::String(_) => ":string",
                                    SystemStream::StdInput => ":standard-input",
                                    SystemStream::StdOutput => ":standard-output",
                                    SystemStream::StdError => ":error-output",
                                },
                                if Symbol::keyword("input").eq_(&stream.direction) {
                                    ":input"
                                } else if Symbol::keyword("output").eq_(&stream.direction) {
                                    ":output"
                                } else if Symbol::keyword("bidir").eq_(&stream.direction) {
                                    ":bidir"
                                } else {
                                    panic!()
                                },
                                if Self::is_open(env, tag) {
                                    ":open"
                                } else {
                                    ":close"
                                },
                            )
                            .as_str(),
                            stream_tag,
                        )
                    }
                    None => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    fn read_char(env: &Env, tag: Tag) -> exception::Result<Option<char>> {
        if !Self::is_open(env, tag) {
            return Err(Exception::new(env, Condition::Open, "crux:read-char", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = block_on(stream_ref.write());

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    drop(streams_ref);
                    drop(stream);
                    return Err(Exception::new(
                        env,
                        Condition::Stream,
                        "crux:read-char",
                        tag,
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

    fn read_byte(env: &Env, tag: Tag) -> exception::Result<Option<u8>> {
        if !Self::is_open(env, tag) {
            return Err(Exception::new(env, Condition::Open, "crux:read-byte", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = block_on(stream_ref.write());

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(
                        env,
                        Condition::Stream,
                        "crux:read-byte",
                        tag,
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

    fn unread_char(env: &Env, tag: Tag, ch: char) -> exception::Result<Option<()>> {
        if !Self::is_open(env, tag) {
            return Err(Exception::new(
                env,
                Condition::Open,
                "crux:unread-char",
                tag,
            ));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = block_on(stream_ref.write());

                SystemStream::close(&stream.system);

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(
                        env,
                        Condition::Type,
                        "crux:unread-char",
                        tag,
                    ));
                }

                if stream.unch.null_() {
                    stream.unch = ch.into();

                    Ok(None)
                } else {
                    Err(Exception::new(
                        env,
                        Condition::Stream,
                        "crux:unread-char",
                        stream.unch,
                    ))
                }
            }
            None => panic!(),
        }
    }

    fn write_char(env: &Env, tag: Tag, ch: char) -> exception::Result<Option<()>> {
        if !Self::is_open(env, tag) {
            return Err(Exception::new(env, Condition::Open, "crux:write-char", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                if stream.direction.eq_(&Symbol::keyword("input")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(env, Condition::Type, "crux:write-char", tag));
                }

                stream.system.write_byte(env, ch as u8)
            }
            None => panic!(),
        }
    }

    fn write_byte(env: &Env, tag: Tag, byte: u8) -> exception::Result<Option<()>> {
        if !Self::is_open(env, tag) {
            return Err(Exception::new(env, Condition::Open, "crux:write-byte", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                if stream.direction.eq_(&Symbol::keyword("input")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(env, Condition::Type, "crux:write-byte", tag));
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
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
