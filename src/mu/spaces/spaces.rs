//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// spaces
#[rustfmt::skip]
#[allow (unused_imports)]
use {
    crate::{
        core::{
            direct::{DirectTag, DirectImage},
            env::Env,
            tag::Tag,
            type_::Type,
        },
        types::{
            async_::Async,
            cons::Cons,
            function::Function,
            symbol::Symbol,
        },
        spaces::{heap::Heap, cache::Cache},
    },
    // super::{heap::Heap, cache::Cache},
};

/*
pub enum Space {
    Heap,
    Cache
}

impl Space for Heap {
    fn allocate(&self) -> Tag {
        Tag::nil()
    }
}

impl Space for Cache {
    fn allocate(&self) -> Tag {
        Tag::nil()
    }
}

pub trait Space {
    fn allocate(&self) -> Tag;

    fn alloc<T: Default>(env: &Env, space: Space) -> Tag {
        .allocate()
    }
}
*/
