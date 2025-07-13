//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! timer interface
#![allow(dead_code)]
#![allow(unused_imports)]
use crate::{
    features::feature::Feature,
    mu::{
        apply::Apply,
        config::Config,
        core::CoreFnDef,
        direct::DirectTag,
        env::Env,
        exception,
        frame::Frame,
        heap::{HeapImageInfo, HeapTypeInfo},
        types::{Tag, Type},
    },
    types::{
        cons::Cons, fixnum::Fixnum, function::Function, struct_::Struct, symbol::Symbol,
        vector::Vector,
    },
};

use {
    memmap,
    std::{
        collections::HashMap,
        fs::{remove_file, OpenOptions},
        io::{Seek, SeekFrom, Write},
        sync::mpsc::channel,
    },
};

use {futures_lite::future::block_on, futures_locks::RwLock};

lazy_static! {
    pub static ref TIMER_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref TIMER_FUNCTIONS: Vec<CoreFnDef> = vec![("delay", 1u16, Feature::timer_delay),];
}

impl Timer for Feature {
    fn feature() -> Feature {
        Feature {
            functions: Some(&TIMER_FUNCTIONS),
            symbols: Some(&TIMER_SYMBOLS),
            namespace: "timer".into(),
        }
    }
}

pub trait Timer {
    fn feature() -> Feature;
}

pub trait CoreFunction {
    fn timer_delay(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn timer_delay(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.fp_argv_check("timer:delay", &[Type::Fixnum], fp)?;

        let delay = Fixnum::as_i64(fp.argv[0]);

        fp.value = Tag::nil();

        let timer = timer::Timer::new();
        let (tx, rx) = channel();

        timer.schedule_with_delay(chrono::Duration::microseconds(delay), move || {
            tx.send(()).unwrap();
        });

        let _ = rx.recv();

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
