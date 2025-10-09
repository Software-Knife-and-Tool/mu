//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! stream type
use {
    crate::{
        core_::{
            apply::Apply as _,
            core::CORE,
            direct::{DirectExt, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            reader::Reader,
            tag::Tag,
            type_::Type,
            writer::Writer,
        },
        streams::{
            builder::StreamBuilder, reader::StreamReader, system::SystemStream,
            writer::StreamWriter,
        },
        types::{char::Char, fixnum::Fixnum, symbol::Symbol, vector::Vector},
    },
    futures_lite::future::block_on,
};

// stream struct
pub struct Stream {
    pub system: SystemStream, // system stream
    pub id: u64,              // stream table index
    pub open: bool,           // stream open
    pub direction: Tag,       // :input | :output | :bidir (keyword)
    pub unch: Tag,            // pushbask for input streams
}

impl From<Stream> for Tag {
    fn from(stream: Stream) -> Tag {
        DirectTag::to_tag(
            stream.id,
            DirectExt::ExtType(ExtType::Stream),
            DirectType::Ext,
        )
    }
}

impl Stream {
    pub fn write(env: &Env, tag: Tag, _: bool, stream_tag: Tag) -> exception::Result<()> {
        assert_eq!(stream_tag.type_of(), Type::Stream);

        let streams_ref = block_on(CORE.streams.read());

        match streams_ref.get(&Stream::stream_id(tag).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                StreamWriter::write_str(
                    env,
                    format!(
                        "#<stream: {} {} {} {}>",
                        stream.id,
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
                        if Stream::is_open(tag) {
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

    pub fn stream_id(stream: Tag) -> exception::Result<u64> {
        match stream {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::Ext => match dtag.ext().try_into() {
                    Ok(ExtType::Stream) => Ok(dtag.data()),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn view(env: &Env, stream: Tag) -> Tag {
        let streams_ref = block_on(CORE.streams.read());

        match streams_ref.get(&Self::stream_id(stream).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());
                let vec = vec![
                    Fixnum::with_or_panic(stream.id as usize),
                    stream.direction,
                    stream.unch,
                ];

                Vector::from(vec).with_heap(env)
            }
            None => panic!(),
        }
    }

    pub fn is_open(stream: Tag) -> bool {
        let streams_ref = block_on(CORE.streams.read());

        match streams_ref.get(&Self::stream_id(stream).unwrap()) {
            Some(stream_ref) => block_on(stream_ref.read()).open,
            None => panic!(),
        }
    }

    pub fn close(stream: Tag) {
        let streams_ref = block_on(CORE.streams.read());

        match streams_ref.get(&Self::stream_id(stream).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                SystemStream::close(&stream.system);
            }
            None => panic!(),
        }
    }

    pub fn get_string(env: &Env, stream: Tag) -> exception::Result<String> {
        if !Self::is_open(stream) {
            Err(Exception::new(
                env,
                Condition::Open,
                "mu:get-string",
                stream,
            ))?
        }

        let streams_ref = block_on(CORE.streams.read());

        match streams_ref.get(&Self::stream_id(stream).unwrap()) {
            Some(stream_ref) => {
                let stream = block_on(stream_ref.read());

                Ok(SystemStream::get_string(&stream.system).unwrap())
            }
            None => panic!(),
        }
    }
}

pub trait CoreFn {
    fn mu_close(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_flush(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_get_string(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_open(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_openp(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_read(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_read_byte(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_read_char(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_unread_char(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_write(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_write_byte(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_write_char(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Stream {
    fn mu_write(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:write", &[Type::T, Type::T, Type::Stream], fp)?;

        fp.value = fp.argv[0];
        let escape = fp.argv[1];
        let stream = fp.argv[2];

        env.write(fp.value, !escape.null_(), stream)?;

        Ok(())
    }

    fn mu_read(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:read", &[Type::Stream], fp)?;

        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        fp.value = env.read(stream, !eof_error_p.null_(), eof_value, false)?;

        Ok(())
    }

    fn mu_close(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:close", &[Type::Stream], fp)?;

        let stream = fp.argv[0];

        fp.value = if Stream::is_open(stream) {
            Stream::close(stream);
            Symbol::keyword("t")
        } else {
            Tag::nil()
        };

        Ok(())
    }

    fn mu_openp(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:openp", &[Type::Stream], fp)?;

        let stream = fp.argv[0];

        fp.value = if Stream::is_open(stream) {
            stream
        } else {
            Tag::nil()
        };

        Ok(())
    }

    fn mu_open(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check(
            "mu:open",
            &[Type::Keyword, Type::Keyword, Type::String, Type::T],
            fp,
        )?;

        let st_type = fp.argv[0];
        let st_dir = fp.argv[1];
        let st_arg = fp.argv[2];
        let st_error_p = fp.argv[3];

        fp.value = if st_type.eq_(&Symbol::keyword("file")) {
            let arg = Vector::as_string(env, st_arg);

            let stream = if st_dir.eq_(&Symbol::keyword("input")) {
                StreamBuilder::new().file(arg).input().build(env, &CORE)
            } else if st_dir.eq_(&Symbol::keyword("output")) {
                StreamBuilder::new().file(arg).output().build(env, &CORE)
            } else if st_error_p.null_() {
                Ok(Tag::nil())
            } else {
                Err(Exception::new(env, Condition::Type, "mu:open", st_dir))?
            };

            stream?
        } else if st_type.eq_(&Symbol::keyword("string")) {
            let arg = Vector::as_string(env, st_arg);

            let stream = if st_dir.eq_(&Symbol::keyword("input")) {
                StreamBuilder::new().string(arg).input().build(env, &CORE)
            } else if st_dir.eq_(&Symbol::keyword("output")) {
                StreamBuilder::new().string(arg).output().build(env, &CORE)
            } else if st_dir.eq_(&Symbol::keyword("bidir")) {
                StreamBuilder::new().string(arg).bidir().build(env, &CORE)
            } else if st_error_p.null_() {
                Ok(Tag::nil())
            } else {
                Err(Exception::new(env, Condition::Type, "mu:open", st_dir))?
            };

            stream?
        } else {
            Err(Exception::new(env, Condition::Type, "mu:open", st_type))?
        };

        Ok(())
    }

    fn mu_flush(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:flush", &[Type::Stream], fp)?;

        let stream_tag = fp.argv[0];
        let streams_ref = block_on(CORE.streams.read());

        fp.value = match streams_ref.get(&Stream::stream_id(stream_tag)?) {
            Some(stream_ref) => {
                if Stream::is_open(stream_tag) {
                    let stream = block_on(stream_ref.read());

                    if stream.direction.eq_(&Symbol::keyword("output")) {
                        SystemStream::flush(&stream.system).unwrap()
                    }

                    stream_tag
                } else {
                    Tag::nil()
                }
            }
            None => panic!(),
        };

        Ok(())
    }

    fn mu_get_string(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:get-string", &[Type::Stream], fp)?;

        let stream = fp.argv[0];
        let string = Stream::get_string(env, stream)?;

        fp.value = Vector::from(string).with_heap(env);

        Ok(())
    }

    fn mu_read_char(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:read-char", &[Type::Stream, Type::T, Type::T], fp)?;

        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        fp.value = match StreamReader::read_char(env, stream)? {
            Some(ch) => ch.into(),
            None if eof_error_p.null_() => eof_value,
            None => return Err(Exception::new(env, Condition::Eof, "mu:read-char", stream)),
        };

        Ok(())
    }

    fn mu_read_byte(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:read-byte", &[Type::Stream, Type::T, Type::T], fp)?;

        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        fp.value = match StreamReader::read_byte(env, stream)? {
            Some(byte) => byte.into(),
            None if eof_error_p.null_() => eof_value,
            None => return Err(Exception::new(env, Condition::Eof, "mu:read-byte", stream)),
        };

        Ok(())
    }

    fn mu_unread_char(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:unread-char", &[Type::Char, Type::Stream], fp)?;

        let ch = fp.argv[0];
        let stream = fp.argv[1];

        fp.value = match StreamReader::unread_char(env, stream, Char::as_char(env, ch))? {
            Some(_) => panic!(),
            None => ch,
        };

        Ok(())
    }

    fn mu_write_char(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:write-char", &[Type::Char, Type::Stream], fp)?;

        let ch = fp.argv[0];
        let stream = fp.argv[1];

        StreamWriter::write_char(env, stream, Char::as_char(env, ch))?;
        fp.value = ch;

        Ok(())
    }

    fn mu_write_byte(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:write-byte", &[Type::Byte, Type::Stream], fp)?;

        let byte = fp.argv[0];
        let stream = fp.argv[1];

        StreamWriter::write_byte(env, stream, Fixnum::as_i64(byte) as u8)?;
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
