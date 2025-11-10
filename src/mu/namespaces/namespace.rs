//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// namespaces
use {
    crate::{
        core::{
            apply::Apply as _,
            direct::DirectTag,
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            tag::Tag,
            type_::Type,
        },
        namespaces::gc::GcContext,
        types::{
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
            Namespace::Static(static_) => match &static_ {
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
pub enum Namespace {
    Static(Option<RwLock<HashMap<String, Tag>>>),
    Dynamic(RwLock<HashMap<String, Tag>>),
}

impl Namespace {
    pub fn with(env: &Env, name: &str) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_map.write());

        if ns_ref.contains_key(name) {
            drop(ns_ref);

            return Err(Exception::new(
                env,
                Condition::Type,
                "mu:make-namespace",
                Vector::from(name).with_heap(env),
            ));
        }

        let ns = Struct::new(env, "ns", vec![Vector::from(name).with_heap(env)]).with_heap(env);

        ns_ref.insert(
            name.to_string(),
            (
                ns,
                Namespace::Dynamic(RwLock::new(HashMap::<String, Tag>::new())),
            ),
        );

        Ok(ns)
    }

    pub fn with_static(
        env: &Env,
        name: &str,
        ns_map: Option<RwLock<HashMap<String, Tag>>>,
    ) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_map.write());

        if ns_ref.contains_key(name) {
            drop(ns_ref);

            return Err(Exception::new(
                env,
                Condition::Type,
                "mu:make-namespace",
                Vector::from(name).with_heap(env),
            ));
        }

        let ns = Struct::new(env, "ns", vec![Vector::from(name).with_heap(env)]).with_heap(env);

        ns_ref.insert(name.to_string(), (ns, Namespace::Static(ns_map)));

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
        let ns_map =
            &ns_ref[&Vector::as_string(env, Vector::ref_(env, Struct::destruct(env, ns).1, 0)?)];

        let hash = block_on(match &ns_map.1 {
            Namespace::Static(static_) => match &static_ {
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

    pub fn find_ns(env: &Env, name: &str) -> Option<Tag> {
        let ns_ref = block_on(env.ns_map.read());
        let ns_desc = ns_ref.get(name)?;

        Some(ns_desc.0)
    }

    pub fn name(env: &Env, ns: Tag) -> String {
        Vector::as_string(
            env,
            Vector::ref_(env, Struct::destruct(env, ns).1, 0).unwrap(),
        )
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

                match &ns_ref[&Self::name(env, ns)].1 {
                    Namespace::Static(static_) => match static_ {
                        Some(hash) => {
                            let name = Vector::as_string(env, Symbol::destruct(env, symbol).1);

                            let mut hash_ref = block_on(hash.write());

                            hash_ref.insert(name, symbol);
                        }
                        None => panic!(),
                    },
                    Namespace::Dynamic(hash) => {
                        let mut hash_ref = block_on(hash.write());

                        hash_ref.insert(name, symbol);
                    }
                }

                Some(symbol)
            }
        }
    }

    pub fn intern_static(env: &Env, ns: Tag, name: String, value: Tag) {
        let symbol = Symbol::new(env, ns, &name, value).with_heap(env);
        let ns_ref = block_on(env.ns_map.read());
        let ns_name = Vector::as_string(
            env,
            Vector::ref_(env, Struct::destruct(env, ns).1, 0).unwrap(),
        );

        match &ns_ref[&ns_name].1 {
            Namespace::Static(ns_map) => match &ns_map {
                Some(hash) => {
                    let name = Vector::as_string(env, Symbol::destruct(env, symbol).1);
                    let mut hash_ref = block_on(hash.write());

                    hash_ref.insert(name, symbol);
                }
                None => panic!(),
            },
            Namespace::Dynamic(_) => panic!(),
        }
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

        fp.value = match Self::find_ns(env, &Vector::as_string(env, fp.argv[0])) {
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

        fp.value = match Self::find_symbol(env, ns_tag, &Vector::as_string(env, name)) {
            Some(sym) => sym,
            None => Tag::nil(),
        };

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
