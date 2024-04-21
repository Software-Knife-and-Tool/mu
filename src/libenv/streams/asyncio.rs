//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! async I/O
#![allow(unused_imports)]
use crate::{
    core::{
        exception::{self, Condition, Exception},
        types::Tag,
    },
    streams::system::SystemStream,
};
use async_std::{
    io::{self, *},
    task,
};

pub trait Core {
    fn async_stdin_read(_: &mut [u8]) -> io::Result<usize>;
    fn async_write_stdout(ch: u8) -> io::Result<u8>;
    fn async_write_stderr(ch: u8) -> io::Result<u8>;
}

impl Core for SystemStream {
    fn async_stdin_read(buf: &mut [u8]) -> io::Result<usize> {
        let task: io::Result<usize> = task::block_on(async { io::stdin().read(buf).await });

        task
    }

    fn async_write_stdout(ch: u8) -> io::Result<u8> {
        let task: io::Result<u8> = task::block_on(async {
            let mut buf = [0; 1];

            buf[0] = ch;

            match io::stdout().write(&buf).await {
                Ok(_) => Ok(buf[0]),
                Err(e) => Err(e),
            }
        });

        task
    }

    fn async_write_stderr(ch: u8) -> io::Result<u8> {
        let task: io::Result<u8> = task::block_on(async {
            let mut buf = [0; 1];

            buf[0] = ch;

            match io::stderr().write(&buf).await {
                Ok(_) => Ok(buf[0]),
                Err(e) => Err(e),
            }
        });

        task
    }
}
