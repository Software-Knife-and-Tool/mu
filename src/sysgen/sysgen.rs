//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

#[rustfmt::skip]
use {
    crate::{
        crates::Crate,
        options::{Options},
        symbols::Symbols,
    },
};

pub struct Sysgen {
    pub options: Options,
    pub workspace: String,
}

impl Sysgen {
    pub const BINDINGS: &'static str = "sysgen";

    pub fn new(options: Options, workspace: String) -> Self {
        Sysgen { options, workspace }
    }

    pub fn generate(&self, crate_: &Crate) {
        crate_.gencode(&self.options).unwrap();

        Symbols::new(crate_)
            .write(&format!("{}.sysgen/{}.SYMS", crate_.sysgen, crate_.name,))
            .unwrap()
    }
}