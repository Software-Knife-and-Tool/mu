//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image interface
use {
    crate::{
        core::{
            core::{Core, CoreFnDef, VERSION},
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
    },
    futures::executor::block_on,
    futures_locks::RwLock,
    std::collections::HashMap,
};

lazy_static! {
    pub static ref IMAGE_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref IMAGE_FUNCTIONS: Vec<CoreFnDef> = vec![
        ("heap-size", 1, Feature::image_hp_size),
        ("heap-stat", 0, Feature::image_hp_stat),
        ("core", 0, Feature::image_core),
        ("env", 0, Feature::image_env),
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

pub trait Image {
    fn feature() -> Feature;
    fn heap_size(_: &Env_, tag: Tag) -> usize;
    fn heap_info(_: &Env_) -> (usize, usize);
    fn heap_type(_: &Env_, type_: Type) -> HeapTypeInfo;
    fn heap_stat(_: &Env_) -> Tag;
    fn ns_map(_: &Env_) -> Tag;
}

impl Image for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: Some(&IMAGE_SYMBOLS),
            functions: Some(&IMAGE_FUNCTIONS),
            namespace: "image".into(),
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
        let type_ref = block_on(alloc_ref[type_ as usize].read());

        *type_ref
    }

    fn heap_stat(env: &Env_) -> Tag {
        let (pagesz, npages) = <Feature as Image>::heap_info(env);

        let mut vec = vec![
            Symbol::keyword("heap"),
            Fixnum::with_or_panic(pagesz * npages),
            Fixnum::with_or_panic(npages),
            Fixnum::with_or_panic(0),
        ];

        for htype in INFOTYPE.iter() {
            let type_map =
                <Feature as Image>::heap_type(env, IndirectTag::to_indirect_type(*htype).unwrap());

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
    fn image_core(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn image_hp_size(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn image_hp_stat(_: &Env_, _: &mut Frame) -> exception::Result<()>;
    fn image_env(_: &Env_, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn image_hp_stat(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        let (pagesz, npages) = <Feature as Image>::heap_info(env);

        let mut vec = vec![
            Symbol::keyword("heap"),
            Fixnum::with_or_panic(pagesz * npages),
            Fixnum::with_or_panic(npages),
            Fixnum::with_or_panic(0),
        ];

        for htype in INFOTYPE.iter() {
            let type_map =
                <Feature as Image>::heap_type(env, IndirectTag::to_indirect_type(*htype).unwrap());

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

    fn image_hp_size(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::with_or_panic(<Feature as Image>::heap_size(env, fp.argv[0]));

        Ok(())
    }

    fn image_env(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
        let (page_size, npages) = <Feature as Image>::heap_info(env);

        let heap_info = vec![
            Symbol::keyword("bump"),
            Fixnum::with_or_panic(page_size),
            Fixnum::with_or_panic(npages),
        ];

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
                Vector::from("heap-info").evict(env),
                Cons::list(env, &heap_info),
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

    fn image_core(env: &Env_, fp: &mut Frame) -> exception::Result<()> {
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
