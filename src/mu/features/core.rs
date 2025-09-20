//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

//! core feature
#[rustfmt::skip]
use {
    crate::{
        core_::{
            apply::Apply,
            core::{Core as Core_, CoreFnDef, CORE},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            namespace::Namespace,
            tag::{Tag,},
            type_::{Type},
            writer::Writer,
        },
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
    futures_locks::RwLock,
    perf_monitor::{
        cpu::cpu_time,
        fd::fd_count_cur,
        mem::get_process_memory_info
    },
    std::{collections::HashMap, sync::mpsc::channel},
};

lazy_static! {
    pub static ref CORE_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref CORE_FUNCTIONS: &'static [CoreFnDef] = &[
        ("core-info", 0, Feature::core_core_info),
        ("process-fds", 0, Feature::core_fds),
        ("process-mem-res", 0, Feature::core_mem_res),
        ("process-mem-virt", 0, Feature::core_mem_virt),
        ("process-time", 0, Feature::core_time),
        ("time-units-per-second", 0, Feature::core_time_units),
        ("delay", 0, Feature::core_delay),
        ("ns-symbols", 1, Feature::core_ns_symbols),
    ];
}

pub trait Core {
    fn feature() -> Feature;
    fn eprint(_: &Env, label: &str, verbose: bool, tag: Tag);
    fn eprintln(_: &Env, label: &str, verbose: bool, tag: Tag);
    fn print(_: &Env, label: &str, verbose: bool, tag: Tag);
    fn println(_: &Env, label: &str, verbose: bool, tag: Tag);
}

impl Core for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: Some(&CORE_SYMBOLS),
            functions: Some(&CORE_FUNCTIONS),
            namespace: "feature/core".into(),
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

pub trait CoreFn {
    fn core_core_info(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_delay(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_fds(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_mem_res(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_mem_virt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_time(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_time_units(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_ns_symbols(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Feature {
    fn core_delay(env: &Env, fp: &mut Frame) -> exception::Result<()> {
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

        fp.value = Fixnum::with_u64(env, fds as u64)?;

        Ok(())
    }

    fn core_time(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match cpu_time() {
            Ok(duration) => Fixnum::with_u64(env, duration.as_micros() as u64)?, // this is a u128
            Err(_) => panic!(),
        };

        Ok(())
    }

    fn core_time_units(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_u64(env, 1000)?;

        Ok(())
    }

    fn core_mem_res(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vmem_info = get_process_memory_info().unwrap().resident_set_size;

        fp.value = Fixnum::with_u64(env, vmem_info * 4)?;

        Ok(())
    }

    fn core_mem_virt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vmem_info = get_process_memory_info().unwrap().virtual_memory_size;

        fp.value = Fixnum::with_u64(env, vmem_info * 4)?;

        Ok(())
    }

    fn core_core_info(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let version = env!("CARGO_PKG_VERSION");
        let alist = vec![
            Cons::cons(
                env,
                Vector::from("version").evict(env),
                Vector::from(version).evict(env),
            ),
            Cons::cons(
                env,
                Vector::from("features").evict(env),
                Core_::features_as_list(env),
            ),
            Cons::cons(
                env,
                Vector::from("envs").evict(env),
                Core_::envs_as_list(env),
            ),
            Cons::cons(env, Vector::from("streams").evict(env), Core_::nstreams()),
        ];

        fp.value = Cons::list(env, &alist);

        Ok(())
    }

    fn core_ns_symbols(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let mut ns = fp.argv[0];

        if Tag::null_(&ns) {
            ns = env.null_ns
        }
        let (stype, _) = Struct::destruct(env, ns);
        if !stype.eq_(&Symbol::keyword("ns")) {
            Err(Exception::new(env, Condition::Type, "mu:intern", ns))?
        }

        let ns_ref = block_on(env.ns_map.read());
        let ns_map = ns_ref
            .iter()
            .find_map(
                |(tag, _, ns_map)| {
                    if ns.eq_(tag) {
                        Some(ns_map)
                    } else {
                        None
                    }
                },
            )
            .unwrap();

        let hash_ref = block_on(match ns_map {
            Namespace::Static(static_) => match static_.hash {
                Some(hash) => hash.read(),
                None => {
                    fp.value = Tag::nil();
                    return Ok(());
                }
            },
            Namespace::Dynamic(hash) => hash.read(),
        });

        fp.value = Cons::list(
            env,
            &hash_ref
                .keys()
                .map(|key| hash_ref[key])
                .collect::<Vec<Tag>>(),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
