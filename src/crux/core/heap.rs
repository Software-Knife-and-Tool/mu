//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env heap interface
//!    Env
use crate::{
    core::{
        direct::DirectTag,
        env::Env,
        exception,
        frame::Frame,
        indirect::{self, IndirectTag},
        types::{Tag, Type},
    },
    images::allocator::AllocTypeInfo,
    images::bump_allocator::BumpAllocator,
    types::{
        cons::{Cons, Core as _},
        function::{Core as _, Function},
        indirect_vector::{TypedVector, VecType},
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vector::{Core as _, Vector},
    },
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
    fn heap_size(_: &Env, _: Tag) -> usize;
    fn heap_info(_: &Env) -> (usize, usize);
    fn heap_type(_: &Env, _: Type) -> AllocTypeInfo;
}

impl Core for Heap {
    fn heap_size(env: &Env, tag: Tag) -> usize {
        match tag.type_of() {
            Type::Cons => Cons::heap_size(env, tag),
            Type::Function => Function::heap_size(env, tag),
            Type::Struct => Struct::heap_size(env, tag),
            Type::Symbol => Symbol::heap_size(env, tag),
            Type::Vector => Vector::heap_size(env, tag),
            _ => std::mem::size_of::<DirectTag>(),
        }
    }

    fn heap_info(env: &Env) -> (usize, usize) {
        let heap_ref = block_on(env.heap.read());

        (heap_ref.page_size, heap_ref.npages)
    }

    fn heap_type(env: &Env, type_: Type) -> AllocTypeInfo {
        let heap_ref = block_on(env.heap.read());
        let alloc_ref = block_on(heap_ref.alloc_map.read());
        let alloc_type = block_on(alloc_ref[type_ as usize].read());

        *alloc_type
    }
}

pub trait CoreFunction {
    fn crux_hp_info(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_hp_size(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_hp_stat(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Heap {
    fn crux_hp_stat(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let (pagesz, npages) = Heap::heap_info(env);

        let mut vec = vec![
            Symbol::keyword("heap"),
            (pagesz * npages).into(),
            npages.into(),
            0_i64.into(),
        ];

        for htype in INFOTYPE.iter() {
            let type_map = Self::heap_type(
                env,
                <IndirectTag as indirect::Core>::to_indirect_type(*htype).unwrap(),
            );

            vec.push(*htype);
            vec.push(type_map.size.into());
            vec.push(type_map.total.into());
            vec.push(type_map.free.into());
        }

        fp.value = TypedVector::<Vec<Tag>> { vec }.vec.to_vector().evict(env);

        Ok(())
    }

    fn crux_hp_info(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let (page_size, npages) = Self::heap_info(env);

        let vec = vec![Symbol::keyword("bump"), page_size.into(), npages.into()];

        fp.value = TypedVector::<Vec<Tag>> { vec }.vec.to_vector().evict(env);

        Ok(())
    }

    fn crux_hp_size(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Self::heap_size(env, fp.argv[0]).into();

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
