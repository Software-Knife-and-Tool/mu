//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// symbol type
use crate::{
    core::{
        direct::{DirectExt, DirectTag, DirectType},
        env::Env,
        tag::Tag,
        type_::Type,
    },
    gc::gc_::{Gc as _, GcContext},
    types::symbol::{Symbol, SymbolImage},
};

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

#[cfg(test)]
mod tests {
    #[test]
    fn symbol_test() {
        assert!(true);
    }
}
