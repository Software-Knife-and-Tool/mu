//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// namespaces
use {
    crate::{
        core_::{
            apply::Apply as _,
            core::CoreFnDef,
            direct::DirectTag,
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            tag::Tag,
            type_::Type,
        },
        spaces::gc::GcContext,
        types::{
            fixnum::Fixnum,
            struct_::Struct,
            symbol::{Gc as _, Symbol},
            vector::Vector,
        },
    },
    futures_lite::future::block_on,
    futures_locks::RwLock,
    std::{collections::HashMap, str},
};

pub trait Gc {
    #[allow(dead_code)]
    fn gc(&mut self, _: &mut GcContext, _: &Env);
}

impl Gc for Namespace {
    #[allow(dead_code)]
    fn gc(&mut self, gc: &mut GcContext, env: &Env) {
        let hash_ref = block_on(match self {
            Namespace::Static(static_) => match static_.hash {
                Some(hash) => hash.read(),
                None => return,
            },
            Namespace::Dynamic(ref hash) => hash.read(),
        });

        for (_, symbol) in hash_ref.iter() {
            Symbol::mark(gc, env, *symbol)
        }
    }
}

#[derive(Clone)]
pub struct Static {
    pub functions: Option<&'static [CoreFnDef]>,
    pub hash: Option<&'static RwLock<HashMap<String, Tag>>>,
}

#[derive(Clone)]
pub enum Namespace {
    Static(Static),
    Dynamic(RwLock<HashMap<String, Tag>>),
}

