//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env gc
#![allow(dead_code)]
use crate::{
    core::{env::Env, exception, frame::Frame, tag::Tag, type_::Type},
    gc::{
        async_::Gc as _, cons::Gc as _, function::Gc as _, struct_::Gc as _, symbol::Gc as _,
        vector::Gc as _,
    },
    namespaces::{
        heap::{Gc as _, Heap},
        namespace::Namespace,
    },
    types::{
        async_::Async, cons::Cons, function::Function, struct_::Struct, symbol::Symbol,
        vector::Vector,
    },
};

use futures_lite::future::block_on;

pub struct GcContext<'a> {
    pub heap_ref: &'a mut futures_locks::RwLockWriteGuard<Heap>,
}

pub trait Gc {
    fn gc(_: &Env) -> exception::Result<bool>;
    //    fn gc_with_context(&mut self, _: &Env) -> exception::Result<bool>;
    fn lexicals(&mut self, _: &Env);
    fn mark(&mut self, _: &Env, tag: Tag);
    fn mark_image(&mut self, _: Tag) -> Option<bool>;
    fn namespaces(&mut self, _: &Env);
}

impl Gc for GcContext<'_> {
    fn mark(&mut self, env: &Env, tag: Tag) {
        match tag.type_of() {
            Type::Async => Async::mark(self, env, tag),
            Type::Cons => Cons::mark(self, env, tag),
            Type::Function => Function::mark(self, env, tag),
            Type::Struct => Struct::mark(self, env, tag),
            Type::Symbol => Symbol::mark(self, env, tag),
            Type::Vector => Vector::mark(self, env, tag),
            _ => (),
        }
    }

    fn mark_image(&mut self, tag: Tag) -> Option<bool> {
        match tag {
            Tag::Direct(_) => None,
            Tag::Indirect(indirect) => {
                let marked = self
                    .heap_ref
                    .get_image_mark(usize::try_from(indirect.image_id()).unwrap());

                match marked {
                    None => (),
                    Some(mark) => {
                        if !mark {
                            self.heap_ref
                                .set_image_mark(usize::try_from(indirect.image_id()).unwrap());
                        }
                    }
                }

                marked
            }
        }
    }

    fn lexicals(&mut self, env: &Env) {
        let lexical_ref = block_on(env.lexical.read());

        for frame_vec in (*lexical_ref).values() {
            let frame_vec_ref = &frame_vec;

            for frame in *frame_vec_ref {
                self.mark(env, frame.func);

                for arg in &frame.argv {
                    self.mark(env, *arg);
                }

                self.mark(env, frame.value);
            }
        }
    }

    fn namespaces(&mut self, env: &Env) {
        let ns_ref = block_on(env.ns_map.read());

        for (_, ns_map) in ns_ref.values() {
            match ns_map {
                Namespace::Static(static_) => {
                    if let Some(hash) = &static_ {
                        for symbol in hash.values() {
                            self.mark(env, *symbol);
                        }
                    }
                }
                Namespace::Dynamic(ref hash) => {
                    let hash_ref = block_on(hash.read());

                    for symbol in hash_ref.values() {
                        self.mark(env, *symbol);
                    }
                }
            }
        }
    }

    fn gc(env: &Env) -> exception::Result<bool> {
        let mut gc = GcContext {
            heap_ref: &mut block_on(env.heap.write()),
        };

        gc.heap_ref.clear_marks();
        gc.namespaces(env);
        gc.lexicals(env);
        gc.heap_ref.sweep();

        Ok(true)
    }
}

pub trait CoreFn {
    fn mu_gc(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for GcContext<'_> {
    fn mu_gc(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        Self::gc(env)?;

        fp.value = Symbol::keyword("t");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn env() {
        assert_eq!(2 + 2, 4);
    }
}
