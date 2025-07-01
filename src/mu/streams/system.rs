//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! system streams
use futures_lite::AsyncReadExt;
use futures_lite::AsyncWriteExt;
use {
    crate::mu::{
        env::Env,
        exception::{self, Condition, Exception},
        types::Tag,
    },
    smol::{
        fs,
        io::{BufReader, BufWriter},
    },
    std::collections::VecDeque,
    std::io,
    std::io::Read,
    std::io::Write,
};
use {futures_lite::future::block_on, futures_locks::RwLock};

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
                Some(_) => {
                    let task: Option<SystemStream> = block_on(async {
                        match fs::File::open(path).await {
                            Ok(file) => {
                                Some(SystemStream::Reader(RwLock::new(BufReader::new(file))))
                            }
                            Err(_) => None,
                        }
                    });

                    task
                }
                None => match self.output {
                    Some(_) => {
                        let task: Option<SystemStream> = block_on(async {
                            match fs::File::create(path).await {
                                Ok(file) => {
                                    Some(SystemStream::Writer(RwLock::new(BufWriter::new(file))))
                                }
                                Err(_) => None,
                            }
                        });

                        task
                    }
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
    Reader(RwLock<BufReader<fs::File>>),
    Writer(RwLock<BufWriter<fs::File>>),
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

impl SystemStream {
    pub fn read_byte(&self, env: &Env) -> exception::Result<Option<u8>> {
        let mut buf = [0; 1];

        match self {
            Self::StdInput => {
                let task: io::Result<usize> = block_on(async { io::stdin().read(&mut buf) });

                match task {
                    Ok(nread) => {
                        if nread == 0 {
                            Ok(None)
                        } else {
                            Ok(Some(buf[0]))
                        }
                    }
                    Err(_) => Err(Exception::new(
                        env,
                        Condition::Read,
                        "mu:read-byte",
                        Tag::nil(),
                    )),
                }
            }
            Self::Reader(file) => {
                let mut file_ref = block_on(file.write());
                let task: io::Result<usize> = block_on(file_ref.read(&mut buf));

                match task {
                    Ok(nread) => {
                        if nread == 0 {
                            Ok(None)
                        } else {
                            Ok(Some(buf[0]))
                        }
                    }
                    Err(_) => Err(Exception::new(
                        env,
                        Condition::Read,
                        "mu:read-byte",
                        Tag::nil(),
                    )),
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

    pub fn write_byte(&self, env: &Env, byte: u8) -> exception::Result<Option<()>> {
        let buf = [byte; 1];

        match self {
            Self::StdOutput => {
                let task: io::Result<usize> = block_on(async { io::stdout().write(&buf) });

                match task {
                    Ok(_) => Ok(None),
                    Err(_) => Err(Exception::new(
                        env,
                        Condition::Write,
                        "mu:write-byte",
                        Tag::nil(),
                    )),
                }
            }
            Self::StdError => {
                let task: io::Result<usize> = block_on(async { io::stderr().write(&buf) });

                match task {
                    Ok(_) => Ok(None),
                    Err(_) => Err(Exception::new(
                        env,
                        Condition::Write,
                        "mu:write-byte",
                        Tag::nil(),
                    )),
                }
            }
            SystemStream::Writer(file) => {
                let mut file_ref = block_on(file.write());
                let task: io::Result<()> = block_on(file_ref.write_all(&buf));

                match task {
                    Ok(_) => Ok(None),
                    Err(_) => Err(Exception::new(
                        env,
                        Condition::Write,
                        "mu:write-byte",
                        Tag::nil(),
                    )),
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
