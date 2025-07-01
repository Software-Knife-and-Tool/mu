//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! procinfo interface
#![allow(unused_imports)]
use crate::{
    features::feature::Feature,
    mu::{
        core::CoreFnDef,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        types::Tag,
    },
    types::{cons::Cons, fixnum::Fixnum, struct_::Struct, symbol::Symbol, vector::Vector},
};
use futures_locks::RwLock;
use std::collections::HashMap;

use perf_monitor::{cpu::cpu_time, fd::fd_count_cur, mem::get_process_memory_info};

lazy_static! {
    pub static ref PROCINFO_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref PROCINFO_FUNCTIONS: Vec<CoreFnDef> = vec![
        ("process-fds", 0, Feature::procinfo_fds),
        ("process-mem-res", 0, Feature::procinfo_mem_res),
        ("process-mem-virt", 0, Feature::procinfo_mem_virt),
        ("process-time", 0, Feature::procinfo_time),
        (
            "time-units-per-second",
            0,
            Feature::procinfo_time_units_per_second
        ),
    ];
}

pub trait ProcInfo {
    fn feature() -> Feature;
}

impl ProcInfo for Feature {
    fn feature() -> Feature {
        Feature {
            functions: Some(&PROCINFO_FUNCTIONS),
            symbols: Some(&PROCINFO_SYMBOLS),
            namespace: "procinfo".into(),
        }
    }
}

pub trait CoreFunction {
    fn procinfo_fds(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn procinfo_mem_res(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn procinfo_mem_virt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn procinfo_time(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn procinfo_time_units_per_second(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn procinfo_fds(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fds = fd_count_cur().unwrap();

        fp.value = Fixnum::with_u64(env, fds as u64)?;

        Ok(())
    }

    fn procinfo_time(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match cpu_time() {
            Ok(duration) => Fixnum::with_u64(env, duration.as_micros() as u64)?, // this is a u128
            Err(_) => panic!(),
        };

        Ok(())
    }

    fn procinfo_time_units_per_second(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_u64(env, 1000)?;

        Ok(())
    }

    fn procinfo_mem_res(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vmem_info = get_process_memory_info().unwrap().resident_set_size;

        fp.value = Fixnum::with_u64(env, vmem_info * 4)?;

        Ok(())
    }

    fn procinfo_mem_virt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vmem_info = get_process_memory_info().unwrap().virtual_memory_size;

        fp.value = Fixnum::with_u64(env, vmem_info * 4)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
