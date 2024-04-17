//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! futures
#[allow(unused_imports)]
use crate::{
    core::{
        apply::Core as _,
        env::{Core as _, Env},
        exception::{self, Condition, Core as _, Exception},
        frame::Frame,
        gc::Core as _,
        lib::{Core as _, LIB},
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, Core as _},
        fixnum::Fixnum,
        function::Function,
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vecimage::{TypedVec, VecType, VectorIter},
        vector::{Core as _, Vector},
    },
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

pub struct Future {}

pub struct FuturePool {
    pool: ThreadPool,
}

impl FuturePool {
    pub fn new() -> Self {
        FuturePool {
            pool: ThreadPool::new().expect("Failed to build pool"),
        }
    }
}

trait Core {
    fn make_future(_: &Env, _: Tag, _: Tag) -> exception::Result<Tag>;
    fn is_future_complete(_: &Env, _: Tag) -> bool;
}

impl Core for Future {
    fn make_future(env: &Env, func: Tag, args: Tag) -> exception::Result<Tag> {
        let env_ref = block_on(env.tag.read());
        let env_tag = (*env_ref).as_u64();

        let (tx, rx) = mpsc::unbounded::<Tag>();

        let fut_values = async move {
            let fut_tx_result = async move {
                let tags: Vec<Tag> = vec![func, args];

                for tag in tags.into_iter() {
                    tx.unbounded_send(tag).expect("Failed to send")
                }
            };

            LIB.threads.pool.spawn_ok(fut_tx_result);

            let fut_values = rx.map(|v| v).collect();

            fut_values.await
        };

        let join_id = std::thread::spawn(move || {
            let values: Vec<Tag> = executor::block_on(fut_values);

            let env_ref = block_on(LIB.env_map.read());
            let env = env_ref.get(&env_tag).unwrap();

            env.apply(values[0], values[1]).unwrap()
        });

        let mut futures_ref = block_on(LIB.futures.write());
        let mut future_id_ref = block_on(LIB.future_id.write());
        let future_id = *future_id_ref;

        *future_id_ref = future_id + 1;

        let vec = vec![Fixnum::as_tag(future_id as i64)];
        let vector = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(env);
        let stype = Symbol::keyword("future");
        let future = Struct { stype, vector }.evict(env);

        futures_ref.insert(future_id, join_id);

        Ok(future)
    }

    fn is_future_complete(env: &Env, future: Tag) -> bool {
        let futures_ref = block_on(LIB.futures.read());

        let index = Vector::ref_(env, Struct::vector(env, future), 0).unwrap();
        let join_id = &futures_ref.get(&(Fixnum::as_i64(index) as u64)).unwrap();

        join_id.is_finished()
    }
}

pub trait LibFunction {
    fn lib_future_apply(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_future_wait(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_future_poll(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Future {
    fn lib_future_wait(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let future = fp.argv[0];

        let mut futures_ref = block_on(LIB.futures.write());

        let index = Vector::ref_(env, Struct::vector(env, future), 0).unwrap();
        let join_id = futures_ref.remove(&(Fixnum::as_i64(index) as u64)).unwrap();

        fp.value = join_id.join().unwrap();

        Ok(())
    }

    fn lib_future_poll(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let future = fp.argv[0];

        fp.value = match env.fp_argv_check("fpoll", &[Type::Struct], fp) {
            Ok(_) if Struct::stype(env, future).eq_(&Symbol::keyword("future")) => {
                if Self::is_future_complete(env, future) {
                    future
                } else {
                    Tag::nil()
                }
            }
            Ok(_) => return Err(Exception::new(Condition::Type, "fpoll", future)),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_future_apply(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        fp.value = match env.fp_argv_check("fapply", &[Type::Function, Type::List], fp) {
            Ok(_) => match Self::make_future(env, func, args) {
                Ok(future) => future,
                Err(_) => return Err(Exception::new(Condition::Future, "fapply", Tag::nil())),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