impl Namespace {
    pub fn with(env: &Env, name: &str) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_map.write());
        let id = ns_ref.len();

        if ns_ref.iter().any(|(_, ns_name, _)| name == ns_name) {
            drop(ns_ref);

            return Err(Exception::new(
                env,
                Condition::Type,
                "mu:make-namespace",
                Vector::from(name).with_heap(env),
            ));
        }

        let ns = Struct::new(
            env,
            "ns",
            vec![
                Fixnum::with_u64_or_panic(id as u64),
                Vector::from(name).with_heap(env),
            ],
        )
        .with_heap(env);

        ns_ref.push((
            ns,
            name.into(),
            Namespace::Dynamic(RwLock::new(HashMap::<String, Tag>::new())),
        ));

        Ok(ns)
    }

    pub fn with_static(
        env: &Env,
        name: &str,
        ns_map: Option<&'static RwLock<HashMap<String, Tag>>>,
        functab: Option<&'static [CoreFnDef]>,
    ) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_map.write());
        let id = ns_ref.len();

        if ns_ref.iter().any(|(_, ns_name, _)| name == ns_name) {
            drop(ns_ref);

            return Err(Exception::new(
                env,
                Condition::Type,
                "mu:make-namespace",
                Vector::from(name).with_heap(env),
            ));
        }

        let ns = Struct::new(
            env,
            "ns",
            vec![
                Fixnum::with_u64_or_panic(id as u64),
                Vector::from(name).with_heap(env),
            ],
        )
        .with_heap(env);

        ns_ref.push((
            ns,
            name.into(),
            Namespace::Static(Static {
                functions: functab,
                hash: ns_map,
            }),
        ));

        Ok(ns)
    }

    pub fn is_namespace(env: &Env, ns: Tag) -> bool {
        if ns.type_of() == Type::Struct {
            let (stype, _) = Struct::destruct(env, ns);

            stype.eq_(&Symbol::keyword("ns"))
        } else {
            false
        }
    }

    pub fn find_symbol(env: &Env, ns: Tag, name: &str) -> Option<Tag> {
        let ns_ref = block_on(env.ns_map.read());

        match ns_ref.iter().find_map(
            |(tag, _, ns_cache)| {
                if ns.eq_(tag) {
                    Some(ns_cache)
                } else {
                    None
                }
            },
        ) {
            Some(ns_cache) => {
                let hash = block_on(match ns_cache {
                    Namespace::Static(static_) => match static_.hash {
                        Some(hash) => hash.read(),
                        None => None?,
                    },
                    Namespace::Dynamic(hash) => hash.read(),
                });

                if hash.contains_key(name) {
                    Some(hash[name])
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn find(env: &Env, name: &str) -> Option<Tag> {
        let ns_ref = block_on(env.ns_map.read());

        ns_ref
            .iter()
            .find_map(
                |(tag, ns_name, _)| {
                    if name == ns_name {
                        Some(tag)
                    } else {
                        None
                    }
                },
            )
            .copied()
    }

    pub fn name(env: &Env, ns: Tag) -> String {
        let ns_ref = block_on(env.ns_map.read());

        match ns_ref.iter().find_map(
            |(tag, ns_name, _)| {
                if ns.eq_(tag) {
                    Some(ns_name)
                } else {
                    None
                }
            },
        ) {
            Some(tag) => tag.into(),
            None => panic!(),
        }
    }

    pub fn intern(env: &Env, ns: Tag, name: String, value: Tag) -> Option<Tag> {
        if env.keyword_ns.eq_(&ns) {
            if name.len() > DirectTag::DIRECT_STR_MAX {
                None?
            }

            return Some(Symbol::keyword(&name));
        }

        match Self::find_symbol(env, ns, &name) {
            Some(symbol) => {
                if Symbol::is_bound(env, symbol) {
                    Some(symbol)
                } else {
                    let image = Symbol::to_image(env, symbol);

                    let slices: &[[u8; 8]] = &[
                        image.namespace.as_slice(),
                        image.name.as_slice(),
                        value.as_slice(),
                    ];

                    let offset = match symbol {
                        Tag::Indirect(heap) => heap.image_id(),
                        _ => panic!(),
                    } as usize;

                    let mut heap_ref = block_on(env.heap.write());

                    heap_ref.write_image(slices, offset);

                    Some(symbol)
                }
            }
            None => {
                let symbol = Symbol::new(env, ns, &name, value).with_heap(env);
                let ns_ref = block_on(env.ns_map.read());

                match ns_ref.iter().find_map(
                    |(tag, _, ns_map)| {
                        if ns.eq_(tag) {
                            Some(ns_map)
                        } else {
                            None
                        }
                    },
                ) {
                    Some(ns_map) => {
                        let name = Vector::as_string(env, Symbol::destruct(env, symbol).1);
                        let mut hash = block_on(match ns_map {
                            Namespace::Static(static_) => match static_.hash {
                                Some(hash) => hash.write(),
                                None => None?,
                            },
                            Namespace::Dynamic(hash) => hash.write(),
                        });

                        hash.insert(name, symbol);
                    }
                    None => None?,
                }

                Some(symbol)
            }
        }
    }

    pub fn intern_static(env: &Env, ns: Tag, name: String, value: Tag) -> Option<Tag> {
        let symbol = Symbol::new(env, ns, &name, value).with_heap(env);
        let ns_ref = block_on(env.ns_map.read());

        match ns_ref.iter().find_map(
            |(tag, _, ns_map)| {
                if ns.eq_(tag) {
                    Some(ns_map)
                } else {
                    None
                }
            },
        ) {
            Some(ns_map) => {
                let name = Vector::as_string(env, Symbol::destruct(env, symbol).1);
                let mut hash = block_on(match ns_map {
                    Namespace::Static(static_) => match static_.hash {
                        Some(hash) => hash.write(),
                        None => None?,
                    },
                    Namespace::Dynamic(_) => None?,
                });

                hash.insert(name, symbol);
            }
            None => None?,
        }

        Some(symbol)
    }
}

pub trait CoreFn {
    fn mu_find(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_find_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_intern(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_make_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns_name(env: &Env, fp: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Namespace {
    fn mu_intern(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:intern", &[Type::T, Type::String, Type::T], fp)?;

        let ns = fp.argv[0];
        let name = fp.argv[1];
        let value = fp.argv[2];

        if !Self::is_namespace(env, ns) {
            Err(Exception::new(env, Condition::Type, "mu:intern", ns))?
        }

        fp.value = match Self::intern(env, ns, Vector::as_string(env, name), value) {
            Some(ns) => ns,
            None => Err(Exception::new(env, Condition::Range, "mu:intern", name))?,
        };

        Ok(())
    }

    fn mu_make_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:make-namespace", &[Type::String], fp)?;

        fp.value = Self::with(env, &Vector::as_string(env, fp.argv[0]))?;

        Ok(())
    }

    fn mu_ns_name(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];

        if !Self::is_namespace(env, ns) {
            Err(Exception::new(
                env,
                Condition::Type,
                "mu:namespace-name",
                ns,
            ))?
        }

        fp.value = Vector::from(Self::name(env, ns)).with_heap(env);

        Ok(())
    }

    fn mu_find_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:find-namespace", &[Type::String], fp)?;

        fp.value = match Self::find(env, &Vector::as_string(env, fp.argv[0])) {
            Some(ns) => ns,
            None => Tag::nil(),
        };

        Ok(())
    }

    fn mu_find(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:find", &[Type::T, Type::String], fp)?;

        let ns_tag = fp.argv[0];
        let name = fp.argv[1];

        if !Self::is_namespace(env, ns_tag) {
            Err(Exception::new(env, Condition::Type, "mu:find", ns_tag))?
        }

        match name.type_of() {
            Type::Vector if Vector::type_of(env, name) == Type::Char => {
                let ns_ref = block_on(env.ns_map.read());
                fp.value =
                    match ns_ref.iter().find_map(
                        |(tag, _, ns_map)| {
                            if ns_tag.eq_(tag) {
                                Some(ns_map)
                            } else {
                                None
                            }
                        },
                    ) {
                        Some(_) => {
                            match Self::find_symbol(env, ns_tag, &Vector::as_string(env, name)) {
                                Some(sym) => sym,
                                None => Tag::nil(),
                            }
                        }
                        None => {
                            drop(ns_ref);

                            Err(Exception::new(env, Condition::Type, "mu:find", ns_tag))?
                        }
                    };
            }
            _ => Err(Exception::new(env, Condition::Type, "mu:find", ns_tag))?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn namespace_test() {
        assert!(true)
    }
}
