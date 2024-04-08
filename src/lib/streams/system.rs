//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! system streams
use {
    crate::core::{
        exception::{self, Condition, Exception},
        types::Tag,
    },
    std::{
        collections::VecDeque,
        fs,
        io::{Read, Write},
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
    fn read_byte(_: &SystemStream) -> exception::Result<Option<u8>>;
    fn write_byte(_: &SystemStream, _: u8) -> exception::Result<Option<()>>;
}

impl Core for SystemStream {
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
