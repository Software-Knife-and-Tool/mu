//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env symbol type
use {
    crate::{
        core::{
            apply::Core as _,
            direct::{DirectInfo, DirectTag, DirectType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            gc::Core as _,
            heap::{Core as _, Heap},
            indirect::IndirectTag,
            readtable::{map_char_syntax, SyntaxType},
            types::{Tag, TagType, Type},
        },
        streams::write::Core as _,
        types::{
            core_stream::{Core as _, Stream},
            indirect_vector::{TypedVector, VecType},
            namespace::Namespace,
            vector::{Core as _, Vector},
        },
    },
    std::str,
};

use futures::executor::block_on;

pub enum Symbol {
    Keyword(Tag),
    Symbol(SymbolImage),
}

pub struct SymbolImage {
    pub namespace: Tag,
    pub name: Tag,
    pub value: Tag,
}

lazy_static! {
    pub static ref UNBOUND: Tag =
        DirectTag::to_direct(0, DirectInfo::Length(0), DirectType::Keyword);
}

impl Symbol {
    pub fn new(env: &Env, namespace: Tag, name: &str, value: Tag) -> Self {
        let str = name.as_bytes();

        if name.is_empty() {
            Symbol::Symbol(SymbolImage {
                namespace,
                name: Vector::from_string(name).evict(env),
                value,
            })
        } else {
            match str[0] as char {
                ':' => Symbol::Keyword(Self::keyword(&name[1..])),
                _ => Symbol::Symbol(SymbolImage {
                    namespace,
                    name: Vector::from_string(name).evict(env),
                    value,
                }),
            }
        }
    }

    pub fn to_image(env: &Env, tag: Tag) -> SymbolImage {
        let heap_ref = block_on(env.heap.read());

        match tag.type_of() {
            Type::Symbol => match tag {
                Tag::Indirect(main) => SymbolImage {
                    namespace: Tag::from_slice(
                        heap_ref.image_slice(main.image_id() as usize).unwrap(),
                    ),
                    name: Tag::from_slice(
                        heap_ref.image_slice(main.image_id() as usize + 1).unwrap(),
                    ),
                    value: Tag::from_slice(
                        heap_ref.image_slice(main.image_id() as usize + 2).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn namespace(env: &Env, symbol: Tag) -> Tag {
        match symbol.type_of() {
            Type::Null => env.null_ns,
            Type::Keyword => env.keyword_ns,
            Type::Symbol => Self::to_image(env, symbol).namespace,
            _ => panic!(),
        }
    }

    pub fn name(env: &Env, symbol: Tag) -> Tag {
        match symbol.type_of() {
            Type::Null | Type::Keyword => match symbol {
                Tag::Direct(dir) => DirectTag::to_direct(
                    dir.data(),
                    DirectInfo::Length(dir.info() as usize),
                    DirectType::ByteVector,
                ),
                _ => panic!(),
            },
            Type::Symbol => Self::to_image(env, symbol).name,
            Type::Namespace => panic!("namespace"),
            _ => panic!(),
        }
    }

    pub fn value(env: &Env, symbol: Tag) -> Tag {
        match symbol.type_of() {
            Type::Null | Type::Keyword => symbol,
            Type::Symbol => Self::to_image(env, symbol).value,
            _ => panic!(),
        }
    }
}

pub trait Core {
    fn evict(&self, _: &Env) -> Tag;
    fn mark(_: &Env, _: Tag);
    fn heap_size(_: &Env, _: Tag) -> usize;
    fn is_bound(_: &Env, _: Tag) -> bool;
    fn keyword(_: &str) -> Tag;
    fn parse(_: &Env, _: String) -> exception::Result<Tag>;
    fn view(_: &Env, _: Tag) -> Tag;
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl Core for Symbol {
    fn view(env: &Env, symbol: Tag) -> Tag {
        let vec = vec![
            Vector::from_string(&format!(
                "\"{}\"",
                Namespace::ns_name(env, Self::namespace(env, symbol)).unwrap()
            ))
            .evict(env),
            Self::name(env, symbol),
            if !Self::is_bound(env, symbol) {
                Symbol::keyword("UNBOUND")
            } else {
                Self::value(env, symbol)
            },
        ];

        TypedVector::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
    }

    fn heap_size(env: &Env, symbol: Tag) -> usize {
        let name_sz = Heap::heap_size(env, Self::name(env, symbol));
        let value_sz = Heap::heap_size(env, Self::value(env, symbol));

        std::mem::size_of::<Symbol>()
            + if name_sz > 8 { name_sz } else { 0 }
            + if value_sz > 8 { value_sz } else { 0 }
    }

    fn mark(env: &Env, symbol: Tag) {
        match symbol {
            Tag::Direct(_) => (), // keyword
            Tag::Indirect(_) => {
                let mark = env.mark_image(symbol).unwrap();

                if !mark {
                    env.mark(Self::name(env, symbol));
                    env.mark(Self::value(env, symbol));
                }
            }
        }
    }

    fn evict(&self, env: &Env) -> Tag {
        match self {
            Symbol::Keyword(tag) => *tag,
            Symbol::Symbol(image) => {
                let slices: &[[u8; 8]] = &[
                    image.namespace.as_slice(),
                    image.name.as_slice(),
                    image.value.as_slice(),
                ];

                let mut heap_ref = block_on(env.heap.write());

                Tag::Indirect(
                    IndirectTag::new()
                        .with_image_id(
                            heap_ref.alloc(slices, None, Type::Symbol as u8).unwrap() as u64
                        )
                        .with_heap_id(1)
                        .with_tag(TagType::Symbol),
                )
            }
        }
    }

    fn keyword(name: &str) -> Tag {
        let str = name.as_bytes();
        let len = str.len();

        if len > DirectTag::DIRECT_STR_MAX || len == 0 {
            panic!("{} {:?}", std::str::from_utf8(str).unwrap(), str)
        }

        let mut data: [u8; 8] = 0u64.to_le_bytes();
        for (src, dst) in str.iter().zip(data.iter_mut()) {
            *dst = *src
        }
        DirectTag::to_direct(
            u64::from_le_bytes(data),
            DirectInfo::Length(len),
            DirectType::Keyword,
        )
    }

    fn parse(env: &Env, token: String) -> exception::Result<Tag> {
        for ch in token.chars() {
            match map_char_syntax(ch) {
                Some(SyntaxType::Constituent) => (),
                _ => {
                    return Err(Exception::new(
                        env,
                        Condition::Range,
                        "symbol",
                        Tag::from(ch),
                    ));
                }
            }
        }

        match token.find(':') {
            Some(0) => {
                if token.starts_with(':')
                    && (token.len() > DirectTag::DIRECT_STR_MAX + 1 || token.len() == 1)
                {
                    return Err(Exception::new(
                        env,
                        Condition::Syntax,
                        "read:sy",
                        Vector::from_string(&token).evict(env),
                    ));
                }
                Ok(Symbol::new(env, Tag::nil(), &token, *UNBOUND).evict(env))
            }
            Some(_) => {
                let sym: Vec<&str> = token.split(':').collect();
                let ns = sym[0].to_string();
                let name = sym[1].to_string();

                if sym.len() != 2 {
                    return Err(Exception::new(
                        env,
                        Condition::Syntax,
                        "read:sy",
                        Vector::from_string(&token).evict(env),
                    ));
                }

                match Namespace::map_ns(env, &ns) {
                    Some(ns) => Ok(Namespace::intern(env, ns, name, *UNBOUND).unwrap()),
                    None => Err(Exception::new(
                        env,
                        Condition::Namespace,
                        "read:sy",
                        Vector::from_string(sym[0]).evict(env),
                    )),
                }
            }
            None => Ok(Namespace::intern(env, env.null_ns, token, *UNBOUND).unwrap()),
        }
    }

    fn write(env: &Env, symbol: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        match symbol.type_of() {
            Type::Null | Type::Keyword => match str::from_utf8(&symbol.data(env).to_le_bytes()) {
                Ok(s) => {
                    Stream::write_char(env, stream, ':').unwrap();
                    for nth in 0..DirectTag::length(symbol) {
                        match Stream::write_char(env, stream, s.as_bytes()[nth] as char) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(())
                }
                Err(_) => panic!(),
            },
            Type::Symbol => {
                let name = Self::name(env, symbol);

                if escape {
                    let ns = Self::namespace(env, symbol);

                    if !Tag::null_(&ns) && !env.null_ns.eq_(&ns) {
                        match Namespace::ns_name(env, ns) {
                            Some(str) => env.write_string(&str, stream).unwrap(),
                            None => panic!(),
                        }

                        match env.write_string(":", stream) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                }
                env.write_stream(name, false, stream)
            }
            _ => panic!(),
        }
    }

    fn is_bound(env: &Env, symbol: Tag) -> bool {
        !Self::value(env, symbol).eq_(&UNBOUND)
    }
}

pub trait CoreFunction {
    fn lib_name(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_value(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_boundp(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_symbol(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Symbol {
    fn lib_name(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Null | Type::Keyword | Type::Symbol => Symbol::name(env, symbol),
            _ => return Err(Exception::new(env, Condition::Type, "sy:name", symbol)),
        };

        Ok(())
    }

    fn lib_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Symbol | Type::Keyword | Type::Null => Symbol::namespace(env, symbol),
            _ => return Err(Exception::new(env, Condition::Type, "sy:ns", symbol)),
        };

        Ok(())
    }

    fn lib_value(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Symbol => {
                if Symbol::is_bound(env, symbol) {
                    Symbol::value(env, symbol)
                } else {
                    return Err(Exception::new(env, Condition::Type, "sy-val", symbol));
                }
            }
            Type::Keyword => symbol,
            _ => return Err(Exception::new(env, Condition::Type, "sy-ns", symbol)),
        };

        Ok(())
    }

    fn lib_boundp(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Keyword => symbol,
            Type::Symbol => {
                if !Self::is_bound(env, symbol) {
                    Tag::nil()
                } else {
                    symbol
                }
            }
            _ => return Err(Exception::new(env, Condition::Type, "unboundp", symbol)),
        };

        Ok(())
    }

    fn lib_symbol(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let name = fp.argv[0];

        fp.value = match env.fp_argv_check("symbol", &[Type::String], fp) {
            Ok(_) => {
                let str = Vector::as_string(env, name);

                Self::new(env, Tag::nil(), &str, *UNBOUND).evict(env)
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
