//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// namespaces
use {
    crate::{
        core::{
            apply::Apply as _,
            core::CoreFunctionDef,
            direct::DirectTag,
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            gc::GcContext,
            tag::Tag,
            type_::Type,
        },
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
    pub functions: Option<&'static Vec<CoreFunctionDef>>,
    pub hash: Option<&'static RwLock<HashMap<String, Tag>>>,
}

#[derive(Clone)]
pub enum Namespace {
    Static(Static),
    Dynamic(RwLock<HashMap<String, Tag>>),
}

impl Namespace {
    pub fn index_of(env: &Env, ns: Tag) -> usize {
        match ns.type_of() {
            Type::Struct if Struct::stype(env, ns).eq_(&Symbol::keyword("ns")) => {
                Fixnum::as_i64(Vector::ref_(env, Struct::vector(env, ns), 0).unwrap()) as usize
            }
            _ => panic!(),
        }
    }

    pub fn with(env: &Env, name: &str) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_map.write());
        let len = ns_ref.len();

        if ns_ref.iter().any(|(_, ns_name, _)| name == ns_name) {
            drop(ns_ref);

            return Err(Exception::new(
                env,
                Condition::Type,
                "mu:make-namespace",
                Vector::from(name).evict(env),
            ));
        }

        let ns = Struct::new(
            env,
            "ns",
            vec![
                Fixnum::with_u64_or_panic(len as u64),
                Vector::from(name).evict(env),
            ],
        )
        .evict(env);

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
        functab: Option<&'static Vec<CoreFunctionDef>>,
    ) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_map.write());
        let len = ns_ref.len();

        if ns_ref.iter().any(|(_, ns_name, _)| name == ns_name) {
            drop(ns_ref);

            return Err(Exception::new(
                env,
                Condition::Type,
                "mu:make-namespace",
                Vector::from(name).evict(env),
            ));
        }

        let ns = Struct::new(
            env,
            "ns",
            vec![
                Fixnum::with_u64_or_panic(len as u64),
                Vector::from(name).evict(env),
            ],
        )
        .evict(env);

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
                        None => return None,
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

    pub fn name(env: &Env, ns: Tag) -> Option<String> {
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
            Some(tag) => {
                if tag.is_empty() {
                    Some("".into())
                } else {
                    Some(tag.into())
                }
            }
            None => None,
        }
    }

    pub fn intern(env: &Env, ns: Tag, name: String, value: Tag) -> Option<Tag> {
        if env.keyword_ns.eq_(&ns) {
            if name.len() > DirectTag::DIRECT_STR_MAX {
                return None;
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
                let symbol = Symbol::new(env, ns, &name, value).evict(env);
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
                        let name = Vector::as_string(env, Symbol::name(env, symbol));
                        let mut hash = block_on(match ns_map {
                            Namespace::Static(static_) => match static_.hash {
                                Some(hash) => hash.write(),
                                None => return None,
                            },
                            Namespace::Dynamic(hash) => hash.write(),
                        });

                        hash.insert(name, symbol);
                    }
                    None => return None,
                }

                Some(symbol)
            }
        }
    }

    pub fn intern_static(env: &Env, ns: Tag, name: String, value: Tag) -> Option<Tag> {
        let symbol = Symbol::new(env, ns, &name, value).evict(env);
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
                let name = Vector::as_string(env, Symbol::name(env, symbol));
                let mut hash = block_on(match ns_map {
                    Namespace::Static(static_) => match static_.hash {
                        Some(hash) => hash.write(),
                        None => return None,
                    },
                    Namespace::Dynamic(_) => return None,
                });

                hash.insert(name, symbol);
            }
            None => return None,
        }

        Some(symbol)
    }
}

pub trait CoreFunction {
    fn mu_find(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_find_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_intern(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_make_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns_name(env: &Env, fp: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Namespace {
    fn mu_intern(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let mut ns = fp.argv[0];
        let name = fp.argv[1];
        let value = fp.argv[2];

        if Tag::null_(&ns) {
            ns = env.null_ns
        }

        if !Struct::stype(env, ns).eq_(&Symbol::keyword("ns")) {
            return Err(Exception::new(env, Condition::Type, "mu:intern", ns));
        }

        fp.value = match Self::intern(env, ns, Vector::as_string(env, name), value) {
            Some(ns) => ns,
            None => return Err(Exception::new(env, Condition::Range, "mu:intern", name)),
        };

        Ok(())
    }

    fn mu_make_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:make-namespace", &[Type::String], fp)?;

        fp.value = Self::with(env, &Vector::as_string(env, fp.argv[0]))?;

        Ok(())
    }

    fn mu_ns_name(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let mut ns = fp.argv[0];

        if Tag::null_(&ns) {
            ns = env.null_ns
        }

        if !Struct::stype(env, ns).eq_(&Symbol::keyword("ns")) {
            return Err(Exception::new(
                env,
                Condition::Type,
                "mu:namespace-name",
                ns,
            ));
        }

        fp.value = Vector::from(Self::name(env, ns).unwrap()).evict(env);

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
        let mut ns_tag = fp.argv[0];
        let name = fp.argv[1];

        if Tag::null_(&ns_tag) {
            ns_tag = env.null_ns
        }

        match name.type_of() {
            Type::Vector if Vector::type_of(env, name) == Type::Char => {
                if !Struct::stype(env, ns_tag).eq_(&Symbol::keyword("ns")) {
                    return Err(Exception::new(env, Condition::Type, "mu:intern", ns_tag));
                }
            }
            _ => Err(Exception::new(env, Condition::Type, "mu:intern", ns_tag))?,
        }

        let ns_ref = block_on(env.ns_map.read());
        fp.value = match ns_ref.iter().find_map(
            |(tag, _, ns_map)| {
                if ns_tag.eq_(tag) {
                    Some(ns_map)
                } else {
                    None
                }
            },
        ) {
            Some(_) => match Self::find_symbol(env, ns_tag, &Vector::as_string(env, name)) {
                Some(sym) => sym,
                None => Tag::nil(),
            },
            None => {
                drop(ns_ref);

                Err(Exception::new(env, Condition::Type, "mu:find", ns_tag))?
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn namespace() {
        assert!(true)
    }
}
