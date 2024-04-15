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

pub struct Futures {}

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

impl Core for Futures {
    fn make_future(env: &Env, _func: Tag, _args: Tag) -> exception::Result<Tag> {
        let (tx, rx) = mpsc::unbounded::<i32>();

        let fut_values = async {
            let fut_tx_result = async move {
                (0..20).for_each(|v| {
                    tx.unbounded_send(v).expect("Failed to send");
                })
            };

            LIB.threads.pool.spawn_ok(fut_tx_result);

            let fut_values = rx.map(|v| v * 2).collect();

            fut_values.await
        };

        let join_id = std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(5));
            let values: Vec<i32> = executor::block_on(fut_values);

            Fixnum::as_tag(values.len() as i64)
        });

        let mut futures_ref = block_on(LIB.futures.write());
        let nfutures = futures_ref.len();

        let vec = vec![Fixnum::as_tag(nfutures as i64)];
        let vector = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(env);
        let stype = Symbol::keyword("future");
        let future = Struct { stype, vector }.evict(env);

        futures_ref.push(join_id);

        Ok(future)
    }

    fn is_future_complete(env: &Env, future: Tag) -> bool {
        let futures_ref = block_on(LIB.futures.read());

        let index = Vector::ref_(env, Struct::vector(env, future), 0).unwrap();
        let join_id = &futures_ref[Fixnum::as_i64(index) as usize];

        join_id.is_finished()
    }
}

pub trait LibFunction {
    fn lib_future(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_fwait(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_ftest(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_fcomplete(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Futures {
    fn lib_fwait(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let future = fp.argv[0];

        let mut futures_ref = block_on(LIB.futures.write());

        let index = Vector::ref_(env, Struct::vector(env, future), 0).unwrap();
        let join_id = futures_ref.remove(Fixnum::as_i64(index) as usize);

        fp.value = join_id.join().unwrap();

        Ok(())
    }

    fn lib_fcomplete(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let future = fp.argv[0];

        fp.value = if Self::is_future_complete(env, future) {
            future
        } else {
            Tag::nil()
        };

        Ok(())
    }

    fn lib_future(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        let future = Self::make_future(env, func, args).unwrap();

        fp.value = future;

        Ok(())
    }

    fn lib_ftest(_: &Env, fp: &mut Frame) -> exception::Result<()> {
        let (tx, rx) = mpsc::unbounded::<i32>();

        let fut_values = async {
            let fut_tx_result = async move {
                (0..20).for_each(|v| {
                    tx.unbounded_send(v).expect("Failed to send");
                })
            };

            LIB.threads.pool.spawn_ok(fut_tx_result);

            let fut_values = rx.map(|v| v * 2).collect();

            fut_values.await
        };

        let values: Vec<i32> = executor::block_on(fut_values);

        println!("Values={:?}", values);

        fp.value = Tag::nil();
        Ok(())
    }
}
