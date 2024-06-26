//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env stream functions
#![allow(unused_braces)]
#![allow(dead_code)]
#![allow(clippy::identity_op)]

use crate::{
    core::{
        apply::Core as _,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        lib::{Lib, LIB},
        types::{Tag, Type},
    },
    streams::{operator::Core as _, system::SystemStream},
    types::{
        char::Char,
        core_stream::{Core as _, Stream},
        fixnum::Fixnum,
        symbol::{Core as _, Symbol},
        vector::{Core as _, Vector},
        vector_image::Core as _,
    },
};

use futures::executor::block_on;

pub struct StreamBuilder {
    pub file: Option<String>,
    pub string: Option<String>,
    pub input: Option<Tag>,
    pub output: Option<Tag>,
    pub bidir: Option<Tag>,
    pub stdin: Option<()>,
    pub stdout: Option<()>,
    pub errout: Option<()>,
}

impl StreamBuilder {
    pub fn new() -> Self {
        Self {
            file: None,
            string: None,
            input: None,
            output: None,
            bidir: None,
            stdin: None,
            stdout: None,
            errout: None,
        }
    }

    pub fn file(&mut self, path: String) -> &mut Self {
        self.file = Some(path);
        self
    }

    pub fn string(&mut self, contents: String) -> &mut Self {
        self.string = Some(contents);
        self
    }

    pub fn input(&mut self) -> &mut Self {
        self.input = Some(Symbol::keyword("input"));
        self
    }

    pub fn output(&mut self) -> &mut Self {
        self.output = Some(Symbol::keyword("output"));
        self
    }

    pub fn bidir(&mut self) -> &mut Self {
        self.bidir = Some(Symbol::keyword("bidir"));
        self
    }

    pub fn stdin(&mut self) -> &mut Self {
        self.stdin = Some(());
        self
    }

    pub fn stdout(&mut self) -> &mut Self {
        self.stdout = Some(());
        self
    }

    pub fn errout(&mut self) -> &mut Self {
        self.errout = Some(());
        self
    }

    pub fn std_build(&self, core: &Lib) -> exception::Result<Tag> {
        match self.stdin {
            Some(_) => SystemStream::open_std_stream(SystemStream::StdInput, core),
            None => match self.stdout {
                Some(_) => SystemStream::open_std_stream(SystemStream::StdOutput, core),
                None => match self.errout {
                    Some(_) => SystemStream::open_std_stream(SystemStream::StdError, core),
                    None => panic!(),
                },
            },
        }
    }

    pub fn build(&self, env: &Env, core: &Lib) -> exception::Result<Tag> {
        match &self.file {
            Some(path) => match self.input {
                Some(_) => SystemStream::open_input_file(env, path),
                None => SystemStream::open_output_file(env, path),
            },
            None => match &self.string {
                Some(contents) => match self.input {
                    Some(_) => SystemStream::open_input_string(env, contents),
                    None => match self.output {
                        Some(_) => SystemStream::open_output_string(env, contents),
                        None => match self.bidir {
                            Some(_) => SystemStream::open_bidir_string(env, contents),
                            None => {
                                Err(Exception::new(env, Condition::Range, "mu:open", Tag::nil()))
                            }
                        },
                    },
                },
                None => match self.stdin {
                    Some(_) => SystemStream::open_std_stream(SystemStream::StdInput, core),
                    None => match self.stdout {
                        Some(_) => SystemStream::open_std_stream(SystemStream::StdOutput, core),
                        None => match self.errout {
                            Some(_) => SystemStream::open_std_stream(SystemStream::StdError, core),
                            None => {
                                Err(Exception::new(env, Condition::Range, "mu:open", Tag::nil()))
                            }
                        },
                    },
                },
            },
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

        env.fp_argv_check("mu:open", &[Type::Keyword, Type::Keyword, Type::String], fp)?;

        fp.value = if st_type.eq_(&Symbol::keyword("file")) {
            let arg = Vector::as_string(env, st_arg);

            let stream = if st_dir.eq_(&Symbol::keyword("input")) {
                StreamBuilder::new().file(arg).input().build(env, &LIB)
            } else if st_dir.eq_(&Symbol::keyword("output")) {
                StreamBuilder::new().file(arg).output().build(env, &LIB)
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

        fp.value = match streams_ref.get(Self::to_stream_index(env, tag).unwrap()) {
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
            .string("hello".to_string())
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
