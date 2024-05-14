//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env gc
//!    Env
use crate::{
    core::{
        env::{Env, HeapRef},
        exception,
        frame::Frame,
        types::{Tag, Type},
    },
    types::{
        cons::Cons,
        function::Function,
        namespace::Namespace,
        struct_::Struct,
        symbol::{Core as _, Symbol},
        vector::Vector,
    },
};

use futures::executor::block_on;

pub struct Gc {}

#[derive(Debug, Copy, Clone)]
pub enum GcMode {
    None,
    Auto,
    Demand,
}

impl Gc {
    pub fn mark(env: &Env, heap_ref: HeapRef, tag: Tag) {
        match tag.type_of() {
            Type::Cons => Cons::mark(env, heap_ref, tag),
            Type::Function => Function::mark(env, heap_ref, tag),
            Type::Struct => Struct::mark(env, heap_ref, tag),
            Type::Symbol => Symbol::mark(env, heap_ref, tag),
            Type::Vector => Vector::mark(env, heap_ref, tag),
            _ => (),
        }
    }

    fn add_root(env: &Env, tag: Tag) {
        let mut root_ref = block_on(env.gc_root.write());

        root_ref.push(tag);
    }

    pub fn mark_image(heap_ref: HeapRef, tag: Tag) -> Option<bool> {
        match tag {
            Tag::Direct(_) => None,
            Tag::Indirect(indirect) => {
                let marked = heap_ref.get_image_mark(indirect.image_id() as usize);

                match marked {
                    None => (),
                    Some(mark) => {
                        if !mark {
                            heap_ref.set_image_mark(indirect.image_id() as usize)
                        }
                    }
                }

                marked
            }
        }
    }

    fn lexicals(env: &Env, heap_ref: HeapRef) {
        let lexical_ref = block_on(env.lexical.read());

        for frame_vec in (*lexical_ref).values() {
            let frame_vec_ref = block_on(frame_vec.read());

            for frame in frame_vec_ref.iter() {
                Self::mark(env, heap_ref, frame.func);

                for arg in &frame.argv {
                    Self::mark(env, heap_ref, *arg)
                }

                Self::mark(env, heap_ref, frame.value);
            }
        }
    }

    fn namespaces(env: &Env, heap_ref: HeapRef) {
        let ns_index_ref = block_on(env.ns_map.read());

        for (_, _, ns_cache) in &*ns_index_ref {
            let hash_ref = block_on(match ns_cache {
                Namespace::Static(hash) => hash.read(),
                Namespace::Dynamic(ref hash) => hash.read(),
            });

            for (_, symbol) in hash_ref.iter() {
                Self::mark(env, heap_ref, *symbol)
            }
        }
    }
}

pub trait Core {
    fn gc(_: &Env) -> exception::Result<bool>;
}

impl Core for Gc {
    fn gc(env: &Env) -> exception::Result<bool> {
        let mut heap_ref = block_on(env.heap.write());
        let root_ref = block_on(env.gc_root.write());

        heap_ref.clear_marks();

        Self::namespaces(env, &mut heap_ref);
        Self::lexicals(env, &mut heap_ref);

        for tag in &*root_ref {
            Self::mark(env, &mut heap_ref, *tag)
        }

        heap_ref.sweep();

        Ok(true)
    }
}

pub trait CoreFunction {
    fn core_gc(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Gc {
    fn core_gc(env: &Env, fp: &mut Frame) -> exception::Result<()> {
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
