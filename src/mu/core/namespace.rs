//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env namespaces
use {
    crate::{
        core::{
            core::CoreFnDef,
            direct::{DirectExt, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            types::{Tag, Type},
        },
        types::vector::Vector,
    },
    std::{collections::HashMap, str},
};

use {futures_lite::future::block_on, futures_locks::RwLock};

#[derive(Clone)]
pub struct Static {
    pub functions: Option<&'static Vec<CoreFnDef>>,
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
            Type::Namespace => ns.data(env) as usize,
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
        ns_map: Option<&'static RwLock<HashMap<String, Tag>>>,
        functab: Option<&'static Vec<CoreFnDef>>,
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn namespace() {
        assert_eq!(true, true)
    }
}
