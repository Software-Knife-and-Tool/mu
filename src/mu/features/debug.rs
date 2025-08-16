//  SPDX-FileCopyrightText: Copyright 2024\5 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// debug feature
#![allow(dead_code)]
use {
    crate::{
        core::{core::CORE, env::Env, types::Tag, writer::Writer},
        features::feature::Feature,
    },
    futures_lite::future::block_on,
};

pub trait Debug {
    fn feature() -> Feature;
    fn eprint(_: &Env, label: &str, verbose: bool, tag: Tag);
    fn eprintln(_: &Env, label: &str, verbose: bool, tag: Tag);
    fn print(_: &Env, label: &str, verbose: bool, tag: Tag);
    fn println(_: &Env, label: &str, verbose: bool, tag: Tag);
}

impl Debug for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: None,
            functions: None,
            namespace: "mu/debug".into(),
        }
    }

    fn eprint(env: &Env, label: &str, verbose: bool, tag: Tag) {
        let stdio = block_on(CORE.stdio.write());

        eprint!("{label}: ");
        env.write(tag, verbose, stdio.2).unwrap();
    }

    fn eprintln(env: &Env, label: &str, verbose: bool, tag: Tag) {
        let stdio = block_on(CORE.stdio.write());

        eprint!("{label}: ");
        env.write(tag, verbose, stdio.2).unwrap();
        eprintln!();
    }

    fn print(env: &Env, label: &str, verbose: bool, tag: Tag) {
        let stdio = block_on(CORE.stdio.write());

        print!("{label}: ");
        env.write(tag, verbose, stdio.1).unwrap();
    }

    fn println(env: &Env, label: &str, verbose: bool, tag: Tag) {
        let stdio = block_on(CORE.stdio.write());

        print!("{label}: ");
        env.write(tag, verbose, stdio.1).unwrap();
        println!();
    }
}
