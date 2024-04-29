//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env gc
//!    Env
use crate::{
    core::{
        env::Env,
        exception,
        frame::Frame,
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, Core as _},
        function::{Core as _, Function},
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vectors::{Core as _, Vector},
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

pub trait Core {
    fn gc(&self) -> exception::Result<bool>;
    fn mark(&self, _: Tag);
    fn add_gc_root(&self, _: Tag);
    fn gc_namespaces(&self);
    fn mark_image(&self, _: Tag) -> Option<bool>;
}

impl Core for Env {
    fn mark(&self, tag: Tag) {
        match tag.type_of() {
            Type::Cons => Cons::mark(self, tag),
            Type::Function => Function::mark(self, tag),
            Type::Struct => Struct::mark(self, tag),
            Type::Symbol => Symbol::mark(self, tag),
            Type::Vector => Vector::mark(self, tag),
            _ => (),
        }
    }

    fn gc(&self) -> exception::Result<bool> {
        let root_ref = block_on(self.gc_root.write());

        {
            let mut heap_ref = block_on(self.heap.write());
            heap_ref.clear_marks();
        }

        self.gc_namespaces();

        Frame::gc_lexical(self);

        let _ = root_ref.iter().map(|tag| Self::mark(self, *tag));

        {
            let mut heap_ref = block_on(self.heap.write());
            heap_ref.sweep();
        }

        Ok(true)
    }

    fn add_gc_root(&self, tag: Tag) {
        let mut root_ref = block_on(self.gc_root.write());

        root_ref.push(tag);
    }

    fn mark_image(&self, tag: Tag) -> Option<bool> {
        match tag {
            Tag::Direct(_) => None,
            Tag::Indirect(indirect) => {
                let mut heap_ref = block_on(self.heap.write());

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

    fn gc_namespaces(&self) {
        let ns_index_ref = block_on(self.ns_map.read());

        for (_, ns) in ns_index_ref.iter() {
            let hash_ref = block_on(match ns.1 {
                Namespace::Static(hash) => hash.read(),
                Namespace::Dynamic(ref hash) => hash.read(),
            });

            for (_, symbol) in hash_ref.iter() {
                self.mark(*symbol)
            }
        }
    }
}

pub trait LibFunction {
    fn lib_gc(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Gc {
    fn lib_gc(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match env.gc() {
            Ok(_) => Symbol::keyword("t"),
            Err(e) => return Err(e),
        };

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
