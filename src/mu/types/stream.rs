//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env stream functions
#![allow(unused_braces)]
#![allow(dead_code)]
#![allow(clippy::identity_op)]

use crate::{
    core::{
        apply::Core as _,
        direct::{DirectExt, DirectTag, DirectType, ExtType},
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        lib::LIB,
        types::{Tag, Type},
    },
    streams::{
        core::StreamBuilder,
        system::{Core as _, SystemStream},
        write::Core as _,
    },
    types::{
        char::Char,
        fixnum::{Core as _, Fixnum},
        symbol::{Core as _, Symbol},
        vector::{Core as _, Vector},
    },
    vectors::core::Core as _,
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
        DirectTag::to_tag(
            stream.index as u64,
            DirectExt::ExtType(ExtType::Stream),
            DirectType::Ext,
        )
    }
}

pub trait Core {
    fn stream_index(_: &Env, _: Tag) -> exception::Result<usize>;
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
    fn stream_index(_: &Env, tag: Tag) -> exception::Result<usize> {
        match tag {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::Ext => match dtag.ext().try_into() {
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

        match streams_ref.get(Self::stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());
                let vec = vec![
                    Fixnum::with_or_panic(stream.index),
                    stream.direction,
                    stream.unch,
                ];

                Vector::from(vec).evict(env)
            }
            None => panic!(),
        }
    }

