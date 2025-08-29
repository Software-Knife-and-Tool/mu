//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// symbol type
use {
    crate::{
        core::{
            apply::Apply as _,
            direct::{DirectExt, DirectTag, DirectType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            gc::{Gc as _, GcContext},
            heap::{Heap, HeapRequest},
            image::Image,
            indirect::IndirectTag,
            namespace::Namespace,
            readtable::SyntaxType,
            tag::{Tag, TagType},
            type_::Type,
            writer::Writer,
        },
        streams::writer::StreamWriter,
        types::vector::Vector,
    },
    futures_lite::future::block_on,
    std::str,
};

lazy_static! {
    pub static ref UNBOUND: Tag = DirectTag::to_tag(0, DirectExt::Length(0), DirectType::Keyword);
}

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
                        .image_slice(main.image_id() as usize)
                        .unwrap(),
                ),
                name: Tag::from_slice(
                    context
                        .heap_ref
                        .image_slice(main.image_id() as usize + 1)
                        .unwrap(),
                ),
                value: Tag::from_slice(
                    context
                        .heap_ref
                        .image_slice(main.image_id() as usize + 2)
                        .unwrap(),
                ),
            },
            _ => panic!(),
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
                _ => panic!(),
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
            Tag::Image(_) => panic!(),
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
                name: Vector::from(name).evict(env),
                value,
            })
        } else {
            match str[0] as char {
                ':' => Symbol::Keyword(Self::keyword(&name[1..])),
                _ => Symbol::Symbol(SymbolImage {
                    namespace,
                    name: Vector::from(name).evict(env),
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

    pub fn to_image_tag(self, env: &Env) -> Tag {
        let image = Image::Symbol(self);

        Image::to_tag(&image, env, Type::Symbol as u8)
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
                Tag::Direct(dir) => DirectTag::to_tag(
                    dir.data(),
                    DirectExt::Length(dir.ext() as usize),
                    DirectType::String,
                ),
                _ => panic!(),
            },
            Type::Symbol => Self::to_image(env, symbol).name,
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

    pub fn view(env: &Env, symbol: Tag) -> Tag {
        let vec = vec![
            Vector::from(format!(
                "\"{}\"",
                Namespace::name(env, Self::namespace(env, symbol)).unwrap()
            ))
            .evict(env),
            Self::name(env, symbol),
            if Self::is_bound(env, symbol) {
                Self::value(env, symbol)
            } else {
                Symbol::keyword("UNBOUND")
            },
        ];

        Vector::from(vec).evict(env)
    }

    pub fn heap_size(env: &Env, symbol: Tag) -> usize {
        let name_sz = Heap::heap_size(env, Self::name(env, symbol));
        let value_sz = Heap::heap_size(env, Self::value(env, symbol));

        std::mem::size_of::<SymbolImage>()
            + if name_sz > 8 { name_sz } else { 0 }
            + if value_sz > 8 { value_sz } else { 0 }
    }

    pub fn evict(&self, env: &Env) -> Tag {
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

    pub fn evict_image(tag: Tag, env: &Env) -> Tag {
        match tag {
            Tag::Image(_) => Symbol::Symbol(Self::to_image(env, tag)).evict(env),
            _ => panic!(),
        }
    }

    pub fn keyword(name: &str) -> Tag {
        let str = name.as_bytes();
        let len = str.len();

        assert!(len <= DirectTag::DIRECT_STR_MAX && len != 0);

        let mut data: [u8; 8] = 0_u64.to_le_bytes();
        for (src, dst) in str.iter().zip(data.iter_mut()) {
            *dst = *src
        }

        DirectTag::to_tag(
            u64::from_le_bytes(data),
            DirectExt::Length(len),
            DirectType::Keyword,
        )
    }

    pub fn parse(env: &Env, token: String) -> exception::Result<Tag> {
        for ch in token.chars() {
            match SyntaxType::map_char_syntax(ch) {
                Some(SyntaxType::Constituent) => (),
                _ => Err(Exception::new(env, Condition::Range, "mu:read", ch.into()))?,
            }
        }

        match token.find(':') {
            Some(0) => {
                if token.starts_with(':')
                    && (token.len() > DirectTag::DIRECT_STR_MAX + 1 || token.len() == 1)
                {
                    Err(Exception::new(
                        env,
                        Condition::Syntax,
                        "mu:read",
                        Vector::from(token.clone()).evict(env),
                    ))?
                }
                Ok(Symbol::new(env, Tag::nil(), &token, *UNBOUND).evict(env))
            }
            Some(_) => {
                let sym: Vec<&str> = token.split(':').collect();
                let ns: String = sym[0].into();
                let name = sym[1].into();

                if sym.len() != 2 {
                    Err(Exception::new(
                        env,
                        Condition::Syntax,
                        "mu:read",
                        Vector::from(token.clone()).evict(env),
                    ))?
                }

                match Namespace::find(env, &ns) {
                    Some(ns) => Ok(Namespace::intern(env, ns, name, *UNBOUND).unwrap()),
                    None => Err(Exception::new(
                        env,
                        Condition::Namespace,
                        "mu:read",
                        Vector::from(sym[0]).evict(env),
                    ))?,
                }
            }
            None => Ok(Namespace::intern(env, env.null_ns, token, *UNBOUND).unwrap()),
        }
    }

    pub fn write(env: &Env, symbol: Tag, _escape: bool, stream: Tag) -> exception::Result<()> {
        match symbol.type_of() {
            Type::Null | Type::Keyword => match str::from_utf8(&symbol.data(env).to_le_bytes()) {
                Ok(s) => {
                    StreamWriter::write_char(env, stream, ':').unwrap();
                    for nth in 0..DirectTag::length(symbol) {
                        StreamWriter::write_char(env, stream, s.as_bytes()[nth] as char)?;
                    }

                    Ok(())
                }
                Err(_) => panic!(),
            },
            Type::Symbol => {
                let name = Self::name(env, symbol);
                let ns = Self::namespace(env, symbol);

                if Tag::nil().eq_(&ns) {
                    StreamWriter::write_str(env, "#:", stream)?
                } else if !env.null_ns.eq_(&ns) {
                    match Namespace::name(env, ns) {
                        Some(str) => StreamWriter::write_str(env, &str, stream).unwrap(),
                        None => panic!(),
                    }
                    StreamWriter::write_str(env, ":", stream)?;
                }

                env.write(name, false, stream)
            }
            _ => panic!(),
        }
    }

    pub fn is_bound(env: &Env, symbol: Tag) -> bool {
        !Self::value(env, symbol).eq_(&UNBOUND)
    }
}

pub trait CoreFunction {
    fn mu_name(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_value(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_boundp(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_symbol(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Symbol {
    fn mu_name(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Null | Type::Keyword | Type::Symbol => Symbol::name(env, symbol),
            _ => {
                return Err(Exception::new(
                    env,
                    Condition::Type,
                    "mu:symbol-name",
                    symbol,
                ))
            }
        };

        Ok(())
    }

    fn mu_ns(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Symbol | Type::Keyword | Type::Null => Symbol::namespace(env, symbol),
            _ => {
                return Err(Exception::new(
                    env,
                    Condition::Type,
                    "mu:symbol-namespace",
                    symbol,
                ))
            }
        };

        Ok(())
    }

    fn mu_value(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match symbol.type_of() {
            Type::Symbol => {
                if Symbol::is_bound(env, symbol) {
                    Symbol::value(env, symbol)
                } else {
                    return Err(Exception::new(
                        env,
                        Condition::Type,
                        "mu:symbol-value",
                        symbol,
                    ));
                }
            }
            Type::Keyword | Type::Null => symbol,
            _ => {
                return Err(Exception::new(
                    env,
                    Condition::Type,
                    "mu:symbol-value",
                    symbol,
                ))
            }
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
            _ => return Err(Exception::new(env, Condition::Type, "mu:boundp", symbol)),
        };

        Ok(())
    }

    fn mu_symbol(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let name = fp.argv[0];

        env.argv_check("mu:make-symbol", &[Type::String], fp)?;

        let str = Vector::as_string(env, name);

        fp.value = Self::new(env, Tag::nil(), &str, *UNBOUND).evict(env);

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
