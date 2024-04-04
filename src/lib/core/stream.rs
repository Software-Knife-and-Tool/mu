//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! system streams
use {
    crate::{
        core::{
            direct::{DirectInfo, DirectTag, DirectType, ExtType},
            exception::{self, Condition, Exception},
            mu::Mu,
            types::Tag,
        },
        types::{
            stream::Stream,
            symbol::{Core as _, Symbol},
        },
    },
    std::{
        collections::VecDeque,
        fs,
        io::{Read, Write},
        str,
    },
};
use {futures::executor::block_on, futures_locks::RwLock};

// stream builder
pub struct SystemStreamBuilder {
    pub file: Option<String>,
    pub string: Option<String>,
    pub input: Option<()>,
    pub output: Option<()>,
    pub bidir: Option<()>,
}

impl SystemStreamBuilder {
    pub fn new() -> Self {
        Self {
            file: None,
            string: None,
            input: None,
            output: None,
            bidir: None,
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
        self.input = Some(());
        self
    }

    pub fn output(&mut self) -> &mut Self {
        self.output = Some(());
        self
    }

    pub fn bidir(&mut self) -> &mut Self {
        self.bidir = Some(());
        self
    }

    pub fn build(&self) -> Option<SystemStream> {
        match &self.file {
            Some(path) => match self.input {
                Some(_) => match fs::File::open(path) {
                    Ok(file) => Some(SystemStream::File(RwLock::new(file))),
                    Err(_) => None,
                },
                None => match self.output {
                    Some(_) => match fs::File::create(path) {
                        Ok(file) => Some(SystemStream::File(RwLock::new(file))),
                        Err(_) => None,
                    },
                    None => None,
                },
            },
            None => self.string.as_ref().map(|contents| {
                SystemStream::String(RwLock::new(VecDeque::from(contents.as_bytes().to_vec())))
            }),
        }
    }
}

// system stream
#[derive(Debug)]
pub enum SystemStream {
    File(RwLock<fs::File>),
    String(RwLock<VecDeque<u8>>),
    StdInput,
    StdOutput,
    StdError,
}

pub enum StringDirection {
    Input,
    Output,
    Bidir,
}

pub trait Core {
    fn close(_: &SystemStream) -> Option<()>;
    fn flush(_: &SystemStream) -> Option<()>;
    fn get_string(_: &SystemStream) -> Option<String>;
    fn is_file(_: &SystemStream) -> Option<bool>;
    fn is_string(_: &SystemStream) -> Option<bool>;
    fn open_file(_: &Mu, _: &str, _: bool) -> exception::Result<Tag>;
    fn open_input_file(_: &Mu, _: &str) -> exception::Result<Tag>;
    fn open_output_file(_: &Mu, _: &str) -> exception::Result<Tag>;
    fn open_string(_: &Mu, _: &str, _: StringDirection) -> exception::Result<Tag>;
    fn open_input_string(_: &Mu, _: &str) -> exception::Result<Tag>;
    fn open_output_string(_: &Mu, _: &str) -> exception::Result<Tag>;
    fn open_bidir_string(_: &Mu, _: &str) -> exception::Result<Tag>;
    fn open_std_stream(_: &Mu, _: SystemStream) -> exception::Result<Tag>;
    fn read_byte(_: &SystemStream) -> exception::Result<Option<u8>>;
    fn write_byte(_: &SystemStream, _: u8) -> exception::Result<Option<()>>;
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

    fn open_file(mu: &Mu, path: &str, is_input: bool) -> exception::Result<Tag> {
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
            None => Err(Exception::new(Condition::Open, "mu:open", Tag::nil())),
            Some(_) => {
                let mut streams_ref = block_on(mu.streams.write());
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

    fn open_input_file(mu: &Mu, path: &str) -> exception::Result<Tag> {
        Self::open_file(mu, path, true)
    }

    fn open_output_file(mu: &Mu, path: &str) -> exception::Result<Tag> {
        Self::open_file(mu, path, false)
    }

    fn open_string(mu: &Mu, contents: &str, dir: StringDirection) -> exception::Result<Tag> {
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
            None => Err(Exception::new(Condition::Open, "mu:open", Tag::nil())),
            Some(_) => {
                let mut streams_ref = block_on(mu.streams.write());
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

    fn open_input_string(mu: &Mu, path: &str) -> exception::Result<Tag> {
        Self::open_string(mu, path, StringDirection::Input)
    }

    fn open_output_string(mu: &Mu, path: &str) -> exception::Result<Tag> {
        Self::open_string(mu, path, StringDirection::Output)
    }

    fn open_bidir_string(mu: &Mu, path: &str) -> exception::Result<Tag> {
        Self::open_string(mu, path, StringDirection::Bidir)
    }

    fn open_std_stream(mu: &Mu, std_stream: SystemStream) -> exception::Result<Tag> {
        match std_stream {
            SystemStream::StdInput | SystemStream::StdOutput | SystemStream::StdError => {
                let mut streams_ref = block_on(mu.streams.write());
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

    fn read_byte(stream: &SystemStream) -> exception::Result<Option<u8>> {
        let mut buf = [0; 1];

        match stream {
            Self::StdInput => match std::io::stdin().read(&mut buf) {
                Ok(nread) => {
                    if nread == 0 {
                        Ok(None)
                    } else {
                        Ok(Some(buf[0]))
                    }
                }
                Err(_) => Err(Exception::new(Condition::Read, "rd_byte", Tag::nil())),
            },
            Self::File(file) => {
                let mut file_ref = block_on(file.write());

                match file_ref.read(&mut buf) {
                    Ok(nread) => {
                        if nread == 0 {
                            Ok(None)
                        } else {
                            Ok(Some(buf[0]))
                        }
                    }
                    Err(_) => Err(Exception::new(Condition::Read, "rd-byte", Tag::nil())),
                }
            }
            SystemStream::String(string) => {
                let mut string_ref = block_on(string.write());

                if string_ref.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(string_ref.pop_front().unwrap()))
                }
            }
            _ => panic!(),
        }
    }

    fn write_byte(stream: &SystemStream, byte: u8) -> exception::Result<Option<()>> {
        let buf = [byte; 1];

        match stream {
            Self::StdOutput => match std::io::stdout().write(&buf) {
                Ok(_) => Ok(None),
                Err(_) => Err(Exception::new(Condition::Write, "wr-byte", Tag::nil())),
            },
            Self::StdError => match std::io::stderr().write(&buf) {
                Ok(_) => Ok(None),
                Err(_) => Err(Exception::new(Condition::Write, "wr-byte", Tag::nil())),
            },
            SystemStream::File(file) => {
                let mut file_ref = block_on(file.write());

                match file_ref.write_all(&buf) {
                    Ok(_) => Ok(None),
                    Err(_) => Err(Exception::new(Condition::Write, "wr-byte", Tag::nil())),
                }
            }
            SystemStream::String(string) => {
                let mut string_ref = block_on(string.write());

                string_ref.push_back(buf[0]);
                Ok(Some(()))
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
