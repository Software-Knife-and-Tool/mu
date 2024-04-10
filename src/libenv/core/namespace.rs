//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env symbol namespaces
use {
    crate::{
        core::{
            apply::Core as _,
            direct::DirectTag,
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            types::{Tag, Type},
        },
        types::{
            cons::{Cons, Core as _},
            symbol::{Core as _, Symbol, UNBOUND},
            vecimage::{TypedVec, VecType},
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
    pub fn register_ns(env: &Env, name: Tag, ns: Namespace) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_index.write());

        if ns_ref.contains_key(&name.as_u64()) {
            return Err(Exception::new(Condition::Type, "make-ns", name));
        }

        ns_ref.insert(name.as_u64(), (name, ns));

        Ok(name)
    }

    pub fn add_ns(env: &Env, ns: Tag) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_index.write());

        if ns_ref.contains_key(&ns.as_u64()) {
            return Err(Exception::new(Condition::Type, "make-ns", ns));
        }

        ns_ref.insert(
            ns.as_u64(),
            (
                ns,
                Namespace::Dynamic(RwLock::new(HashMap::<String, Tag>::new())),
            ),
        );

        Ok(ns)
    }

    pub fn add_static_ns(
        env: &Env,
        name: Tag,
        ns: &'static RwLock<HashMap<String, Tag>>,
    ) -> exception::Result<Tag> {
        let mut ns_ref = block_on(env.ns_index.write());

        if ns_ref.contains_key(&name.as_u64()) {
            return Err(Exception::new(Condition::Type, "make-ns", name));
        }

        ns_ref.insert(name.as_u64(), (name, Namespace::Static(ns)));

        Ok(name)
    }

    fn map_symbol(env: &Env, ns: Tag, name: &str) -> Option<Tag> {
        let ns_ref = block_on(env.ns_index.read());

        let (_, ns_cache) = &ns_ref[&ns.as_u64()];

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

    pub fn intern(env: &Env, ns: Tag, symbol: Tag) {
        let ns_ref = block_on(env.ns_index.read());

        let (_, ns_cache) = &ns_ref[&ns.as_u64()];
        let name = Vector::as_string(env, Symbol::name(env, symbol));

        let mut hash = block_on(match ns_cache {
            Namespace::Static(hash) => hash.write(),
            Namespace::Dynamic(hash) => hash.write(),
        });

        hash.insert(name, symbol);
    }

    pub fn is_ns(env: &Env, tag: Tag) -> Option<Tag> {
        match tag.type_of() {
            Type::Null => Some(tag),
            Type::Keyword => {
                let ns_ref = block_on(env.ns_index.read());

                if ns_ref.contains_key(&tag.as_u64()) {
                    Some(tag)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn intern_symbol(env: &Env, ns: Tag, name: String, value: Tag) -> Tag {
        // if !ns.eq_(&Symbol::keyword("lib")) { print!("intern: {} ", name) }
        match Self::is_ns(env, ns) {
            Some(ns) => match Self::map_symbol(env, ns, &name) {
                Some(symbol) => {
                    /*
                        if !ns.eq_(&Symbol::keyword("lib")) { println!("existing symbol: {} boundp, value is unbound {}",
                                 Symbol::is_bound(env, symbol),
                                 value.eq_(&*UNBOUND)
                    )}
                        */
                    // if the symbol is unbound, bind it.
                    if Symbol::is_bound(env, symbol)
                    /* && !value.eq_(&*UNBOUND) */
                    {
                        symbol
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

                        symbol
                    }
                }
                None => {
                    /*
                        if !ns.eq_(&Symbol::keyword("lib")) { println!("new symbol: value is unbound {}",
                                                                       value.eq_(&*UNBOUND)
                    )}
                        */

                    let symbol = Symbol::new(env, ns, &name, value).evict(env);

                    Self::intern(env, ns, symbol);

                    symbol
                }
            },
            _ => panic!(),
        }
    }
}

pub trait LibFunction {
    fn lib_intern(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_make_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_ns_find(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_ns_map(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_ns_symbols(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_unbound(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Namespace {
    fn lib_unbound(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];
        let name = fp.argv[1];

        fp.value = match env.fp_argv_check("unbound", &[Type::T, Type::String], fp) {
            Ok(_) => {
                let ns = match ns.type_of() {
                    Type::Null => env.null_ns,
                    Type::Keyword => match Self::is_ns(env, ns) {
                        Some(ns) => ns,
                        _ => return Err(Exception::new(Condition::Type, "unbound", ns)),
                    },
                    _ => return Err(Exception::new(Condition::Type, "unbound", ns)),
                };

                if Vector::length(env, name) == 0 {
                    return Err(Exception::new(Condition::Syntax, "unbound", ns));
                }

                let name_str = Vector::as_string(env, name);
                let str = name_str.as_bytes();
                let len = str.len();

                if len == 0 {
                    return Err(Exception::new(Condition::Syntax, "unbound", name));
                }

                if ns.eq_(&env.keyword_ns) {
                    if len > DirectTag::DIRECT_STR_MAX {
                        return Err(Exception::new(Condition::Syntax, "unbound", name));
                    }

                    Symbol::keyword(&name_str)
                } else {
                    Self::intern_symbol(env, ns, name_str, *UNBOUND)
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_intern(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns_tag = fp.argv[0];
        let name = fp.argv[1];
        let value = fp.argv[2];

        fp.value = match env.fp_argv_check("intern", &[Type::T, Type::String, Type::T], fp) {
            Ok(_) => {
                if ns_tag.eq_(&Symbol::keyword("core")) {
                    return Err(Exception::new(Condition::Write, "intern", ns_tag));
                }
                let ns = match ns_tag.type_of() {
                    Type::Null => env.null_ns,
                    Type::Keyword => match Self::is_ns(env, ns_tag) {
                        Some(ns) => ns,
                        _ => return Err(Exception::new(Condition::Type, "intern", ns_tag)),
                    },
                    _ => return Err(Exception::new(Condition::Type, "intern", ns_tag)),
                };

                let name_str = Vector::as_string(env, name);
                let str = name_str.as_bytes();
                let len = str.len();

                if len == 0 {
                    return Err(Exception::new(Condition::Syntax, "intern", name));
                }

                if ns.eq_(&env.keyword_ns) {
                    if len > DirectTag::DIRECT_STR_MAX {
                        return Err(Exception::new(Condition::Syntax, "intern", name));
                    }

                    Symbol::keyword(&name_str)
                } else {
                    Self::intern_symbol(env, ns, name_str, value)
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_make_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns_tag = fp.argv[0];

        match ns_tag.type_of() {
            Type::Keyword => {
                fp.value = ns_tag;
                match Self::is_ns(env, ns_tag) {
                    Some(_) => return Err(Exception::new(Condition::Namespace, "make-ns", ns_tag)),
                    None => Self::add_ns(env, fp.value).unwrap(),
                };
            }
            _ => return Err(Exception::new(Condition::Type, "make-ns", ns_tag)),
        }

        Ok(())
    }

    fn lib_ns_find(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns_tag = fp.argv[0];
        let name = fp.argv[1];

        fp.value = match env.fp_argv_check("ns-find", &[Type::T, Type::String], fp) {
            Ok(_) => {
                match ns_tag.type_of() {
                    Type::Null => env.null_ns,
                    Type::Keyword => match Self::is_ns(env, ns_tag) {
                        Some(_) => ns_tag,
                        _ => return Err(Exception::new(Condition::Type, "ns-find", ns_tag)),
                    },
                    _ => return Err(Exception::new(Condition::Type, "ns-find", ns_tag)),
                };

                match Self::map_symbol(env, ns_tag, &Vector::as_string(env, name)) {
                    Some(sym) => sym,
                    None => Tag::nil(),
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_ns_symbols(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let type_ = fp.argv[0];
        let ns = fp.argv[1];

        fp.value = match env.fp_argv_check("ns-syms", &[Type::Keyword, Type::T], fp) {
            Ok(_) => match Self::is_ns(env, ns) {
                Some(_) => {
                    let ns_ref = block_on(env.ns_index.read());
                    let (_, ns_cache) = &ns_ref[&ns.as_u64()];

                    let hash = block_on(match ns_cache {
                        Namespace::Static(hash) => hash.read(),
                        Namespace::Dynamic(hash) => hash.read(),
                    });

                    let vec = hash.keys().map(|key| hash[key]).collect::<Vec<Tag>>();

                    if type_.eq_(&Symbol::keyword("list")) {
                        Cons::vlist(env, &vec)
                    } else if type_.eq_(&Symbol::keyword("vector")) {
                        TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
                    } else {
                        return Err(Exception::new(Condition::Type, "ns-syms", type_));
                    }
                }
                _ => return Err(Exception::new(Condition::Type, "ns-syms", ns)),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_ns_map(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let ns_ref = block_on(env.ns_index.read());
        let vec = ns_ref
            .keys()
            .map(|key| Tag::from(&key.to_le_bytes()))
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
