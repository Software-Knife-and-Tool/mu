//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

// instrumentation
#[allow(unused_imports)]
use {
    crate::{
        core::{
            apply::Apply as _,
            core_::CORE,
            dynamic::Dynamic,
            env::Env,
            exception::{self, Condition, Exception},
            tag::Tag,
            type_::Type,
        },
        streams::writer::StreamWriter,
        types::{
            async_::Async, cons::Cons, fixnum::Fixnum, function::Function, struct_::Struct,
            symbol::Symbol, vector::Vector,
        },
    },
    //    log::{info, trace, warn},
    futures_lite::future::block_on,
};

// minimal stdout/stderr logging
pub trait Instrument {
    fn eprint(&self, _: &str, _: bool, _: Tag);
    fn eprintln(&self, _: &str, _: bool, _: Tag);
    fn print(&self, _: &str, _: bool, _: Tag);
    fn println(&self, _: &str, _: bool, _: Tag);
}

impl Instrument for Env {
    fn eprint(&self, label: &str, verbose: bool, tag: Tag) {
        eprint!("{label}: ");
        StreamWriter::write(self, tag, verbose, CORE.stdio.2).unwrap();
    }

    fn eprintln(&self, label: &str, verbose: bool, tag: Tag) {
        eprint!("{label}: ");
        StreamWriter::write(self, tag, verbose, CORE.stdio.2).unwrap();
        eprintln!();
    }

    fn print(&self, label: &str, verbose: bool, tag: Tag) {
        print!("{label}: ");
        StreamWriter::write(self, tag, verbose, CORE.stdio.1).unwrap();
    }

    fn println(&self, label: &str, verbose: bool, tag: Tag) {
        print!("{label}: ");
        StreamWriter::write(self, tag, verbose, CORE.stdio.1).unwrap();
        println!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn instrument() {
        assert!(true);
    }
}
