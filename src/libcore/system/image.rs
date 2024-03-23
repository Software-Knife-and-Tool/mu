//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! elf object files
#[allow(unused_imports)]
use crate::{
    core::{
        exception,
        mu::Mu,
        types::{Tag, Type},
    },
    system::System,
    types::{
        fixnum::Fixnum,
        vector::{Core as _, Vector},
    },
};

pub struct Image {
    path: String,
}

impl Image {
    fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    fn write(&self) -> exception::Result<()> {
        Ok(())
    }

    fn read(&self, _: String) -> exception::Result<()> {
        Ok(())
    }
}

pub trait Core {
    fn image_path(&self) -> &String;
    fn image_write(&self) -> exception::Result<()>;
    fn image_read(&self, _: String) -> exception::Result<Image>;
}

impl Core for Image {
    fn image_path(&self) -> &String {
        &self.path
    }

    fn image_write(&self) -> exception::Result<()> {
        self.write()
    }

    fn image_read(&self, _: String) -> exception::Result<Image> {
        Ok(Self {
            path: String::new(),
        })
    }
}

#[cfg(test)]
mod tests {}
