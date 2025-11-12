//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// namespaces
use {
    crate::{
        core::{
            apply::Apply as _,
            core_::{CoreFnDef, CORE, CORE_FUNCTIONS},
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
        match self {
            Namespace::Static(static_) => {
                if let Some(hash) = &static_ {
                    for (_, symbol) in hash.iter() {
                        Symbol::mark(gc, env, *symbol)
                    }
                }
            }
            Namespace::Dynamic(ref hash) => {
                let hash_ref = block_on(hash.read());
                for (_, symbol) in hash_ref.iter() {
                    Symbol::mark(gc, env, *symbol)
                }
            }
        };
    }
}

#[derive(Clone)]
pub enum Namespace {
    Static(Option<HashMap<String, Tag>>),
    Dynamic(RwLock<HashMap<String, Tag>>),
}

#[derive(Clone)]
pub struct StaticSymbols(pub Option<Vec<(String, Tag)>>, pub Option<Vec<CoreFnDef>>);

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

    pub fn with_static(env: &Env, name: &str, defs: StaticSymbols) -> exception::Result<Tag> {
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
        let mut ns_map = HashMap::new();

        if let Some(sym_defs) = defs.0 {
            for def in sym_defs {
                let symbol = Symbol::new(env, ns, &def.0, def.1).with_heap(env);

                ns_map.insert(def.0, symbol).unwrap();
            }
        }

        if let Some(fn_defs) = defs.1 {
            for def in fn_defs {
                let name = def.0;

                let (ndef, _) = CORE
                    .core_defs
                    .iter()
                    .enumerate()
                    .find(|(_, static_)| name == static_.0)
                    .unwrap();
                let symbol = Symbol::new(
                    env,
                    ns,
                    name,
                    DirectTag::function(ndef + CORE_FUNCTIONS.len()),
                )
                .with_heap(env);

                ns_map.insert(name.to_string(), symbol);
            }
        }

        ns_ref.insert(name.to_string(), (ns, Namespace::Static(Some(ns_map))));

        Ok(ns)
    }

    pub fn with_mu_static(env: &Env, defs: StaticSymbols) -> Tag {
        let mut ns_ref = block_on(env.ns_map.write());

        if ns_ref.contains_key("mu") {
            panic!()
        }

        let ns = Struct::new(env, "ns", vec![Vector::from("mu").with_heap(env)]).with_heap(env);
        let mut ns_map = HashMap::new();

        if let Some(sym_defs) = defs.0 {
            for def in sym_defs {
                let symbol = Symbol::new(env, ns, &def.0, def.1).with_heap(env);

                ns_map.insert(def.0, symbol);
            }
        }

        if let Some(fn_defs) = defs.1 {
            for (ndef, def) in fn_defs.iter().enumerate() {
                let name = def.0;

                let symbol = Symbol::new(env, ns, name, DirectTag::function(ndef)).with_heap(env);

                ns_map.insert(name.to_string(), symbol);
            }
        }

        ns_ref.insert("mu".to_string(), (ns, Namespace::Static(Some(ns_map))));

        ns
    }

    fn is_namespace(env: &Env, ns: Tag) -> bool {
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

        match &ns_map.1 {
            Namespace::Static(static_) => match &static_ {
                Some(hash) => {
                    if hash.contains_key(name) {
                        Some(hash[name])
                    } else {
                        None?
                    }
                }
                None => None?,
            },
            Namespace::Dynamic(hash) => {
                let hash_ref = block_on(hash.read());
                if hash_ref.contains_key(name) {
                    Some(hash_ref[name])
                } else {
                    None?
                }
            }
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

                    block_on(env.heap.write()).write_image(slices, offset);

                    Some(symbol)
                }
            }
            None => {
                let symbol = Symbol::new(env, ns, &name, value).with_heap(env);
                let ns_ref = block_on(env.ns_map.read());

                match &ns_ref[&Self::name(env, ns)].1 {
                    Namespace::Static(_) => (),
                    Namespace::Dynamic(hash) => {
                        block_on(hash.write()).insert(name, symbol);
                    }
                }

                Some(symbol)
            }
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
