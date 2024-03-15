//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu gc
//!    Mu
#[allow(unused_imports)]
use {
    crate::{
        allocators::{bump_allocator::BumpAllocator, stack_allocator::StackAllocator},
        core::{
            config::Config,
            direct::DirectTag,
            exception,
            frame::Frame,
            indirect::{self, IndirectTag},
            mu::{Core as _, Mu},
            types::{Tag, Type},
        },
        types::{
            char::{Char, Core as _},
            cons::{Cons, Core as _},
            fixnum::{Core as _, Fixnum},
            float::{Core as _, Float},
            function::{Core as _, Function},
            stream::{Core as _, Stream},
            struct_::{Core as _, Struct},
            symbol::{Core as _, Symbol},
            vecimage::{TypedVec, VecType},
            vector::{Core as _, Vector},
        },
    },
    memmap,
    modular_bitfield::specifiers::{B11, B4},
    num_enum::TryFromPrimitive,
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
    fn gc_asyncs(&self);
    fn gc_namespaces(&self);
    fn mark_image(&self, _: Tag) -> Option<bool>;
}

impl Core for Mu {
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
        self.gc_asyncs();

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

    fn gc_asyncs(&self) {
        let async_index_ref = block_on(self.async_index.read());
        for (_name, _hash) in async_index_ref.iter() {
            // mu.mark(*context)
        }
    }

    fn gc_namespaces(&self) {
        let ns_index_ref = block_on(self.ns_index.read());

        for (_, hash) in ns_index_ref.iter() {
            let hash_ref = block_on(hash.1.read());

            for (_, symbol) in hash_ref.iter() {
                self.mark(*symbol)
            }
        }
    }
}

pub trait MuFunction {
    fn mu_gc(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Gc {
    fn mu_gc(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match mu.gc() {
            Ok(_) => Symbol::keyword("t"),
            Err(e) => return Err(e),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mu() {
        assert_eq!(2 + 2, 4);
    }
}
