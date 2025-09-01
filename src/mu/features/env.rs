//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env feature
#[rustfmt::skip]
use {
    crate::{
        core::{
            cache::Cache,
            core::CoreFunctionDef,
            direct::DirectTag,
            env,
            exception::{self},
            frame::Frame,
            heap::HeapTypeInfo,
            indirect::IndirectTag,
            tag::{Tag},
            type_::{Type},
        },
        features::feature::Feature,
        types::{
            async_::Async,
            cons::Cons,
            fixnum::Fixnum,
            function::Function,
            struct_::Struct,
            symbol::Symbol,
            vector::Vector,
        },
    },
    futures_lite::future::block_on,
    futures_locks::RwLock,
    std::collections::HashMap,
};

lazy_static! {
    pub static ref ENV_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref ENV_FUNCTIONS: Vec<CoreFunctionDef> = vec![
        ("dynamic-room", 0, Feature::env_images_room),
        ("env", 0, Feature::env_env),
        ("heap-info", 0, Feature::env_hp_info),
        ("heap-room", 0, Feature::env_hp_room),
        ("heap-size", 1, Feature::env_hp_size),
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
    fn heap_room(_: &env::Env) -> Tag;
    fn heap_size(_: &env::Env, tag: Tag) -> usize;
    fn heap_type(_: &env::Env, type_: Type) -> HeapTypeInfo;
    fn images_room(_: &env::Env) -> Tag;
    fn ns_map(_: &env::Env) -> Tag;
}

impl Env for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: Some(&ENV_SYMBOLS),
            functions: Some(&ENV_FUNCTIONS),
            namespace: "mu/env".into(),
        }
    }

    fn heap_size(env: &env::Env, tag: Tag) -> usize {
        match tag.type_of() {
            Type::Async => Async::heap_size(env, tag),
            Type::Cons => Cons::heap_size(env, tag),
            Type::Function => Function::heap_size(env, tag),
            Type::Struct => Struct::heap_size(env, tag),
            Type::Symbol => Symbol::heap_size(env, tag),
            Type::Vector => Vector::heap_size(env, tag),
            _ => std::mem::size_of::<DirectTag>(),
        }
    }

    fn heap_type(env: &env::Env, type_: Type) -> HeapTypeInfo {
        let heap_ref = block_on(env.heap.read());

        heap_ref.alloc_map[type_ as usize]
    }

    fn heap_room(env: &env::Env) -> Tag {
        let mut vec = Vec::new();

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

    fn images_room(env: &env::Env) -> Tag {
        let mut vec = Vec::new();

        for htype in INFOTYPE.iter() {
            let type_map = Cache::type_info(env, IndirectTag::to_indirect_type(*htype).unwrap());

            match type_map {
                None => (),
                Some(type_map) => vec.extend(vec![
                    *htype,
                    Fixnum::with_or_panic(type_map.size),
                    Fixnum::with_or_panic(type_map.total),
                ]),
            }
        }

        Vector::from(vec).evict(env)
    }

    fn ns_map(env: &env::Env) -> Tag {
        let ns_ref = block_on(env.ns_map.read());
        let vec = ns_ref
            .iter()
            .map(|(_, name, _)| Vector::from((*name).clone()).evict(env))
            .collect::<Vec<Tag>>();

        Cons::list(env, &vec)
    }
}

pub trait CoreFunction {
    fn env_env(_: &env::Env, _: &mut Frame) -> exception::Result<()>;
    fn env_hp_info(_: &env::Env, _: &mut Frame) -> exception::Result<()>;
    fn env_hp_room(_: &env::Env, _: &mut Frame) -> exception::Result<()>;
    fn env_hp_size(_: &env::Env, _: &mut Frame) -> exception::Result<()>;
    fn env_images_room(_: &env::Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn env_hp_info(env: &env::Env, fp: &mut Frame) -> exception::Result<()> {
        let heap_ref = block_on(env.heap.read());

        println!("type           :bump");
        println!("page-size      {}", heap_ref.page_size);
        println!("npages         {}", heap_ref.npages);
        println!("size           {}", heap_ref.size);
        println!("alloc-barrier  {}", heap_ref.alloc_barrier);
        println!("free-space     {}", heap_ref.free_space);
        println!("gc-allocated   {}", heap_ref.gc_allocated);

        fp.value = Tag::nil();

        Ok(())
    }

    fn env_images_room(env: &env::Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Self::images_room(env);

        Ok(())
    }

    fn env_hp_room(env: &env::Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Self::heap_room(env);

        Ok(())
    }

    fn env_hp_size(env: &env::Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_or_panic(<Feature as Env>::heap_size(env, fp.argv[0]));

        Ok(())
    }

    fn env_env(env: &env::Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Cons::list(
            env,
            &[
                Cons::cons(
                    env,
                    Vector::from("config").evict(env),
                    env.config.as_list(env),
                ),
                Cons::cons(
                    env,
                    Vector::from("namespaces").evict(env),
                    Self::ns_map(env),
                ),
                Cons::cons(
                    env,
                    Vector::from("heap-room").evict(env),
                    Self::heap_room(env),
                ),
            ],
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
