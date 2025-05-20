//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env stream type
use crate::{
    mu::{
        core::Core,
        env::Env,
        exception::{self, Condition, Exception},
        types::Tag,
    },
    streams::system::SystemStream,
    types::symbol::Symbol,
};

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

    pub fn std_build(&self, core: &Core) -> exception::Result<Tag> {
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

    pub fn build(&self, env: &Env, core: &Core) -> exception::Result<Tag> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
