//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! lib implementation
use perf_monitor::{cpu::cpu_time, fd::fd_count_cur, mem::get_process_memory_info};
use {
    crate::{
        core::{
            apply::Apply,
            core::{Core, CoreFnDef, VERSION},
            env::Env,
            exception::{self},
            frame::Frame,
            types::{Tag, Type},
        },
        features::feature::Feature,
        types::{cons::Cons, fixnum::Fixnum, vector::Vector},
    },
    futures_locks::RwLock,
    std::{collections::HashMap, sync::mpsc::channel},
};

lazy_static! {
    pub static ref MU_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref MU_FUNCTIONS: Vec<CoreFnDef> = vec![
        ("core", 0, Feature::mu_core),
        ("process-fds", 0, Feature::mu_fds),
        ("process-mem-res", 0, Feature::mu_mem_res),
        ("process-mem-virt", 0, Feature::mu_mem_virt),
        ("process-time", 0, Feature::mu_time),
        ("time-units-per-second", 0, Feature::mu_time_units),
        ("delay", 0, Feature::mu_delay),
    ];
}

pub trait Mu {
    fn feature() -> Feature;
}

impl Mu for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: Some(&MU_SYMBOLS),
            functions: Some(&MU_FUNCTIONS),
            namespace: "%mu".into(),
        }
    }
}

pub trait CoreFunction {
    fn mu_core(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fds(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_mem_res(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_mem_virt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_time(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_time_units(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_delay(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn mu_delay(env: &Env, fp: &mut Frame) -> exception::Result<()> {
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

    fn mu_fds(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fds = fd_count_cur().unwrap();

        fp.value = Fixnum::with_u64(env, fds as u64)?;

        Ok(())
    }

    fn mu_time(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match cpu_time() {
            Ok(duration) => Fixnum::with_u64(env, duration.as_micros() as u64)?, // this is a u128
            Err(_) => panic!(),
        };

        Ok(())
    }

    fn mu_time_units(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_u64(env, 1000)?;

        Ok(())
    }

    fn mu_mem_res(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vmem_info = get_process_memory_info().unwrap().resident_set_size;

        fp.value = Fixnum::with_u64(env, vmem_info * 4)?;

        Ok(())
    }

    fn mu_mem_virt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vmem_info = get_process_memory_info().unwrap().virtual_memory_size;

        fp.value = Fixnum::with_u64(env, vmem_info * 4)?;

        Ok(())
    }

    fn mu_core(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let alist = vec![
            Cons::cons(
                env,
                Vector::from("version").evict(env),
                Vector::from(VERSION).evict(env),
            ),
            Cons::cons(
                env,
                Vector::from("features").evict(env),
                Core::features_as_list(env),
            ),
            Cons::cons(
                env,
                Vector::from("envs").evict(env),
                Core::envs_as_list(env),
            ),
            Cons::cons(env, Vector::from("streams").evict(env), Core::nstreams()),
        ];

        fp.value = Cons::list(env, &alist);

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
