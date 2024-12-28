//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! futures
use crate::{
    core::{
        apply::Apply as _,
        core::CORE,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        types::{Tag, Type},
    },
    types::{fixnum::Fixnum, struct_::Struct, symbol::Symbol, vector::Vector},
};

#[allow(unused_imports)]
use {
    futures::{
        channel::mpsc,
        executor::{self, block_on, ThreadPool},
        StreamExt,
    },
    futures_locks::RwLock,
};

pub enum Future {
    Eager(std::thread::JoinHandle<Tag>),
    Lazy(Tag, Tag),
}

pub struct FuturePool {
    pool: ThreadPool,
}

impl Default for FuturePool {
    fn default() -> Self {
        Self::new()
    }
}

impl FuturePool {
    pub fn new() -> Self {
        FuturePool {
            pool: ThreadPool::new().expect("Failed to build pool"),
        }
    }
}

impl Future {
    fn make_defer_future(env: &Env, func: Tag, args: Tag) -> exception::Result<Tag> {
        let mut futures_ref = block_on(CORE.futures.write());
        let mut future_id_ref = block_on(CORE.future_id.write());
        let future_id = *future_id_ref;

        *future_id_ref = future_id + 1;

        let future = Struct {
            stype: Symbol::keyword("future"),
            vector: Vector::from(vec![Fixnum::with_u64_or_panic(future_id)]).evict(env),
        }
        .evict(env);

        futures_ref.insert(future_id, Future::Lazy(func, args));

        Ok(future)
    }

    fn make_detach_future(env: &Env, func: Tag, args: Tag) -> exception::Result<Tag> {
        let env_ref = block_on(env.env_key.read());
        let env_tag = (*env_ref).as_u64();

        let (tx, rx) = mpsc::unbounded::<Tag>();

        let fut_values = async move {
            let fut_tx_result = async move {
                let tags: Vec<Tag> = vec![func, args];

                for tag in tags.into_iter() {
                    tx.unbounded_send(tag).expect("Failed to send")
                }
            };

            CORE.threads.pool.spawn_ok(fut_tx_result);

            let fut_values = rx.map(|v| v).collect();

            fut_values.await
        };

        let mut futures_ref = block_on(CORE.futures.write());
        let mut future_id_ref = block_on(CORE.future_id.write());
        let future_id = *future_id_ref;

        *future_id_ref = future_id + 1;

        let future = Struct {
            stype: Symbol::keyword("future"),
            vector: Vector::from(vec![Fixnum::with_u64_or_panic(future_id)]).evict(env),
        }
        .evict(env);

        let join_id = std::thread::spawn(move || {
            let env_ref = block_on(CORE.env_map.read());
            let env = env_ref.get(&env_tag).unwrap();

            let values: Vec<Tag> = executor::block_on(fut_values);
            env.apply(values[0], values[1]).unwrap()
        });

        futures_ref.insert(future_id, Future::Eager(join_id));

        Ok(future)
    }

    fn is_future_complete(env: &Env, future: Tag) -> bool {
        let futures_ref = block_on(CORE.futures.read());

        let index = Vector::ref_(env, Struct::vector(env, future), 0).unwrap();

        let join_id = match &futures_ref.get(&(Fixnum::as_i64(index) as u64)).unwrap() {
            Future::Eager(join_id) => join_id,
            Future::Lazy(_, _) => return false,
        };

        join_id.is_finished()
    }
}

pub trait CoreFunction {
    fn mu_future_defer(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_future_detach(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_future_force(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_future_poll(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Future {
    fn mu_future_force(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let future = fp.argv[0];

        env.fp_argv_check("mu:force", &[Type::Struct], fp)?;

        fp.value = if Struct::stype(env, future).eq_(&Symbol::keyword("future")) {
            let mut futures_ref = block_on(CORE.futures.write());
            let index = Vector::ref_(env, Struct::vector(env, future), 0).unwrap();

            match futures_ref.remove(&(Fixnum::as_i64(index) as u64)).unwrap() {
                Future::Eager(join_id) => join_id.join().unwrap(),
                Future::Lazy(func, args) => env.apply(func, args).unwrap(),
            }
        } else {
            return Err(Exception::new(env, Condition::Type, "mu:force", future));
        };

        Ok(())
    }

    fn mu_future_poll(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let future = fp.argv[0];

        env.fp_argv_check("mu:poll", &[Type::Struct], fp)?;
        fp.value = if Struct::stype(env, future).eq_(&Symbol::keyword("future")) {
            if Self::is_future_complete(env, future) {
                future
            } else {
                Tag::nil()
            }
        } else {
            return Err(Exception::new(env, Condition::Type, "mu:poll", future));
        };

        Ok(())
    }

    fn mu_future_defer(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        env.fp_argv_check("mu:defer", &[Type::Function, Type::List], fp)?;
        fp.value = match Self::make_defer_future(env, func, args) {
            Ok(future) => future,
            Err(_) => return Err(Exception::new(env, Condition::Type, "mu:defer", Tag::nil())),
        };

        Ok(())
    }

    fn mu_future_detach(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        env.fp_argv_check("mu:detach", &[Type::Function, Type::List], fp)?;
        fp.value = match Self::make_detach_future(env, func, args) {
            Ok(future) => future,
            Err(_) => {
                return Err(Exception::new(
                    env,
                    Condition::Type,
                    "mu:detach",
                    Tag::nil(),
                ))
            }
        };

        Ok(())
    }
}
