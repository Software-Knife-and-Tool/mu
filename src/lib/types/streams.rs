//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu stream functions
#![allow(unused_braces)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::identity_op)]

use crate::{
    core::{
        apply::Core as _,
        exception::{self, Condition, Exception},
        frame::Frame,
        mu::Mu,
        types::{Tag, Type},
    },
    streams::{operator::Core as _, system::SystemStream},
    types::{
        char::Char,
        fixnum::Fixnum,
        stream::{Core as _, Stream},
        streambuilder::StreamBuilder,
        symbol::{Core as _, Symbol},
        vector::{Core as _, Vector},
    },
};

use futures::executor::block_on;

pub trait MuFunction {
    fn lib_close(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_flush(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_get_string(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_open(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_openp(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_read_byte(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_read_char(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_unread_char(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_write_byte(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_write_char(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Stream {
    fn lib_close(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];

        fp.value = match mu.fp_argv_check("close", &[Type::Stream], fp) {
            Ok(_) => {
                if Self::is_open(mu, stream) {
                    Self::close(mu, stream);
                    Symbol::keyword("t")
                } else {
                    Tag::nil()
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_openp(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];

        fp.value = match mu.fp_argv_check("openp", &[Type::Stream], fp) {
            Ok(_) => {
                if Self::is_open(mu, stream) {
                    stream
                } else {
                    Tag::nil()
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_open(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let st_type = fp.argv[0];
        let st_dir = fp.argv[1];
        let st_arg = fp.argv[2];

        fp.value = match mu.fp_argv_check("open", &[Type::Keyword, Type::Keyword, Type::String], fp)
        {
            Ok(_) if st_type.eq_(&Symbol::keyword("file")) => {
                let arg = Vector::as_string(mu, st_arg);

                let stream = if st_dir.eq_(&Symbol::keyword("input")) {
                    StreamBuilder::new().file(arg).input().build(mu)
                } else if st_dir.eq_(&Symbol::keyword("output")) {
                    StreamBuilder::new().file(arg).output().build(mu)
                } else {
                    return Err(Exception::new(Condition::Type, "open", st_dir));
                };

                match stream {
                    Err(e) => return Err(e),
                    Ok(stream) => stream,
                }
            }
            Ok(_) if st_type.eq_(&Symbol::keyword("string")) => {
                let arg = Vector::as_string(mu, st_arg);

                let stream = if st_dir.eq_(&Symbol::keyword("input")) {
                    StreamBuilder::new().string(arg).input().build(mu)
                } else if st_dir.eq_(&Symbol::keyword("output")) {
                    StreamBuilder::new().string(arg).output().build(mu)
                } else if st_dir.eq_(&Symbol::keyword("bidir")) {
                    StreamBuilder::new().string(arg).bidir().build(mu)
                } else {
                    return Err(Exception::new(Condition::Type, "open", st_dir));
                };

                match stream {
                    Err(e) => return Err(e),
                    Ok(stream) => stream,
                }
            }
            Ok(_) => return Err(Exception::new(Condition::Type, "open", st_type)),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_flush(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        fp.value = match mu.fp_argv_check("flush", &[Type::Stream], fp) {
            Ok(_) => {
                if Self::is_open(mu, tag) {
                    let streams_ref = block_on(mu.streams.read());

                    match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
                        Some(stream_ref) => {
                            let stream = block_on(stream_ref.read());

                            if stream.direction.eq_(&Symbol::keyword("output")) {
                                SystemStream::flush(&stream.system).unwrap()
                            }
                        }
                        None => panic!(),
                    }
                }

                Tag::nil()
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_get_string(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];

        fp.value = match mu.fp_argv_check("get-str", &[Type::Stream], fp) {
            Ok(_) => match Self::get_string(mu, stream) {
                Ok(string) => Vector::from_string(&string).evict(mu),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_read_char(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        fp.value = match mu.fp_argv_check("rd-char", &[Type::Stream, Type::T, Type::T], fp) {
            Ok(_) => match Self::read_char(mu, stream) {
                Ok(Some(ch)) => Tag::from(ch),
                Ok(None) if eof_error_p.null_() => eof_value,
                Ok(None) => return Err(Exception::new(Condition::Eof, "rd-char", stream)),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_read_byte(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        fp.value = match mu.fp_argv_check("rd-byte", &[Type::Stream, Type::T, Type::T], fp) {
            Ok(_) => match Self::read_byte(mu, stream) {
                Ok(Some(byte)) => Tag::from(byte as i64),
                Ok(None) if eof_error_p.null_() => eof_value,
                Ok(None) => return Err(Exception::new(Condition::Eof, "rd-byte", stream)),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_unread_char(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let ch = fp.argv[0];
        let stream = fp.argv[1];

        fp.value = match mu.fp_argv_check("un-char", &[Type::Char, Type::Stream], fp) {
            Ok(_) => match Self::unread_char(mu, stream, Char::as_char(mu, ch)) {
                Ok(Some(_)) => panic!(),
                Ok(None) => ch,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_write_char(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let ch = fp.argv[0];
        let stream = fp.argv[1];

        fp.value = match mu.fp_argv_check("wr-char", &[Type::Char, Type::Stream], fp) {
            Ok(_) => match Self::write_char(mu, stream, Char::as_char(mu, ch)) {
                Ok(_) => ch,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_write_byte(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let byte = fp.argv[0];
        let stream = fp.argv[1];

        fp.value = match mu.fp_argv_check("wr-byte", &[Type::Byte, Type::Stream], fp) {
            Ok(_) => match Self::write_byte(mu, stream, Fixnum::as_i64(byte) as u8) {
                Ok(_) => byte,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
