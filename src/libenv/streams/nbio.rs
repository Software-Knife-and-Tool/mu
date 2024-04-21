//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! non-blocking I/O
use async_std::{
    io::{self, *},
    task,
};

fn read_stdin() -> io::Result<u8> {
    let task: io::Result<u8> = task::block_on(async {
        let mut buf = [0; 1];

        match io::stdin().read(&mut buf).await {
            Ok(_) => Ok(buf[0]),
            Err(e) => Err(e),
        }
    });

    task
}

fn write_stdout(ch: u8) -> io::Result<u8> {
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

fn write_stderr(ch: u8) -> io::Result<u8> {
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