    fn is_open(env: &Env, tag: Tag) -> bool {
        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                stream.open
            }
            None => panic!(),
        }
    }

    fn close(env: &Env, tag: Tag) {
        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                SystemStream::close(&stream.system);
            }
            None => panic!(),
        }
    }

    fn get_string(env: &Env, tag: Tag) -> exception::Result<String> {
        if !Self::is_open(env, tag) {
            return Err(Exception::new(env, Condition::Open, "mu:get-string", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::stream_index(env, tag).unwrap()) {
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

                match streams_ref.get(Self::stream_index(env, tag).unwrap()) {
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
            return Err(Exception::new(env, Condition::Open, "mu:read-char", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = block_on(stream_ref.write());

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    drop(streams_ref);
                    drop(stream);
                    return Err(Exception::new(env, Condition::Stream, "mu:read-char", tag));
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
            return Err(Exception::new(env, Condition::Open, "mu:read-byte", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = block_on(stream_ref.write());

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(env, Condition::Stream, "mu:read-byte", tag));
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
            return Err(Exception::new(env, Condition::Open, "mu:unread-char", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = block_on(stream_ref.write());

                SystemStream::close(&stream.system);

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(env, Condition::Type, "mu:unread-char", tag));
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

    fn write_char(env: &Env, tag: Tag, ch: char) -> exception::Result<Option<()>> {
        if !Self::is_open(env, tag) {
            return Err(Exception::new(env, Condition::Open, "mu:write-char", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                if stream.direction.eq_(&Symbol::keyword("input")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(env, Condition::Type, "mu:write-char", tag));
                }

                stream.system.write_byte(env, ch as u8)
            }
            None => panic!(),
        }
    }

    fn write_byte(env: &Env, tag: Tag, byte: u8) -> exception::Result<Option<()>> {
        if !Self::is_open(env, tag) {
            return Err(Exception::new(env, Condition::Open, "mu:write-byte", tag));
        }

        let streams_ref = block_on(LIB.streams.read());

        match streams_ref.get(Self::stream_index(env, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                if stream.direction.eq_(&Symbol::keyword("input")) {
                    drop(streams_ref);
                    drop(stream);

                    return Err(Exception::new(env, Condition::Type, "mu:write-byte", tag));
                }

                stream.system.write_byte(env, byte)
            }
            None => panic!(),
        }
    }
}

pub trait CoreFunction {
    fn mu_close(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_flush(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_get_string(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_open(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_openp(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_read_byte(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_read_char(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_unread_char(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_write_byte(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_write_char(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Stream {
    fn mu_close(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];

        env.fp_argv_check("mu:close", &[Type::Stream], fp)?;

        fp.value = if Self::is_open(env, stream) {
            Self::close(env, stream);
            Symbol::keyword("t")
        } else {
            Tag::nil()
        };

        Ok(())
    }

    fn mu_openp(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];

        env.fp_argv_check("mu:openp", &[Type::Stream], fp)?;
        fp.value = if Self::is_open(env, stream) {
            stream
        } else {
            Tag::nil()
        };

        Ok(())
    }

    fn mu_open(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let st_type = fp.argv[0];
        let st_dir = fp.argv[1];
        let st_arg = fp.argv[2];
        let st_error_p = fp.argv[3];

        env.fp_argv_check(
            "mu:open",
            &[Type::Keyword, Type::Keyword, Type::String, Type::T],
            fp,
        )?;

        fp.value = if st_type.eq_(&Symbol::keyword("file")) {
            let arg = Vector::as_string(env, st_arg);

            let stream = if st_dir.eq_(&Symbol::keyword("input")) {
                StreamBuilder::new().file(arg).input().build(env, &LIB)
            } else if st_dir.eq_(&Symbol::keyword("output")) {
                StreamBuilder::new().file(arg).output().build(env, &LIB)
            } else if st_error_p.null_() {
                Ok(Tag::nil())
            } else {
                return Err(Exception::new(env, Condition::Type, "mu:open", st_dir));
            };

            stream?
        } else if st_type.eq_(&Symbol::keyword("string")) {
            let arg = Vector::as_string(env, st_arg);

            let stream = if st_dir.eq_(&Symbol::keyword("input")) {
                StreamBuilder::new().string(arg).input().build(env, &LIB)
            } else if st_dir.eq_(&Symbol::keyword("output")) {
                StreamBuilder::new().string(arg).output().build(env, &LIB)
            } else if st_dir.eq_(&Symbol::keyword("bidir")) {
                StreamBuilder::new().string(arg).bidir().build(env, &LIB)
            } else if st_error_p.null_() {
                Ok(Tag::nil())
            } else {
                return Err(Exception::new(env, Condition::Type, "mu:open", st_dir));
            };

            stream?
        } else {
            return Err(Exception::new(env, Condition::Type, "mu:open", st_type));
        };

        Ok(())
    }

    fn mu_flush(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        env.fp_argv_check("mu:flush", &[Type::Stream], fp)?;

        let streams_ref = block_on(LIB.streams.read());

        fp.value = match streams_ref.get(Self::stream_index(env, tag)?) {
            Some(stream_ref) => {
                if Self::is_open(env, tag) {
                    let stream = block_on(stream_ref.read());

                    if stream.direction.eq_(&Symbol::keyword("output")) {
                        SystemStream::flush(&stream.system).unwrap()
                    }

                    tag
                } else {
                    Tag::nil()
                }
            }
            None => panic!(),
        };

        Ok(())
    }

    fn mu_get_string(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];

        env.fp_argv_check("mu:get-string", &[Type::Stream], fp)?;

        let string = Self::get_string(env, stream)?;
        fp.value = Vector::from(string).evict(env);

        Ok(())
    }

    fn mu_read_char(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        env.fp_argv_check("mu:read-char", &[Type::Stream, Type::T, Type::T], fp)?;
        fp.value = match Self::read_char(env, stream)? {
            Some(ch) => ch.into(),
            None if eof_error_p.null_() => eof_value,
            None => return Err(Exception::new(env, Condition::Eof, "mu:read-char", stream)),
        };

        Ok(())
    }

    fn mu_read_byte(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        env.fp_argv_check("mu:read-byte", &[Type::Stream, Type::T, Type::T], fp)?;
        fp.value = match Self::read_byte(env, stream)? {
            Some(byte) => byte.into(),
            None if eof_error_p.null_() => eof_value,
            None => return Err(Exception::new(env, Condition::Eof, "mu:read-byte", stream)),
        };

        Ok(())
    }

    fn mu_unread_char(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ch = fp.argv[0];
        let stream = fp.argv[1];

        env.fp_argv_check("mu:unread-char", &[Type::Char, Type::Stream], fp)?;
        fp.value = match Self::unread_char(env, stream, Char::as_char(env, ch))? {
            Some(_) => panic!(),
            None => ch,
        };

        Ok(())
    }

    fn mu_write_char(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ch = fp.argv[0];
        let stream = fp.argv[1];

        env.fp_argv_check("mu:write-char", &[Type::Char, Type::Stream], fp)?;
        Self::write_char(env, stream, Char::as_char(env, ch))?;
        fp.value = ch;

        Ok(())
    }

    fn mu_write_byte(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let byte = fp.argv[0];
        let stream = fp.argv[1];

        env.fp_argv_check("mu:write-byte", &[Type::Byte, Type::Stream], fp)?;
        Self::write_byte(env, stream, Fixnum::as_i64(byte) as u8)?;
        fp.value = byte;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::streams::system::{SystemStream, SystemStreamBuilder};

    #[test]
    fn stream_builder() {
        let stream = SystemStreamBuilder::new()
            .string("hello".into())
            .input()
            .build();

        match stream {
            Some(stream) => match stream {
                SystemStream::String(_) => assert_eq!(true, true),
                _ => assert_eq!(true, false),
            },
            None => assert_eq!(true, false),
        }
    }
}
