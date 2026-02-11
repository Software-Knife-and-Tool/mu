//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

//! core feature
#[rustfmt::skip]
use {
    crate::{
        core::{
            apply::Apply,
            core_::{Core as Core_},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            tag::{Tag,},
            type_::{Type},
        },
        namespaces::namespace::Namespace,
        features::feature::Feature,
        types::{
            cons::Cons,
            fixnum::Fixnum,
            struct_::Struct,
            symbol::Symbol,
            vector::Vector
        },
    },
    futures_lite::future::block_on,
    perf_monitor::{
        cpu::cpu_time,
        fd::fd_count_cur,
        mem::get_process_memory_info
    },
    std::{sync::mpsc::channel},
};

pub trait Core {
    fn feature() -> Feature;
}

impl Core for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: None,
            functions: Some(vec![
                ("core-info", 0, Feature::core_core_info),
                ("process-fds", 0, Feature::core_fds),
                ("process-mem-res", 0, Feature::core_mem_res),
                ("process-mem-virt", 0, Feature::core_mem_virt),
                ("process-time", 0, Feature::core_time),
                ("time-units-per-second", 0, Feature::core_time_units),
                ("sleep", 1, Feature::core_sleep),
                ("ns-symbols", 1, Feature::core_ns_symbols),
            ]),
            namespace: "feature/core".into(),
        }
    }
}

pub trait CoreFn {
    fn core_core_info(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_sleep(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_fds(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_mem_res(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_mem_virt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_time(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_time_units(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_ns_symbols(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Feature {
    fn core_sleep(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("%core:delay", &[Type::Fixnum], fp)?;

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

    fn core_fds(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fds = fd_count_cur().unwrap();

        fp.value = Fixnum::with_u64(env, fds as u64, "core:core")?;

        Ok(())
    }

    fn core_time(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_u64(
            env,
            u64::try_from(cpu_time().unwrap().as_micros()).unwrap(),
            "core:core",
        )?;

        Ok(())
    }

    fn core_time_units(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_u64(env, 1000, "coree:core")?;

        Ok(())
    }

    fn core_mem_res(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vmem_info = get_process_memory_info().unwrap().resident_set_size;

        fp.value = Fixnum::with_u64(env, vmem_info * 4, "core:core")?;

        Ok(())
    }

    fn core_mem_virt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vmem_info = get_process_memory_info().unwrap().virtual_memory_size;

        fp.value = Fixnum::with_u64(env, vmem_info * 4, "core:core")?;

        Ok(())
    }

    fn core_core_info(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let version = env!("CARGO_PKG_VERSION");
        let alist = vec![
            Cons::cons(
                env,
                Vector::from("version").with_heap(env),
                Vector::from(version).with_heap(env),
            ),
            Cons::cons(
                env,
                Vector::from("features").with_heap(env),
                Core_::features_as_list(env),
            ),
            Cons::cons(
                env,
                Vector::from("streams").with_heap(env),
                Core_::nstreams(env),
            ),
        ];

        fp.value = Cons::list(env, &alist);

        Ok(())
    }

    fn core_ns_symbols(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];
        let (stype, svec) = Struct::destruct(env, ns);

        if !stype.eq_(&Symbol::keyword("ns")) {
            Err(Exception::err(env, ns, Condition::Type, "mu:intern"))?;
        }

        let name = Vector::as_string(env, Vector::ref_(env, svec, 0).unwrap());
        let ns_ref = block_on(env.ns_map.read());

        fp.value = match &ns_ref[&name].1 {
            Namespace::Static(static_) => match &static_ {
                Some(hash) => {
                    Cons::list(env, &hash.keys().map(|key| hash[key]).collect::<Vec<Tag>>())
                }
                None => Tag::nil(),
            },
            Namespace::Dynamic(hash) => {
                let hash_ref = block_on(hash.read());

                Cons::list(
                    env,
                    &hash_ref
                        .keys()
                        .map(|key| hash_ref[key])
                        .collect::<Vec<Tag>>(),
                )
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
