//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// system stream operators
use {
    crate::{
        core::{
            core_::{Core, CORE},
            direct::{DirectExt, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            tag::Tag,
        },
        streams::system::{StringDirection, SystemStream, SystemStreamBuilder},
        types::{stream::Stream, symbol::Symbol},
    },
    futures_lite::{future::block_on, AsyncWriteExt},
    futures_locks::RwLock,
    std::{io::Write, str},
};

impl SystemStream {
    pub fn is_file(&self) -> bool {
        matches!(self, SystemStream::Reader(_) | SystemStream::Writer(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, SystemStream::String(_))
    }

    pub fn flush(&self) {
        match self {
            Self::StdOutput => {
                std::io::stdout().flush().unwrap();
            }
            Self::StdError => {
                std::io::stderr().flush().unwrap();
            }
            _ => (),
        }
    }

    pub fn close(&self) {
        match self {
            Self::StdInput | Self::StdOutput | Self::StdError | Self::String(_) => (),
            Self::Reader(file) => drop(block_on(file.read())),
            Self::Writer(file) => {
                let mut file = block_on(file.write());
                let _ = block_on(async { file.flush().await });

                drop(file);
            }
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::StdInput
            | Self::StdOutput
            | Self::StdError
            | Self::Reader(_)
            | Self::Writer(_) => None,
            Self::String(string) => {
                let mut string_ref = block_on(string.write());
                let string_vec: Vec<u8> = string_ref.iter().copied().collect();

                string_ref.clear();
                Some(str::from_utf8(&string_vec).unwrap().to_owned())
            }
        }
    }

    pub fn open_file(env: &Env, path: &str, is_input: bool) -> exception::Result<Tag> {
        let system_stream = if is_input {
            SystemStreamBuilder::new().file(path.into()).input().build()
        } else {
            SystemStreamBuilder::new()
                .file(path.into())
                .output()
                .build()
        };

        if let Some(stream) = system_stream {
            let mut streams_ref = block_on(CORE.streams.write());
            let mut id = block_on(CORE.stream_id.write());
            let stream_id = *id;

            *id += 1;

            streams_ref.insert(
                stream_id,
                RwLock::new(Stream {
                    id: stream_id,
                    system: stream,
                    open: true,
                    direction: Symbol::keyword(if is_input { "input" } else { "output" }),
                    unch: Tag::nil(),
                }),
            );

            Ok(DirectTag::to_tag(
                stream_id,
                DirectExt::ExtType(ExtType::Stream),
                DirectType::Ext,
            ))
        } else {
            Err(Exception::err(env, Tag::nil(), Condition::Open, "mu:open"))
        }
    }

    pub fn open_input_file(env: &Env, path: &str) -> exception::Result<Tag> {
        Self::open_file(env, path, true)
    }

    pub fn open_output_file(env: &Env, path: &str) -> exception::Result<Tag> {
        Self::open_file(env, path, false)
    }

    pub fn open_string(env: &Env, contents: &str, dir: &StringDirection) -> exception::Result<Tag> {
        let system_stream = match dir {
            StringDirection::Input => SystemStreamBuilder::new()
                .string(contents.into())
                .input()
                .build(),
            StringDirection::Output => SystemStreamBuilder::new()
                .string(contents.into())
                .output()
                .build(),
            StringDirection::Bidir => SystemStreamBuilder::new()
                .string(contents.into())
                .bidir()
                .build(),
        };

        if let Some(stream) = system_stream {
            let mut streams_ref = block_on(CORE.streams.write());
            let mut id = block_on(CORE.stream_id.write());
            let stream_id = *id;

            *id += 1;

            streams_ref.insert(
                stream_id,
                RwLock::new(Stream {
                    id: stream_id,
                    open: true,
                    system: stream,
                    direction: match dir {
                        StringDirection::Input => Symbol::keyword("input"),
                        StringDirection::Output => Symbol::keyword("output"),
                        StringDirection::Bidir => Symbol::keyword("bidir"),
                    },
                    unch: Tag::nil(),
                }),
            );

            Ok(DirectTag::to_tag(
                stream_id,
                DirectExt::ExtType(ExtType::Stream),
                DirectType::Ext,
            ))
        } else {
            Err(Exception::err(env, Tag::nil(), Condition::Open, "env:open"))?
        }
    }

    pub fn open_input_string(env: &Env, path: &str) -> exception::Result<Tag> {
        Self::open_string(env, path, &StringDirection::Input)
    }

    pub fn open_output_string(env: &Env, path: &str) -> exception::Result<Tag> {
        Self::open_string(env, path, &StringDirection::Output)
    }

    pub fn open_bidir_string(env: &Env, path: &str) -> exception::Result<Tag> {
        Self::open_string(env, path, &StringDirection::Bidir)
    }

    pub fn open_std_stream(std_stream: SystemStream, core: &Core) -> exception::Result<Tag> {
        match std_stream {
            SystemStream::StdInput | SystemStream::StdOutput | SystemStream::StdError => {
                let mut streams_ref = block_on(core.streams.write());
                let mut id = block_on(core.stream_id.write());
                let stream_id = *id;

                *id += 1;

                streams_ref.insert(
                    stream_id,
                    RwLock::new(Stream {
                        id: stream_id,
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
                    }),
                );

                Ok(DirectTag::to_tag(
                    stream_id,
                    DirectExt::ExtType(ExtType::Stream),
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
        assert!(true)
    }
}
