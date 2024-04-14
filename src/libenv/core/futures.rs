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
        vecimage::VectorIter,
        vector::{Core as _, Vector},
    },
};

#[allow(unused_imports)]
use {futures::executor::block_on, futures_locks::RwLock};
use {
    futures::{
        executor::{
            self,
            ThreadPool,
        },
        channel::mpsc,
        StreamExt,
    },
};

pub struct Futures {
    pool: ThreadPool,
}

impl Futures {
    pub fn new() -> Self {
        Futures {
            pool: ThreadPool::new().expect("Failed to build pool"),
        }
    }
}

trait Core {
    fn make(_: &Env) -> exception::Result<Tag>;
}

impl Core for Futures {
    fn make(_env: &Env) -> exception::Result<Tag> {
        let (tx, rx) = mpsc::unbounded::<i32>();

        let fut_values = async {
            let fut_tx_result = async move {
                (0..100).for_each(|v| {
                    tx.unbounded_send(v).expect("Failed to send");
                })
            };

            LIB.futures.pool.spawn_ok(fut_tx_result);
            
            let fut_values = rx
                .map(|v| v * 2)
                .collect();

            fut_values.await
        };

        let values: Vec<i32> = executor::block_on(fut_values);

        println!("Values={:?}", values);

        Ok(Tag::nil())
    }
}

pub trait LibFunction {
    fn lib_fwait(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Futures {
    fn lib_fwait(_: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Tag::nil();

        Ok(())
    }
}
