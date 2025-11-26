//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// symbol type
use {
    crate::{
        core::{
            apply::Apply as _,
            direct::{DirectExt, DirectImage, DirectTag, DirectType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            indirect::IndirectTag,
            tag::{Tag, TagType},
            type_::Type,
        },
        namespaces::{
            cache::Cache,
            gc::{Gc as _, GcContext},
            heap::{Heap, HeapRequest},
            namespace::Namespace,
        },
        reader::readtable::SyntaxType,
        streams::writer::StreamWriter,
        types::vector::Vector,
    },
    futures_lite::future::block_on,
    std::{str, sync::LazyLock},
};

pub static UNBOUND: LazyLock<Tag> =
    LazyLock::new(|| DirectTag::to_tag(0, DirectExt::Length(0), DirectType::Keyword));

#[derive(Copy, Clone)]
pub enum Symbol {
    Keyword(Tag),
    Symbol(SymbolImage),
}

#[derive(Copy, Clone)]
pub struct SymbolImage {
    pub namespace: Tag,
    pub name: Tag,
    pub value: Tag,
}

pub trait Gc {
    fn gc_ref_image(_: &mut GcContext, tag: Tag) -> SymbolImage;
    fn ref_name(_: &mut GcContext, symbol: Tag) -> Tag;
    fn ref_value(_: &mut GcContext, symbol: Tag) -> Tag;
    fn mark(_: &mut GcContext, env: &Env, symbol: Tag);
}

impl Gc for Symbol {
    fn gc_ref_image(context: &mut GcContext, tag: Tag) -> SymbolImage {
        assert_eq!(tag.type_of(), Type::Symbol);

        match tag {
            Tag::Indirect(main) => SymbolImage {
                namespace: Tag::from_slice(
                    context
                        .heap_ref
                        .image_slice(usize::try_from(main.image_id()).unwrap())
                        .unwrap(),
                ),
                name: Tag::from_slice(
                    context
                        .heap_ref
                        .image_slice(usize::try_from(main.image_id()).unwrap() + 1)
                        .unwrap(),
                ),
                value: Tag::from_slice(
                    context
                        .heap_ref
                        .image_slice(usize::try_from(main.image_id()).unwrap() + 2)
                        .unwrap(),
                ),
            },
            Tag::Direct(_) => panic!(),
        }
    }

    fn ref_name(context: &mut GcContext, symbol: Tag) -> Tag {
        match symbol.type_of() {
            Type::Null | Type::Keyword => match symbol {
                Tag::Direct(dir) => DirectTag::to_tag(
                    dir.data(),
                    DirectExt::Length(dir.ext() as usize),
                    DirectType::String,
                ),
                Tag::Indirect(_) => panic!(),
            },
            Type::Symbol => Self::gc_ref_image(context, symbol).name,
            _ => panic!(),
        }
    }

    fn ref_value(context: &mut GcContext, symbol: Tag) -> Tag {
        match symbol.type_of() {
            Type::Null | Type::Keyword => symbol,
            Type::Symbol => Self::gc_ref_image(context, symbol).value,
            _ => panic!(),
        }
    }

    fn mark(context: &mut GcContext, env: &Env, symbol: Tag) {
        match symbol {
            Tag::Direct(_) => (),
            Tag::Indirect(_) => {
                let mark = context.mark_image(symbol).unwrap();

                if !mark {
                    let name = Self::ref_name(context, symbol);
                    let value = Self::ref_value(context, symbol);

                    context.mark(env, name);
                    context.mark(env, value);
                }
            }
        }
    }
}

impl Symbol {
    pub fn new(env: &Env, namespace: Tag, name: &str, value: Tag) -> Self {
        let str = name.as_bytes();

        if name.is_empty() {
            Symbol::Symbol(SymbolImage {
                namespace,
                name: Vector::from(name).with_heap(env),
                value,
            })
        } else {
            match str[0] as char {
                ':' => Symbol::Keyword(Self::keyword(&name[1..])),
                _ => Symbol::Symbol(SymbolImage {
                    namespace,
                    name: Vector::from(name).with_heap(env),
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
                        heap_ref
                            .image_slice(usize::try_from(main.image_id()).unwrap())
                            .unwrap(),
                    ),
                    name: Tag::from_slice(
                        heap_ref
                            .image_slice(usize::try_from(main.image_id()).unwrap() + 1)
                            .unwrap(),
                    ),
                    value: Tag::from_slice(
                        heap_ref
                            .image_slice(usize::try_from(main.image_id()).unwrap() + 2)
                            .unwrap(),
                    ),
                },
                Tag::Direct(_) => {
                    let image = Cache::ref_(env, DirectTag::cache_ref(tag).0);

                    match image {
                        DirectImage::Symbol(image) => image,
                        _ => panic!(),
                    }
                }
            },
            _ => panic!(),
        }
    }

    pub fn destruct(env: &Env, symbol: Tag) -> (Tag, Tag, Tag) {
        match symbol.type_of() {
            Type::Null => (
                env.mu_ns,
                match symbol {
                    Tag::Direct(dir) => DirectTag::to_tag(
                        dir.data(),
                        DirectExt::Length(dir.ext() as usize),
                        DirectType::String,
                    ),
                    Tag::Indirect(_) => panic!(),
                },
                symbol,
            ),
            Type::Keyword => (
                env.keyword_ns,
                match symbol {
                    Tag::Direct(dir) => DirectTag::to_tag(
                        dir.data(),
                        DirectExt::Length(dir.ext() as usize),
                        DirectType::String,
                    ),
                    Tag::Indirect(_) => panic!(),
                },
                symbol,
            ),
            Type::Symbol => {
                let image = Self::to_image(env, symbol);

                (image.namespace, image.name, image.value)
            }
            _ => panic!(),
        }
    }

    pub fn view(env: &Env, symbol: Tag) -> Tag {
        let (ns, name, value) = Self::destruct(env, symbol);
        let vec = vec![
            match ns.type_of() {
                Type::Null => ns,
                Type::Keyword => {
                    if ns.eq_(&UNBOUND) {
                        Self::keyword("unqual")
                    } else {
                        ns
                    }
                }
                Type::Struct => {
                    Vector::from(format!("\"{}\"", Namespace::name(env, ns))).with_heap(env)
                }
                _ => panic!(),
            },
            name,
            if Self::is_bound(env, symbol) {
                value
            } else {
                Symbol::keyword("UNBOUND")
            },
        ];

        Vector::from(vec).with_heap(env)
    }

    pub fn image_size(env: &Env, symbol: Tag) -> usize {
        let (_, name, value) = Self::destruct(env, symbol);
        let name_sz = Heap::image_size(env, name);
        let value_sz = Heap::image_size(env, value);

        std::mem::size_of::<SymbolImage>()
            + if name_sz > 8 { name_sz } else { 0 }
            + if value_sz > 8 { value_sz } else { 0 }
    }

    pub fn with_cache(&self, env: &Env) -> Tag {
        match self {
            Symbol::Symbol(image) => {
                DirectImage::Symbol(*image).with_cache(env, Type::Symbol as u8)
            }
            Symbol::Keyword(_) => panic!(),
        }
    }

    pub fn with_heap(&self, env: &Env) -> Tag {
        match self {
            Symbol::Keyword(tag) => *tag,
            Symbol::Symbol(image) => {
                let slices: &[[u8; 8]] = &[
                    image.namespace.as_slice(),
                    image.name.as_slice(),
                    image.value.as_slice(),
                ];

                let mut heap_ref = block_on(env.heap.write());
                let ha = HeapRequest {
                    env,
                    image: slices,
                    vdata: None,
                    type_id: Type::Symbol as u8,
                };

                match heap_ref.alloc(&ha) {
                    Some(image_id) => {
                        let ind = IndirectTag::new()
                            .with_image_id(image_id as u64)
                            .with_heap_id(1)
                            .with_tag(TagType::Symbol);

                        Tag::Indirect(ind)
                    }
                    None => panic!(),
                }
            }
        }
    }

    pub fn keyword(name: &str) -> Tag {
        let str = name.as_bytes();
        let len = str.len();

        assert!(len <= DirectTag::DIRECT_STR_MAX && len != 0);

        let mut data: [u8; 8] = 0_u64.to_le_bytes();
        for (src, dst) in str.iter().zip(data.iter_mut()) {
            *dst = *src;
        }

        DirectTag::to_tag(
            u64::from_le_bytes(data),
            DirectExt::Length(len),
            DirectType::Keyword,
        )
    }

    pub fn parse(env: &Env, token: &str) -> exception::Result<Tag> {
        let type_check = token.chars().find(|ch| {
            !matches!(
                SyntaxType::map_char_syntax(*ch).unwrap(),
                SyntaxType::Constituent
            )
        });

        if let Some(ch) = type_check {
            Err(Exception::err(env, ch.into(), Condition::Range, "mu:read"))?;
        }

        match token.find(':') {
            Some(0) => {
                if token.len() > DirectTag::DIRECT_STR_MAX + 1 || token.len() == 1 {
                    Err(Exception::err(
                        env,
                        Vector::from(token).with_heap(env),
                        Condition::Syntax,
                        "mu:read",
                    ))?;
                }

                let keyword: String = token.chars().skip(1).collect();

                Ok(Self::keyword(&keyword))
            }
            Some(_) => {
                let sym: Vec<&str> = token.split(':').collect();
                let ns: String = sym[0].into();
                let name = sym[1].into();

                if sym.len() != 2 {
                    Err(Exception::err(
                        env,
                        Vector::from(token).with_heap(env),
                        Condition::Syntax,
                        "mu:read",
                    ))?;
                }

                match Namespace::find_ns(env, &ns) {
                    Some(ns) => Ok(Namespace::intern(env, ns, name, *UNBOUND).unwrap()),
                    None => Err(Exception::err(
                        env,
                        Vector::from(sym[0]).with_heap(env),
                        Condition::Namespace,
                        "mu:read",
                    ))?,
                }
            }
            None => Ok(Self::new(env, *UNBOUND, token, *UNBOUND).with_cache(env)),
        }
    }

    pub fn write(env: &Env, symbol: Tag, _escape: bool, stream: Tag) -> exception::Result<()> {
        match symbol.type_of() {
            Type::Null | Type::Keyword => {
                let str = symbol.data(env).to_le_bytes();
                let s = str::from_utf8(&str).unwrap();

                StreamWriter::write_char(env, stream, ':').unwrap();
                for nth in 0..DirectTag::length(symbol) {
                    StreamWriter::write_char(env, stream, s.as_bytes()[nth] as char)?;
                }

                Ok(())
            }
            Type::Symbol => {
                let (ns, name, _) = Self::destruct(env, symbol);

                if !ns.eq_(&UNBOUND) {
                    if ns.null_() {
                        StreamWriter::write_str(env, "#:", stream)?;
                    } else {
                        StreamWriter::write_str(env, &Namespace::name(env, ns), stream)?;
                        StreamWriter::write_str(env, ":", stream)?;
                    }
                }

                StreamWriter::write(env, name, false, stream)
            }
            _ => panic!(),
        }
    }

    pub fn is_bound(env: &Env, symbol: Tag) -> bool {
        !Symbol::destruct(env, symbol).2.eq_(&UNBOUND)
    }
}

pub trait CoreFn {
    fn mu_name(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_value(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_boundp(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_symbol(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Symbol {
    fn mu_name(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Null | Type::Keyword | Type::Symbol => Self::destruct(env, symbol).1,
            _ => Err(Exception::err(
                env,
                symbol,
                Condition::Type,
                "mu:symbol-name",
            ))?,
        };

        Ok(())
    }

    fn mu_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Symbol => {
                let ns = Self::destruct(env, symbol).0;

                if ns.eq_(&UNBOUND) {
                    Self::keyword("unqual")
                } else {
                    ns
                }
            }
            Type::Null => env.mu_ns,
            Type::Keyword => env.keyword_ns,
            _ => Err(Exception::err(
                env,
                symbol,
                Condition::Type,
                "mu:symbol-namespace",
            ))?,
        };

        Ok(())
    }

    fn mu_value(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Symbol => {
                if Symbol::is_bound(env, symbol) {
                    Symbol::destruct(env, symbol).2
                } else {
                    Err(Exception::err(
                        env,
                        symbol,
                        Condition::Type,
                        "mu:symbol-value",
                    ))?
                }
            }
            Type::Keyword | Type::Null => symbol,
            _ => Err(Exception::err(
                env,
                symbol,
                Condition::Type,
                "mu:symbol-value",
            ))?,
        };

        Ok(())
    }

    fn mu_boundp(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Keyword | Type::Null => Self::keyword("t"),
            Type::Symbol => {
                if Self::is_bound(env, symbol) {
                    symbol
                } else {
                    Tag::nil()
                }
            }
            _ => Err(Exception::err(env, symbol, Condition::Type, "mu:boundp"))?,
        };

        Ok(())
    }

    fn mu_symbol(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:make-symbol", &[Type::String], fp)?;

        fp.value = Self::new(
            env,
            Tag::nil(),
            Vector::as_string(env, fp.argv[0]).as_str(),
            *UNBOUND,
        )
        .with_heap(env);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn symbol_test() {
        assert!(true);
    }
}
