//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu heap interface
//!    Mu
#[allow(unused_imports)]
use {
    crate::{
        allocators::{
            bump_allocator::{BumpAllocator, BumpAllocatorIter},
            stack_allocator::{StackAllocator, StackAllocatorIter},
        },
        core::{
            allocator::AllocTypeInfo,
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
    std::fmt,
};

use futures::executor::block_on;

lazy_static! {
    static ref INFOTYPE: Vec<Tag> = vec![
        Symbol::keyword("cons"),
        Symbol::keyword("func"),
        Symbol::keyword("stream"),
        Symbol::keyword("struct"),
        Symbol::keyword("symbol"),
        Symbol::keyword("vector"),
    ];
}

pub struct Heap {
    pub allocator: BumpAllocator,
}

pub trait Core {
    fn heap_size(_: &Mu, _: Tag) -> usize;
    fn heap_info(_: &Mu) -> (usize, usize);
    fn heap_type(_: &Mu, _: Type) -> AllocTypeInfo;
}

impl Core for Heap {
    fn heap_size(mu: &Mu, tag: Tag) -> usize {
        match tag.type_of() {
            Type::Cons => Cons::heap_size(mu, tag),
            Type::Function => Function::heap_size(mu, tag),
            Type::Struct => Struct::heap_size(mu, tag),
            Type::Symbol => Symbol::heap_size(mu, tag),
            Type::Vector => Vector::heap_size(mu, tag),
            _ => std::mem::size_of::<DirectTag>(),
        }
    }

    fn heap_info(mu: &Mu) -> (usize, usize) {
        let heap_ref = block_on(mu.heap.read());

        (heap_ref.page_size, heap_ref.npages)
    }

    fn heap_type(mu: &Mu, type_: Type) -> AllocTypeInfo {
        let heap_ref = block_on(mu.heap.read());
        let alloc_ref = block_on(heap_ref.alloc_map.read());
        let alloc_type = block_on(alloc_ref[type_ as usize].read());

        *alloc_type
    }
}

pub trait MuFunction {
    fn libcore_hp_info(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn libcore_hp_size(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn libcore_hp_stat(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Heap {
    fn libcore_hp_stat(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let (pagesz, npages) = Heap::heap_info(mu);

        let mut vec = vec![
            Symbol::keyword("heap"),
            Tag::from((pagesz * npages) as i64),
            Tag::from(npages as i64),
            Tag::from(0_i64),
        ];

        for htype in INFOTYPE.iter() {
            let type_map = Self::heap_type(
                mu,
                <IndirectTag as indirect::Core>::to_indirect_type(*htype).unwrap(),
            );

            vec.push(*htype);
            vec.push(Tag::from(type_map.size as i64));
            vec.push(Tag::from(type_map.total as i64));
            vec.push(Tag::from(type_map.free as i64));
        }

        fp.value = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu);
        Ok(())
    }

    fn libcore_hp_info(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let (page_size, npages) = Self::heap_info(mu);

        let vec = vec![
            Symbol::keyword("bump"),
            Tag::from(page_size as i64),
            Tag::from(npages as i64),
        ];

        fp.value = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu);
        Ok(())
    }

    fn libcore_hp_size(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Tag::from(Self::heap_size(mu, fp.argv[0]) as i64);

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
