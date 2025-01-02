//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env gc
//!    Env
use crate::{
    core::{
        env::Env,
        exception,
        frame::Frame,
        heap::{HeapAllocator, GC},
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, GC as _},
        function::{Function, GC as _},
        struct_::{Struct, GC as _},
        symbol::{Symbol, GC as _},
        vector::Vector,
    },
    vectors::vector::GC as _,
};

use futures::executor::block_on;

pub type HeapGcRef = futures_locks::RwLockWriteGuard<HeapAllocator>;

pub struct Gc {
    pub lock: HeapGcRef,
}

impl Gc {
    #[allow(dead_code)]
    fn add_root(env: &Env, tag: Tag) {
        let mut root_ref = block_on(env.gc_root.write());

        root_ref.push(tag);
    }

    pub fn mark(&mut self, env: &Env, tag: Tag) {
        match tag.type_of() {
            Type::Cons => Cons::mark(self, env, tag),
            Type::Function => Function::mark(self, env, tag),
            Type::Struct => Struct::mark(self, env, tag),
            Type::Symbol => Symbol::mark(self, env, tag),
            Type::Vector => Vector::mark(self, env, tag),
            _ => (),
        }
    }

    pub fn mark_image(&mut self, tag: Tag) -> Option<bool> {
        match tag {
            Tag::Direct(_) => None,
            Tag::Indirect(indirect) => {
                let marked = self.lock.get_image_mark(indirect.image_id() as usize);

                match marked {
                    None => (),
                    Some(mark) => {
                        if !mark {
                            self.lock.set_image_mark(indirect.image_id() as usize)
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

    pub fn gc(env: &Env) -> exception::Result<bool> {
        let root_ref = block_on(env.gc_root.write());
        let mut gc = Gc {
            lock: block_on(env.heap.write()),
        };

        gc.lock.clear_marks();

        gc.namespaces(env);
        gc.lexicals(env);

        for tag in &*root_ref {
            gc.mark(env, *tag)
        }

        gc.lock.sweep();

        Ok(true)
    }
}

pub trait CoreFunction {
    fn mu_gc(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Gc {
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
