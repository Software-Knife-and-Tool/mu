//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env gc
//!    Env
use crate::{
    core::{
        env::Env,
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
    pub fn mark(env: &Env, tag: Tag) {
        match tag.type_of() {
            Type::Cons => Cons::mark(env, tag),
            Type::Function => Function::mark(env, tag),
            Type::Struct => Struct::mark(env, tag),
            Type::Symbol => Symbol::mark(env, tag),
            Type::Vector => Vector::mark(env, tag),
            _ => (),
        }
    }

    fn add_root(env: &Env, tag: Tag) {
        let mut root_ref = block_on(env.gc_root.write());

        root_ref.push(tag);
    }

    pub fn mark_image(env: &Env, tag: Tag) -> Option<bool> {
        let mut heap_ref = block_on(env.heap.write());

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

    fn lexicals(env: &Env) {
        let lexical_ref = block_on(env.lexical.read());

        for frame_vec in (*lexical_ref).values() {
            let frame_vec_ref = block_on(frame_vec.read());

            for frame in frame_vec_ref.iter() {
                Self::mark(env, frame.func);

                for arg in &frame.argv {
                    Self::mark(env, *arg)
                }

                Self::mark(env, frame.value);
            }
        }
    }

    fn namespaces(env: &Env) {
        let ns_map_ref = block_on(env.ns_map.read());

        for (_, _, ns_cache) in &*ns_map_ref {
            let hash_ref = block_on(match ns_cache {
                Namespace::Static(hash) => hash.read(),
                Namespace::Dynamic(ref hash) => hash.read(),
            });
            for (_, symbol) in hash_ref.iter() {
                Self::mark(env, *symbol)
            }
        }
    }
}

pub trait Core {
    fn gc(_: &Env) -> exception::Result<bool>;
}

impl Core for Gc {
    fn gc(env: &Env) -> exception::Result<bool> {
        let root_ref = block_on(env.gc_root.write());

        {
            let mut heap_ref = block_on(env.heap.write());
            heap_ref.clear_marks();
        }

        Self::namespaces(env);

        Self::lexicals(env);

        for tag in &*root_ref {
            Self::mark(env, *tag)
        }

        {
            let mut heap_ref = block_on(env.heap.write());

            heap_ref.sweep();
        }

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
