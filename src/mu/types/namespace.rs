//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env namespaces
use {
    crate::{
        core::{
            apply::Core as _,
            direct::{DirectExt, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            types::{Tag, Type},
        },
        streams::write::Core as _,
        types::{
            cons::{Cons, Core as _},
            symbol::{Core as _, Symbol},
            vector::{Core as _, Vector},
        },
        vectors::core::Core as _,
    },
    std::{collections::HashMap, str},
};

use {futures::executor::block_on, futures_locks::RwLock};

#[derive(Clone)]
pub enum Namespace {
    Static(&'static RwLock<HashMap<String, Tag>>),
    Dynamic(RwLock<HashMap<String, Tag>>),
}

impl Namespace {
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

        let ns = DirectTag::to_tag(
            len as u64,
            DirectExt::ExtType(ExtType::Namespace),
            DirectType::Ext,
        );

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
        ns_map: &'static RwLock<HashMap<String, Tag>>,
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

        let ns = DirectTag::to_tag(
            len as u64,
            DirectExt::ExtType(ExtType::Namespace),
            DirectType::Ext,
        );

        ns_ref.push((ns, name.into(), Namespace::Static(ns_map)));

        Ok(ns)
    }

    fn find_symbol(env: &Env, ns: Tag, name: &str) -> Option<Tag> {
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
                    Namespace::Static(hash) => hash.read(),
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

    pub fn is_(tag: Tag) -> Option<Tag> {
        match tag.type_of() {
            Type::Namespace => Some(tag),
            _ => None,
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
                            Namespace::Static(hash) => hash.write(),
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
                    Namespace::Static(hash) => hash.write(),
                    Namespace::Dynamic(_) => return None,
                });

                hash.insert(name, symbol);
            }
            None => return None,
        }

        Some(symbol)
    }
}

pub trait Core {
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl Core for Namespace {
    fn write(env: &Env, ns: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        if escape {
            env.write_string(
                &format!("#<:ns \"{}\">", Namespace::name(env, ns).unwrap()),
                stream,
            )?
        } else {
            env.write_string(&Namespace::name(env, ns).unwrap(), stream)?
        }

        Ok(())
    }
}

pub trait CoreFunction {
    fn mu_find(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_find_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_intern(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_make_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns_map(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns_name(env: &Env, fp: &mut Frame) -> exception::Result<()>;
    fn mu_symbols(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Namespace {
    fn mu_intern(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];
        let name = fp.argv[1];
        let value = fp.argv[2];

        env.fp_argv_check("mu:intern", &[Type::Namespace, Type::String, Type::T], fp)?;
        fp.value = match Self::intern(env, ns, Vector::as_string(env, name), value) {
            Some(ns) => ns,
            None => return Err(Exception::new(env, Condition::Range, "mu:intern", name)),
        };

        Ok(())
    }

    fn mu_make_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let name = fp.argv[0];

        env.fp_argv_check("mu:make-namespace", &[Type::String], fp)?;
        fp.value = Self::with(env, &Vector::as_string(env, name))?;

        Ok(())
    }

    fn mu_ns_name(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];

        env.fp_argv_check("mu:ns-name", &[Type::Namespace], fp)?;
        fp.value = Vector::from(Self::name(env, ns).unwrap()).evict(env);

        Ok(())
    }

    fn mu_find_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let name = fp.argv[0];

        env.fp_argv_check("mu:find-namespace", &[Type::String], fp)?;
        fp.value = match Self::find(env, &Vector::as_string(env, name)) {
            Some(ns) => ns,
            None => {
                return Err(Exception::new(
                    env,
                    Condition::Type,
                    "mu:find-namespace",
                    name,
                ))
            }
        };

        Ok(())
    }

    fn mu_find(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns_tag = fp.argv[0];
        let name = fp.argv[1];

        env.fp_argv_check("mu:find", &[Type::Namespace, Type::String], fp)?;

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

                return Err(Exception::new(env, Condition::Type, "mu:find", ns_tag));
            }
        };

        Ok(())
    }

    fn mu_symbols(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];

        env.fp_argv_check("mu:symbols", &[Type::Namespace], fp)?;

        let ns_ref = block_on(env.ns_map.read());

        fp.value = match ns_ref.iter().find_map(
            |(tag, _, ns_map)| {
                if ns.eq_(tag) {
                    Some(ns_map)
                } else {
                    None
                }
            },
        ) {
            Some(ns_map) => {
                let hash = block_on(match ns_map {
                    Namespace::Static(hash) => hash.read(),
                    Namespace::Dynamic(hash) => hash.read(),
                });

                let vec = hash.keys().map(|key| hash[key]).collect::<Vec<Tag>>();

                Cons::list(env, &vec)
            }
            None => panic!(),
        };

        Ok(())
    }

    fn mu_ns_map(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.fp_argv_check("mu:ns-map", &[], fp)?;

        let ns_ref = block_on(env.ns_map.read());
        let vec = ns_ref
            .iter()
            .map(|(_, name, _)| Vector::from((*name).clone()).evict(env))
            .collect::<Vec<Tag>>();

        fp.value = Cons::list(env, &vec);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn namespace() {
        assert_eq!(true, true)
    }
}
