//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env gc
#![allow(dead_code)]
use crate::{
    core::{
        env::Env,
        exception,
        frame::Frame,
        heap::{Gc as _, HeapAllocator},
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, Gc as _},
        function::{Function, Gc as _},
        struct_::{Gc as _, Struct},
        symbol::{Gc as _, Symbol},
        vector::Vector,
    },
    vectors::vector::Gc as _,
};

use futures::executor::block_on;

pub struct GcContext<'a> {
    pub heap_ref: &'a mut futures_locks::RwLockWriteGuard<HeapAllocator>,
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
            Tag::Image(_) => panic!(),
            Tag::Direct(_) => None,
            Tag::Indirect(indirect) => {
                let marked = self.heap_ref.get_image_mark(indirect.image_id() as usize);

                match marked {
                    None => (),
                    Some(mark) => {
                        if !mark {
                            self.heap_ref.set_image_mark(indirect.image_id() as usize)
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
            let frame_vec_ref = block_on(frame_vec.read());

            for frame in frame_vec_ref.iter() {
                self.mark(env, frame.func);

                for arg in &frame.argv {
                    self.mark(env, *arg)
                }

                self.mark(env, frame.value);
            }
        }
    }

    fn namespaces(&mut self, env: &Env) {
        let ns_map_ref = block_on(env.ns_map.read());

        for (_, _, ns_cache) in &*ns_map_ref {
            let hash_ref = block_on(match ns_cache {
                Namespace::Static(static_) => match static_.hash {
                    Some(hash) => hash.read(),
                    None => return,
                },
                Namespace::Dynamic(ref hash) => hash.read(),
            });

            for (_, symbol) in hash_ref.iter() {
                self.mark(env, *symbol)
            }
        }
    }

    /*
        fn gc_with_context(&mut self, env: &Env) -> exception::Result<bool> {
            self.heap_ref.clear_marks();
            self.namespaces(env);
            self.lexicals(env);
            self.heap_ref.sweep();

            Ok(true)
    }
        */

    fn gc(env: &Env) -> exception::Result<bool> {
        let mut gc_context = GcContext {
            heap_ref: &mut block_on(env.heap.write()),
        };

        gc_context.heap_ref.clear_marks();
        gc_context.namespaces(env);
        gc_context.lexicals(env);
        gc_context.heap_ref.sweep();

        Ok(true)
    }
}

pub trait CoreFunction {
    fn mu_gc(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for GcContext<'_> {
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
