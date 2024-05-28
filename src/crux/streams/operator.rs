//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! stream operators
#![allow(unused_imports)]
use {
    crate::{
        core::{
            direct::{DirectInfo, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            lib::{Lib, LIB},
            types::Tag,
        },
        streams::system::{StringDirection, SystemStream, SystemStreamBuilder},
        types::{
            core_stream::Stream,
            symbol::{Core as _, Symbol},
        },
    },
    async_std::{
        fs,
        io::{self, BufReader, BufWriter, ReadExt, WriteExt},
        task,
    },
    std::{io::Write, str},
};

use futures::executor::block_on;
use futures_locks::RwLock;

pub trait Core {
    fn open_file(_: &Env, _: &str, _: bool) -> exception::Result<Tag>;
    fn open_input_file(_: &Env, _: &str) -> exception::Result<Tag>;
    fn open_output_file(_: &Env, _: &str) -> exception::Result<Tag>;
    fn open_string(_: &Env, _: &str, _: StringDirection) -> exception::Result<Tag>;
    fn open_input_string(_: &Env, _: &str) -> exception::Result<Tag>;
    fn open_output_string(_: &Env, _: &str) -> exception::Result<Tag>;
    fn open_bidir_string(_: &Env, _: &str) -> exception::Result<Tag>;
    fn open_std_stream(_: SystemStream, _: &Lib) -> exception::Result<Tag>;
}

impl SystemStream {
    pub fn is_file(&self) -> Option<bool> {
        match self {
            SystemStream::Reader(_) | SystemStream::Writer(_) => Some(true),
            _ => Some(false),
        }
    }

    pub fn is_string(&self) -> Option<bool> {
        match self {
            SystemStream::String(_) => Some(true),
            _ => Some(false),
        }
    }

    pub fn flush(&self) -> Option<()> {
        match self {
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

    pub fn close(&self) -> Option<()> {
        match self {
            Self::StdInput | Self::StdOutput | Self::StdError => (),
            Self::Reader(file) => drop(block_on(file.read())),
            Self::Writer(file) => {
                let mut file = block_on(file.write());
                let _unused = task::block_on(async { file.flush().await });

                drop(file)
            }
            SystemStream::String(_) => (),
        };

        Some(())
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::StdInput | Self::StdOutput | Self::StdError => None,
            SystemStream::Reader(_) | SystemStream::Writer(_) => None,
            SystemStream::String(string) => {
                let mut string_ref = block_on(string.write());
                let string_vec: Vec<u8> = string_ref.iter().cloned().collect();

                string_ref.clear();
                Some(str::from_utf8(&string_vec).unwrap().to_owned())
            }
        }
    }
}

impl Core for SystemStream {
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
            None => Err(Exception::new(
                env,
                Condition::Open,
                "crux:open",
                Tag::nil(),
            )),
            Some(_) => {
                let mut streams_ref = block_on(LIB.streams.write());
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
            None => Err(Exception::new(env, Condition::Open, "env:open", Tag::nil())),
            Some(_) => {
                let mut streams_ref = block_on(LIB.streams.write());
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

    fn open_std_stream(std_stream: SystemStream, core: &Lib) -> exception::Result<Tag> {
        match std_stream {
            SystemStream::StdInput | SystemStream::StdOutput | SystemStream::StdError => {
                let mut streams_ref = block_on(core.streams.write());
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn stream() {
        assert_eq!(true, true)
    }
}
