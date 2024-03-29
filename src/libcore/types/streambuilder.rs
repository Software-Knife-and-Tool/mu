//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! system streams
use crate::{
    core::{
        exception::{self, Condition, Exception},
        mu::Mu,
        stream::{Core as _, SystemStream},
        types::Tag,
    },
    types::symbol::{Core as _, Symbol},
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

    pub fn build(&self, mu: &Mu) -> exception::Result<Tag> {
        match &self.file {
            Some(path) => match self.input {
                Some(_) => SystemStream::open_input_file(mu, path),
                None => SystemStream::open_output_file(mu, path),
            },
            None => match &self.string {
                Some(contents) => match self.input {
                    Some(_) => SystemStream::open_input_string(mu, contents),
                    None => match self.output {
                        Some(_) => SystemStream::open_output_string(mu, contents),
                        None => match self.bidir {
                            Some(_) => SystemStream::open_bidir_string(mu, contents),
                            None => Err(Exception::new(Condition::Range, "open", Tag::nil())),
                        },
                    },
                },
                None => match self.stdin {
                    Some(_) => SystemStream::open_std_stream(mu, SystemStream::StdInput),
                    None => match self.stdout {
                        Some(_) => SystemStream::open_std_stream(mu, SystemStream::StdOutput),
                        None => match self.errout {
                            Some(_) => SystemStream::open_std_stream(mu, SystemStream::StdError),
                            None => Err(Exception::new(Condition::Range, "open", Tag::nil())),
                        },
                    },
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::stream::{SystemStream, SystemStreamBuilder};

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
