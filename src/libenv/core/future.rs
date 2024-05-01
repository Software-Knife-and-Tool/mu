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
        indirect_vector::{TypedVector, VecType, VectorIter},
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
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

pub enum Future {
    Eager(std::thread::JoinHandle<Tag>),
    Lazy(Tag, Tag),
}

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
    fn make_defer_future(_: &Env, _: Tag, _: Tag) -> exception::Result<Tag>;
    fn make_detach_future(_: &Env, _: Tag, _: Tag) -> exception::Result<Tag>;
    fn is_future_complete(_: &Env, _: Tag) -> bool;
}

impl Core for Future {
    fn make_defer_future(env: &Env, func: Tag, args: Tag) -> exception::Result<Tag> {
        let mut futures_ref = block_on(LIB.futures.write());
        let mut future_id_ref = block_on(LIB.future_id.write());
        let future_id = *future_id_ref;

        *future_id_ref = future_id + 1;

        let future = Struct {
            stype: Symbol::keyword("future"),
            vector: TypedVector::<Vec<Tag>> {
                vec: vec![Fixnum::as_tag(future_id as i64)],
            }
            .vec
            .to_vector()
            .evict(env),
        }
        .evict(env);

        futures_ref.insert(future_id, Future::Lazy(func, args));

        Ok(future)
    }

    fn make_detach_future(env: &Env, func: Tag, args: Tag) -> exception::Result<Tag> {
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

        let mut futures_ref = block_on(LIB.futures.write());
        let mut future_id_ref = block_on(LIB.future_id.write());
        let future_id = *future_id_ref;

        *future_id_ref = future_id + 1;

        let future = Struct {
            stype: Symbol::keyword("future"),
            vector: TypedVector::<Vec<Tag>> {
                vec: vec![Fixnum::as_tag(future_id as i64)],
            }
            .vec
            .to_vector()
            .evict(env),
        }
        .evict(env);

        let join_id = std::thread::spawn(move || {
            let env_ref = block_on(LIB.env_map.read());
            let env = env_ref.get(&env_tag).unwrap();

            let values: Vec<Tag> = executor::block_on(fut_values);
            env.apply(values[0], values[1]).unwrap()
        });

        futures_ref.insert(future_id, Future::Eager(join_id));

        Ok(future)
    }

    fn is_future_complete(env: &Env, future: Tag) -> bool {
        let futures_ref = block_on(LIB.futures.read());

        let index = Vector::ref_(env, Struct::vector(env, future), 0).unwrap();

        let join_id = match &futures_ref.get(&(Fixnum::as_i64(index) as u64)).unwrap() {
            Future::Eager(join_id) => join_id,
            Future::Lazy(_, _) => return false,
        };

        join_id.is_finished()
    }
}

pub trait CoreFunction {
    fn lib_future_defer(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_future_detach(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_future_force(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_future_poll(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Future {
    fn lib_future_force(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let future = fp.argv[0];

        fp.value = match env.fp_argv_check("fwait", &[Type::Struct], fp) {
            Ok(_) if Struct::stype(env, future).eq_(&Symbol::keyword("future")) => {
                let mut futures_ref = block_on(LIB.futures.write());
                let index = Vector::ref_(env, Struct::vector(env, future), 0).unwrap();

                match futures_ref.remove(&(Fixnum::as_i64(index) as u64)).unwrap() {
                    Future::Eager(join_id) => join_id.join().unwrap(),
                    Future::Lazy(func, args) => env.apply(func, args).unwrap(),
                }
            }
            Ok(_) => return Err(Exception::new(Condition::Type, "fwait", future)),
            Err(e) => return Err(e),
        };

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
            Ok(_) => return Err(Exception::new(Condition::Type, "poll", future)),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_future_defer(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        fp.value = match env.fp_argv_check("defer", &[Type::Function, Type::List], fp) {
            Ok(_) => match Self::make_defer_future(env, func, args) {
                Ok(future) => future,
                Err(_) => return Err(Exception::new(Condition::Type, "defer", Tag::nil())),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_future_detach(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        fp.value = match env.fp_argv_check("detach", &[Type::Function, Type::List], fp) {
            Ok(_) => match Self::make_detach_future(env, func, args) {
                Ok(future) => future,
                Err(_) => return Err(Exception::new(Condition::Type, "detach", Tag::nil())),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
