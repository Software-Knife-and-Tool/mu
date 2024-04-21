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
    fs,
    io::{self, *},
    task,
};

pub trait Core {
    fn async_file_read(_: &mut fs::File, _: &mut [u8]) -> io::Result<usize>;
    fn async_file_write(_: &mut fs::File, _: &[u8]) -> io::Result<()>;
    fn async_stderr_write(_: &[u8]) -> io::Result<usize>;
    fn async_stdin_read(_: &mut [u8]) -> io::Result<usize>;
    fn async_stdout_write(_: &[u8]) -> io::Result<usize>;
}

impl Core for SystemStream {
    fn async_file_read(file: &mut fs::File, buf: &mut [u8]) -> io::Result<usize> {
        let task: io::Result<usize> = task::block_on(async { file.read(buf).await });

        task
    }

    fn async_file_write(file: &mut fs::File, buf: &[u8]) -> io::Result<()> {
        let task: io::Result<()> = task::block_on(async { file.write_all(buf).await });

        task
    }

    fn async_stdin_read(buf: &mut [u8]) -> io::Result<usize> {
        let task: io::Result<usize> = task::block_on(async { io::stdin().read(buf).await });

        task
    }

    fn async_stdout_write(buf: &[u8]) -> io::Result<usize> {
        let task: io::Result<usize> = task::block_on(async { io::stdout().write(buf).await });

        task
    }

    fn async_stderr_write(buf: &[u8]) -> io::Result<usize> {
        let task: io::Result<usize> = task::block_on(async { io::stderr().write(buf).await });

        task
    }
}
