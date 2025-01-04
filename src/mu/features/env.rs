//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env interface
use crate::{
    core::{
        config::GcMode,
        core::CoreFnDef,
        direct::DirectTag,
        env::Env as Env_,
        exception::{self},
        frame::Frame,
        heap::HeapTypeInfo,
        indirect::IndirectTag,
        types::{Tag, Type},
    },
    features::feature::Feature,
    types::{
        cons::Cons, fixnum::Fixnum, function::Function, struct_::Struct, symbol::Symbol,
        vector::Vector,
    },
};
use futures::executor::block_on;
use futures_locks::RwLock;
use std::collections::HashMap;

lazy_static! {
    pub static ref ENV_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref ENV_FUNCTIONS: Vec<CoreFnDef> = vec![
        ("heap-info", 0, Feature::env_hp_info),
        ("heap-size", 1, Feature::env_hp_size),
        ("heap-stat", 0, Feature::env_hp_stat),
        ("state", 0, Feature::env_state),
    ];
    static ref INFOTYPE: Vec<Tag> = vec![
        Symbol::keyword("cons"),
        Symbol::keyword("func"),
        Symbol::keyword("stream"),
        Symbol::keyword("struct"),
        Symbol::keyword("symbol"),
        Symbol::keyword("vector"),
    ];
}

pub trait Env {
    fn feature() -> Feature;
    fn config(_: &Env_) -> Tag;
    fn heap_size(_: &Env_, tag: Tag) -> usize;
    fn heap_info(_: &Env_) -> (usize, usize);
    fn heap_type(_: &Env_, type_: Type) -> HeapTypeInfo;
    fn heap_stat(_: &Env_) -> Tag;
    fn ns_map(_: &Env_) -> Tag;
}

impl Env for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: Some(&ENV_SYMBOLS),
            functions: Some(&ENV_FUNCTIONS),
            namespace: "env".into(),
        }
    }

    fn heap_size(env: &Env_, tag: Tag) -> usize {
        match tag.type_of() {
            Type::Cons => Cons::heap_size(env, tag),
            Type::Function => Function::heap_size(env, tag),
            Type::Struct => Struct::heap_size(env, tag),
            Type::Symbol => Symbol::heap_size(env, tag),
            Type::Vector => Vector::heap_size(env, tag),
            _ => std::mem::size_of::<DirectTag>(),
        }
    }

    fn heap_info(env: &Env_) -> (usize, usize) {
        let heap_ref = block_on(env.heap.read());

        (heap_ref.page_size, heap_ref.npages)
    }

    fn heap_type(env: &Env_, type_: Type) -> HeapTypeInfo {
        let heap_ref = block_on(env.heap.read());
        let alloc_ref = block_on(heap_ref.alloc_map.read());
        let alloc_type = block_on(alloc_ref[type_ as usize].read());

        *alloc_type
    }

    fn heap_stat(env: &Env_) -> Tag {
        let (pagesz, npages) = <Feature as Env>::heap_info(env);

        let mut vec = vec![
            Symbol::keyword("heap"),
            Fixnum::with_or_panic(pagesz * npages),
            Fixnum::with_or_panic(npages),
            Fixnum::with_or_panic(0),
        ];

        for htype in INFOTYPE.iter() {
            let type_map =
                <Feature as Env>::heap_type(env, IndirectTag::to_indirect_type(*htype).unwrap());

            vec.extend(vec![
                *htype,
                Fixnum::with_or_panic(type_map.size),
                Fixnum::with_or_panic(type_map.total),
                Fixnum::with_or_panic(type_map.free),
            ])
        }

        Vector::from(vec).evict(env)
    }

    fn ns_map(env: &Env_) -> Tag {
        let ns_ref = block_on(env.ns_map.read());
        let vec = ns_ref
            .iter()
            .map(|(_, name, _)| Vector::from((*name).clone()).evict(env))
            .collect::<Vec<Tag>>();

        Cons::list(env, &vec)
    }

    fn config(env: &Env_) -> Tag {
        let alist = vec![
            Cons::cons(
                env,
                Vector::from("gcmode").evict(env),
                match env.config.gcmode {
                    GcMode::None => Vector::from("none").evict(env),
                    GcMode::Auto => Vector::from("auto").evict(env),
                    GcMode::Demand => Vector::from("demand").evict(env),
                },
            ),
            Cons::cons(
                env,
                Vector::from("npages").evict(env),
                Fixnum::with_or_panic(env.config.npages),
            ),
            Cons::cons(
                env,
                Vector::from("page_size").evict(env),
                Fixnum::with_or_panic(env.config.page_size),
            ),
            Cons::cons(
                env,
                Vector::from("version").evict(env),
                Vector::from(env.config.version.as_str()).evict(env),
            ),
        ];

        Cons::list(env, &alist)
    }
}

pub trait CoreFunction {
    fn env_hp_info(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn env_hp_size(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn env_hp_stat(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn env_state(_: &Env_, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn env_hp_stat(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        let (pagesz, npages) = <Feature as Env>::heap_info(env);

        let mut vec = vec![
            Symbol::keyword("heap"),
            Fixnum::with_or_panic(pagesz * npages),
            Fixnum::with_or_panic(npages),
            Fixnum::with_or_panic(0),
        ];

        for htype in INFOTYPE.iter() {
            let type_map =
                <Feature as Env>::heap_type(env, IndirectTag::to_indirect_type(*htype).unwrap());

            vec.extend(vec![
                *htype,
                Fixnum::with_or_panic(type_map.size),
                Fixnum::with_or_panic(type_map.total),
                Fixnum::with_or_panic(type_map.free),
            ])
        }

        fp.value = Vector::from(vec).evict(env);

        Ok(())
    }

    fn env_hp_info(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        let (page_size, npages) = <Feature as Env>::heap_info(env);

        let vec = vec![
            Symbol::keyword("bump"),
            Fixnum::with_or_panic(page_size),
            Fixnum::with_or_panic(npages),
        ];

        fp.value = Vector::from(vec).evict(env);

        Ok(())
    }

    fn env_hp_size(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_or_panic(<Feature as Env>::heap_size(env, fp.argv[0]));

        Ok(())
    }

    fn env_state(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        let alist = vec![
            Cons::cons(
                env,
                Vector::from("env-key").evict(env),
                *block_on(env.env_key.read()),
            ),
            Cons::cons(
                env,
                Vector::from("namespaces").evict(env),
                Self::ns_map(env),
            ),
            Cons::cons(env, Vector::from("config").evict(env), Self::config(env)),
            Cons::cons(
                env,
                Vector::from("heap-info").evict(env),
                Self::heap_stat(env),
            ),
            Cons::cons(
                env,
                Vector::from("heap-stat").evict(env),
                Self::heap_stat(env),
            ),
        ];

        fp.value = Cons::list(env, &alist);

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
