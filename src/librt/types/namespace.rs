//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env namespaces
use {
    crate::{
        core::{
            apply::Core as _,
            direct::{DirectInfo, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            types::{Tag, Type},
        },
        streams::write::Core as _,
        types::{
            cons::{Cons, Core as _},
            symbol::{Core as _, Symbol, UNBOUND},
            vector::{Core as _, Vector},
        },
    },
    std::{collections::HashMap, str},
};

use {futures::executor::block_on, futures_locks::RwLock};

pub enum Namespace {
    Static(&'static RwLock<HashMap<String, Tag>>),
    Dynamic(RwLock<HashMap<String, Tag>>),
}

impl Namespace {
    pub fn add_ns(env: &Env, name: &str) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_map.write());
        let len = ns_ref.len();

        if ns_ref.iter().any(|(_, ns_name, _)| name == ns_name) {
            drop(ns_ref);

            return Err(Exception::new(
                env,
                Condition::Type,
                "lib:make-ns",
                Vector::from_string(name).evict(env),
            ));
        }

        let ns = DirectTag::to_direct(
            len as u64,
            DirectInfo::ExtType(ExtType::Namespace),
            DirectType::Ext,
        );

        ns_ref.push((
            ns,
            name.to_string(),
            Namespace::Dynamic(RwLock::new(HashMap::<String, Tag>::new())),
        ));

        Ok(ns)
    }

    pub fn add_static_ns(
        env: &Env,
        name: &str,
        ns_map: &'static RwLock<HashMap<String, Tag>>,
    ) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_map.write());
        let len = ns_ref.len();

        if ns_ref.iter().any(|(_, ns_name, _)| name == ns_name) {
            return Err(Exception::new(
                env,
                Condition::Type,
                "lib:make-ns",
                Vector::from_string(name).evict(env),
            ));
        }

        let ns = DirectTag::to_direct(
            len as u64,
            DirectInfo::ExtType(ExtType::Namespace),
            DirectType::Ext,
        );

        ns_ref.push((ns, name.to_string(), Namespace::Static(ns_map)));

        Ok(ns)
    }

    fn map_symbol(env: &Env, ns: Tag, name: &str) -> Option<Tag> {
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

    pub fn is_ns(tag: Tag) -> Option<Tag> {
        match tag.type_of() {
            Type::Null | Type::Namespace => Some(tag),
            _ => None,
        }
    }

    pub fn map_ns(env: &Env, name: &str) -> Option<Tag> {
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

    pub fn ns_name(env: &Env, ns: Tag) -> Option<String> {
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
                    Some("".to_string())
                } else {
                    Some(tag.to_string())
                }
            }
            None => None,
        }
    }

    pub fn makunbound(env: &Env, symbol: Tag) -> Tag {
        let image = Symbol::to_image(env, symbol);
        let slices: &[[u8; 8]] = &[
            image.namespace.as_slice(),
            image.name.as_slice(),
            UNBOUND.as_slice(),
        ];

        let offset = match symbol {
            Tag::Indirect(heap) => heap.image_id(),
            _ => panic!(),
        } as usize;

        let mut heap_ref = block_on(env.heap.write());

        heap_ref.write_image(slices, offset);

        symbol
    }

    pub fn intern(env: &Env, ns: Tag, name: String, value: Tag) -> Option<Tag> {
        if env.keyword_ns.eq_(&ns) {
            if name.len() > DirectTag::DIRECT_STR_MAX {
                return None;
            }

            return Some(Symbol::keyword(&name));
        }

        match Self::map_symbol(env, ns, &name) {
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

    pub fn unintern(env: &Env, symbol: Tag) -> Option<Tag> {
        let ns = Symbol::namespace(env, symbol);

        let image = Symbol::to_image(env, symbol);
        let slices: &[[u8; 8]] = &[
            Tag::nil().as_slice(),
            image.name.as_slice(),
            image.value.as_slice(),
        ];

        let offset = match symbol {
            Tag::Indirect(heap) => heap.image_id(),
            _ => panic!(),
        } as usize;

        {
            let mut heap_ref = block_on(env.heap.write());

            heap_ref.write_image(slices, offset);
        }

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
                    Namespace::Static(_) => return None,
                    Namespace::Dynamic(hash) => hash.write(),
                });

                hash.remove(&name);
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
            match env.write_string(
                &format!("#<:ns \"{}\">", Namespace::ns_name(env, ns).unwrap()),
                stream,
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else {
            match env.write_string(&Namespace::ns_name(env, ns).unwrap(), stream) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

pub trait CoreFunction {
    fn lib_find(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_find_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_intern(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_makunbound(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_make_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_ns_map(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_ns_name(env: &Env, fp: &mut Frame) -> exception::Result<()>;
    fn lib_symbols(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_unintern(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Namespace {
    fn lib_unintern(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match env.fp_argv_check("lib:unintern", &[Type::Symbol], fp) {
            Ok(_) => match Self::map_symbol(
                env,
                Symbol::namespace(env, symbol),
                &Vector::as_string(env, Symbol::name(env, symbol)),
            ) {
                Some(_) => match Self::unintern(env, symbol) {
                    Some(_) => symbol,
                    None => Tag::nil(),
                },
                None => Tag::nil(),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_intern(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];
        let name = fp.argv[1];
        let value = fp.argv[2];

        fp.value =
            match env.fp_argv_check("lib:intern", &[Type::Namespace, Type::String, Type::T], fp) {
                Ok(_) => match Self::intern(env, ns, Vector::as_string(env, name), value) {
                    Some(ns) => ns,
                    None => return Err(Exception::new(env, Condition::Range, "lib:intern", name)),
                },
                Err(e) => return Err(e),
            };

        Ok(())
    }

    fn lib_makunbound(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match env.fp_argv_check("lib:makunbound", &[Type::Symbol], fp) {
            Ok(_) => Self::makunbound(env, symbol),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_make_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let name = fp.argv[0];

        fp.value = match env.fp_argv_check("lib:make-ns", &[Type::String], fp) {
            Ok(_) => match Self::add_ns(env, &Vector::as_string(env, name)) {
                Ok(ns) => ns,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_ns_name(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];

        fp.value = match env.fp_argv_check("lib:ns-name", &[Type::Namespace], fp) {
            Ok(_) => Vector::from_string(&Self::ns_name(env, ns).unwrap()).evict(env),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_find_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let name = fp.argv[0];

        fp.value = match env.fp_argv_check("lib:find-ns", &[Type::String], fp) {
            Ok(_) => match Self::map_ns(env, &Vector::as_string(env, name)) {
                Some(ns) => ns,
                None => return Err(Exception::new(env, Condition::Type, "lib:find-ns", name)),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_find(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns_tag = fp.argv[0];
        let name = fp.argv[1];

        fp.value = match env.fp_argv_check("lib:find", &[Type::Namespace, Type::String], fp) {
            Ok(_) => {
                let ns_ref = block_on(env.ns_map.read());
                match ns_ref.iter().find_map(
                    |(tag, _, ns_map)| {
                        if ns_tag.eq_(tag) {
                            Some(ns_map)
                        } else {
                            None
                        }
                    },
                ) {
                    Some(_) => match Self::map_symbol(env, ns_tag, &Vector::as_string(env, name)) {
                        Some(sym) => sym,
                        None => Tag::nil(),
                    },
                    None => return Err(Exception::new(env, Condition::Type, "lib:find", ns_tag)),
                }
            }
            _ => return Err(Exception::new(env, Condition::Type, "lib:find", ns_tag)),
        };

        Ok(())
    }

    fn lib_symbols(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];

        fp.value = match env.fp_argv_check("lib:symbols", &[Type::Namespace], fp) {
            Ok(_) => match Self::is_ns(ns) {
                Some(_) => {
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
                            let hash = block_on(match ns_map {
                                Namespace::Static(hash) => hash.read(),
                                Namespace::Dynamic(hash) => hash.read(),
                            });

                            let vec = hash.keys().map(|key| hash[key]).collect::<Vec<Tag>>();

                            Cons::vlist(env, &vec)
                        }
                        None => panic!(),
                    }
                }
                _ => return Err(Exception::new(env, Condition::Type, "lib:symbols", ns)),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_ns_map(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns_ref = block_on(env.ns_map.read());
        let vec = ns_ref
            .iter()
            .map(|(_, name, _)| Vector::from_string(name).evict(env))
            .collect::<Vec<Tag>>();

        fp.value = Cons::vlist(env, &vec);

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
