//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! cpu-time interface
use crate::{
    core::{
        core::CoreFnDef,
        env::Env,
        exception::{self},
        frame::Frame,
        types::Tag,
    },
    features::feature::Feature,
    types::fixnum::Fixnum,
};
use futures_locks::RwLock;
use std::collections::HashMap;

use cpu_time::{self, ProcessTime};

lazy_static! {
    pub static ref CPU_TIME_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref CPU_TIME_FUNCTIONS: Vec<CoreFnDef> = vec![
        ("process-time", 0, Feature::cpu_time_process_time),
        (
            "time-units-per-second",
            0,
            Feature::cpu_time_units_per_second
        ),
    ];
}

pub trait CpuTime {
    fn feature() -> Feature;
}

impl CpuTime for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: Some(&CPU_TIME_SYMBOLS),
            functions: Some(&CPU_TIME_FUNCTIONS),
            namespace: "cpu-time".into(),
        }
    }
}

pub trait CoreFunction {
    fn cpu_time_process_time(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn cpu_time_units_per_second(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn cpu_time_process_time(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match ProcessTime::try_now() {
            Ok(process_time) => {
                Fixnum::with_u64(env, process_time.as_duration().as_micros() as u64)?
            } // this is a u128
            Err(_) => panic!(),
        };

        Ok(())
    }

    fn cpu_time_units_per_second(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_u64(env, 1000)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
