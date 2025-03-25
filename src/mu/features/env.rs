//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env interface
use {
    crate::{
        core::{
            core::{Core, CoreFnDef, VERSION},
            direct::DirectTag,
            env::Env as Env_,
            exception::{self},
            frame::Frame,
            heap::{HeapAllocator, HeapTypeInfo},
            indirect::IndirectTag,
            types::{Tag, Type},
        },
        features::feature::Feature,
        types::{
            cons::Cons, fixnum::Fixnum, function::Function, struct_::Struct, symbol::Symbol,
            vector::Vector,
        },
    },
    futures::executor::block_on,
    futures_locks::RwLock,
    std::collections::HashMap,
};

lazy_static! {
    pub static ref ENV_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref ENV_FUNCTIONS: Vec<CoreFnDef> = vec![
        ("core", 0, Feature::env_core),
        ("env", 0, Feature::env_env),
        ("heap-free", 0, Feature::env_hp_free),
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
    fn heap_free(_: &Env_) -> usize;
    fn heap_size(_: &Env_, tag: Tag) -> usize;
    fn heap_info(_: &Env_) -> (&str, usize, usize);
    fn heap_stat(_: &Env_) -> Tag;
    fn heap_type(_: &Env_, type_: Type) -> HeapTypeInfo;
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

    fn heap_info(env: &Env_) -> (&str, usize, usize) {
        let heap_ref = block_on(env.heap.read());

        ("bump", heap_ref.page_size, heap_ref.npages)
    }

    fn heap_free(env: &Env_) -> usize {
        HeapAllocator::heap_free(env)
    }

    fn heap_type(env: &Env_, type_: Type) -> HeapTypeInfo {
        let heap_ref = block_on(env.heap.read());

        heap_ref.alloc_map[type_ as usize]
    }

    fn heap_stat(env: &Env_) -> Tag {
        let (_heap_type, pagesz, npages) = <Feature as Env>::heap_info(env);

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
}

pub trait CoreFunction {
    fn env_core(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn env_env(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn env_hp_free(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn env_hp_info(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn env_hp_room(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn env_hp_size(_: &Env_, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn env_hp_free(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_or_panic(<Feature as Env>::heap_free(env));

        Ok(())
    }

    fn env_hp_info(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        let heap_ref = block_on(env.heap.read());
        println!("type           :bump");
        println!("page-size      {}", heap_ref.page_size);
        println!("npages         {}", heap_ref.npages);
        println!("size           {}", heap_ref.size);
        println!("alloc-barrier  {}", heap_ref.alloc_barrier);
        println!("free-space     {}", heap_ref.free_space);
        println!("gc-threshold   {}", heap_ref.gc_threshold);
        println!("gc-allocated   {}", heap_ref.gc_allocated);

        fp.value = Tag::nil();

        Ok(())
    }

    fn env_hp_room(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        let mut vec = vec![];

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

    fn env_hp_size(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_or_panic(<Feature as Env>::heap_size(env, fp.argv[0]));

        Ok(())
    }

    fn env_env(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        let alist = vec![
            Cons::cons(
                env,
                Vector::from("env-key").evict(env),
                *block_on(env.env_key.read()),
            ),
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
                Vector::from("heap-stat").evict(env),
                Self::heap_stat(env),
            ),
        ];

        fp.value = Cons::list(env, &alist);

        Ok(())
    }

    fn env_core(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        let alist = vec![
            Cons::cons(
                env,
                Vector::from("version").evict(env),
                Vector::from(VERSION).evict(env),
            ),
            Cons::cons(
                env,
                Vector::from("features").evict(env),
                Core::features_as_list(env),
            ),
            Cons::cons(
                env,
                Vector::from("envs").evict(env),
                Core::envs_as_list(env),
            ),
            Cons::cons(env, Vector::from("streams").evict(env), Core::nstreams()),
        ];

        fp.value = Cons::list(env, &alist);

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
