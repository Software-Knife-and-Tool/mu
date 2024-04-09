//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! stream operators
use {
    crate::{
        core::{
            direct::{DirectInfo, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            types::Tag,
        },
        streams::system::{StringDirection, SystemStream, SystemStreamBuilder},
        types::{
            stream::Stream,
            symbol::{Core as _, Symbol},
        },
    },
    std::{io::Write, str},
};

use futures::executor::block_on;
use futures_locks::RwLock;

pub trait Core {
    fn close(_: &SystemStream) -> Option<()>;
    fn flush(_: &SystemStream) -> Option<()>;

    fn is_file(_: &SystemStream) -> Option<bool>;
    fn is_string(_: &SystemStream) -> Option<bool>;

    fn open_file(_: &Env, _: &str, _: bool) -> exception::Result<Tag>;
    fn open_input_file(_: &Env, _: &str) -> exception::Result<Tag>;
    fn open_output_file(_: &Env, _: &str) -> exception::Result<Tag>;
    fn open_string(_: &Env, _: &str, _: StringDirection) -> exception::Result<Tag>;
    fn open_input_string(_: &Env, _: &str) -> exception::Result<Tag>;
    fn open_output_string(_: &Env, _: &str) -> exception::Result<Tag>;
    fn open_bidir_string(_: &Env, _: &str) -> exception::Result<Tag>;
    fn open_std_stream(_: &Env, _: SystemStream) -> exception::Result<Tag>;

    fn get_string(_: &SystemStream) -> Option<String>;
}

impl Core for SystemStream {
    fn is_file(stream: &SystemStream) -> Option<bool> {
        match stream {
            SystemStream::File(_) => Some(true),
            _ => Some(false),
        }
    }

    fn is_string(stream: &SystemStream) -> Option<bool> {
        match stream {
            SystemStream::String(_) => Some(true),
            _ => Some(false),
        }
    }

    fn flush(stream: &SystemStream) -> Option<()> {
        match stream {
            Self::StdOutput => {
                std::io::stdout().flush().unwrap();
            }
            Self::StdError => {
                std::io::stderr().flush().unwrap();
            }
            _ => (),
        };

        Some(())
    }

    fn close(stream: &SystemStream) -> Option<()> {
        match stream {
            Self::StdInput | Self::StdOutput | Self::StdError => (),
            Self::File(file) => {
                std::mem::drop(block_on(file.read()));
            }
            SystemStream::String(_) => (),
        };

        Some(())
    }

    fn open_file(env: &Env, path: &str, is_input: bool) -> exception::Result<Tag> {
        let system_stream = if is_input {
            SystemStreamBuilder::new()
                .file(path.to_string())
                .input()
                .build()
        } else {
            SystemStreamBuilder::new()
                .file(path.to_string())
                .output()
                .build()
        };

        match system_stream {
            None => Err(Exception::new(Condition::Open, "env:open", Tag::nil())),
            Some(_) => {
                let mut streams_ref = block_on(env.streams.write());
                let index = streams_ref.len();

                streams_ref.push(RwLock::new(Stream {
                    index,
                    system: system_stream.unwrap(),
                    open: true,
                    direction: Symbol::keyword(if is_input { "input" } else { "output" }),
                    unch: Tag::nil(),
                }));

                Ok(DirectTag::to_direct(
                    index as u64,
                    DirectInfo::ExtType(ExtType::Stream),
                    DirectType::Ext,
                ))
            }
        }
    }

    fn open_input_file(env: &Env, path: &str) -> exception::Result<Tag> {
        Self::open_file(env, path, true)
    }

    fn open_output_file(env: &Env, path: &str) -> exception::Result<Tag> {
        Self::open_file(env, path, false)
    }

    fn open_string(env: &Env, contents: &str, dir: StringDirection) -> exception::Result<Tag> {
        let system_stream = match dir {
            StringDirection::Input => SystemStreamBuilder::new()
                .string(contents.to_string())
                .input()
                .build(),
            StringDirection::Output => SystemStreamBuilder::new()
                .string(contents.to_string())
                .output()
                .build(),
            StringDirection::Bidir => SystemStreamBuilder::new()
                .string(contents.to_string())
                .bidir()
                .build(),
        };

        match system_stream {
            None => Err(Exception::new(Condition::Open, "env:open", Tag::nil())),
            Some(_) => {
                let mut streams_ref = block_on(env.streams.write());
                let index = streams_ref.len();

                streams_ref.push(RwLock::new(Stream {
                    index,
                    open: true,
                    system: system_stream.unwrap(),
                    direction: match dir {
                        StringDirection::Input => Symbol::keyword("input"),
                        StringDirection::Output => Symbol::keyword("output"),
                        StringDirection::Bidir => Symbol::keyword("bidir"),
                    },
                    unch: Tag::nil(),
                }));

                Ok(DirectTag::to_direct(
                    index as u64,
                    DirectInfo::ExtType(ExtType::Stream),
                    DirectType::Ext,
                ))
            }
        }
    }

    fn open_input_string(env: &Env, path: &str) -> exception::Result<Tag> {
        Self::open_string(env, path, StringDirection::Input)
    }

    fn open_output_string(env: &Env, path: &str) -> exception::Result<Tag> {
        Self::open_string(env, path, StringDirection::Output)
    }

    fn open_bidir_string(env: &Env, path: &str) -> exception::Result<Tag> {
        Self::open_string(env, path, StringDirection::Bidir)
    }

    fn open_std_stream(env: &Env, std_stream: SystemStream) -> exception::Result<Tag> {
        match std_stream {
            SystemStream::StdInput | SystemStream::StdOutput | SystemStream::StdError => {
                let mut streams_ref = block_on(env.streams.write());
                let index = streams_ref.len();

                streams_ref.push(RwLock::new(Stream {
                    index,
                    open: true,
                    direction: match &std_stream {
                        SystemStream::StdInput => Symbol::keyword("input"),
                        SystemStream::StdOutput | SystemStream::StdError => {
                            Symbol::keyword("output")
                        }
                        _ => panic!(),
                    },
                    system: std_stream,
                    unch: Tag::nil(),
                }));

                Ok(DirectTag::to_direct(
                    index as u64,
                    DirectInfo::ExtType(ExtType::Stream),
                    DirectType::Ext,
                ))
            }
            _ => panic!(),
        }
    }

    fn get_string(stream: &SystemStream) -> Option<String> {
        match stream {
            Self::StdInput | Self::StdOutput | Self::StdError => None,
            SystemStream::File(_) => None,
            SystemStream::String(string) => {
                let mut string_ref = block_on(string.write());
                let string_vec: Vec<u8> = string_ref.iter().cloned().collect();

                string_ref.clear();
                Some(str::from_utf8(&string_vec).unwrap().to_owned())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn stream() {
        assert_eq!(true, true)
    }
}
