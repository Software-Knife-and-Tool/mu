#[allow(unused_imports)]
use {
    std::fmt,
    syn::{
        self,
        token::Impl,
        GenericArgument, Ident, ImplItem, ImplItemFn, Item, ItemFn, ItemImpl, Lifetime, Path,
        PathArguments::{self, AngleBracketed, None, Parenthesized},
        PathSegment, ReturnType, Signature, Type, Visibility,
    },
};

pub enum Display {
    GenericArgument(GenericArgument),
    Ident(Ident),
    Item(Item),
    ImplItem(ImplItem),
    PathSegment(PathSegment),
    Type(Type),
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Display::ImplItem(impl_item) => match impl_item {
                ImplItem::Const(_impl) => write!(f, "ImplItem::Const"),
                ImplItem::Fn(fn_) => {
                    let ident = &fn_.sig.ident;
                    let is_public = matches!(fn_.vis, Visibility::Public(_));
                    let is_const = fn_.sig.constness.is_some();
                    let is_async = fn_.sig.asyncness.is_some();

                    let return_type = match &fn_.sig.output {
                        ReturnType::Default => "".to_string(),
                        ReturnType::Type(_, type_) => {
                            format!(" -> {}", Display::Type(*type_.clone()))
                        }
                    };

                    write!(
                        f,
                        "{}{}{}fn {}(...) {}",
                        if is_const { "const " } else { "" },
                        if is_async { "async " } else { "" },
                        if is_public { "pub " } else { "" },
                        ident,
                        return_type
                    )
                }
                ImplItem::Type(_impl) => write!(f, "ImplItem::Type"),
                ImplItem::Macro(_impl) => write!(f, "ImplItem::Macro"),
                ImplItem::Verbatim(_tokens) => write!(f, "ImplItem::Verbatim"),
                _ => panic!(),
            },
            Display::Item(item) => match item {
                Item::Const(_const) => f.pad("<Item::Const>"),
                Item::Enum(_enum) => f.pad("<Item::Enum>"),
                Item::ExternCrate(_crate) => f.pad("<Item::ExternCrate>"),
                Item::Fn(item) => write!(f, "{}", item.sig.ident),
                Item::ForeignMod(_mod) => f.pad("<Item::ForeignMod>"),
                Item::Impl(_impl) => {
                    for impl_ in &_impl.items {
                        write!(f, "{}", Display::ImplItem(impl_.clone())).unwrap()
                    }

                    Ok(())
                }
                Item::Macro(_macro) => f.pad("<Item::Macro>"),
                Item::Mod(_mod) => f.pad("<Item::Mod>"),
                Item::Static(_static) => f.pad("<Item::Static>"),
                Item::Struct(_struct) => f.pad("<Item::Struct>"),
                Item::Trait(_trait) => f.pad("<Item::Trait>"),
                Item::TraitAlias(_alias) => f.pad("<Item::TraitAlias>"),
                Item::Type(_type) => f.pad("<Item::Type>"),
                Item::Union(_union) => f.pad("<Item::Union>"),
                Item::Use(_use) => f.pad("<Item::Use>"),
                Item::Verbatim(_stream) => f.pad("<Item::Vebatim>"),
                _ => panic!(),
            },
            Display::Ident(ident) => write!(f, "{}", ident),
            Display::GenericArgument(arg) => match arg {
                GenericArgument::Lifetime(lifetime) => {
                    write!(f, "{}", Display::Ident(lifetime.ident.clone()))
                }
                GenericArgument::Type(type_) => write!(f, "{}", Display::Type(type_.clone())),
                GenericArgument::Const(_expr) => f.pad("<GenericArgument::Const>"),
                GenericArgument::AssocType(_type) => f.pad("<GenericArgument::AssocType>"),
                GenericArgument::AssocConst(_const) => f.pad("<GenericArgument::AssocConst>"),
                GenericArgument::Constraint(_constraint) => f.pad("<GenericArgument::Constraint>"),
                _ => panic!(),
            },
            Display::Type(type_) => match type_ {
                Type::Array(_array) => f.pad("[T; n]"),
                Type::BareFn(_fn) => f.pad("bool"),
                Type::Group(_group) => f.pad("-Group-"),
                Type::ImplTrait(_trait) => f.pad("-Bound-"),
                Type::Infer(_infer) => f.pad("_"),
                Type::Macro(_macro) => f.pad("-Macro-"),
                Type::Never(_never) => f.pad("!"),
                Type::Paren(_paren) => f.pad("()"),
                Type::Path(_path) => {
                    if _path.path.leading_colon.is_some() {
                        write!(f, " ::").unwrap()
                    }

                    let path_len = _path.path.segments.len();

                    for (index, segment) in _path.path.segments.iter().enumerate() {
                        write!(f, "{}", Display::PathSegment(segment.clone())).unwrap();
                        if index < path_len - 1 {
                            write!(f, "::").unwrap()
                        }
                    }

                    Ok(())
                }
                Type::Ptr(_ptr) => f.pad("*type"),
                Type::Reference(_reference) => f.pad("&'a type"),
                Type::Slice(_slice) => f.pad("[T]"),
                Type::TraitObject(_trait) => f.pad("*-Bound"),
                Type::Tuple(_tuple) => f.pad("(...)"),
                Type::Verbatim(_tokens) => f.pad("-Verbatim-"),
                _ => panic!(),
            },
            Display::PathSegment(segment) => {
                write!(f, "{}", segment.ident).unwrap();

                match &segment.arguments {
                    None => (),
                    AngleBracketed(generic) => {
                        let args = &generic.args;

                        write!(f, "<").unwrap();
                        for arg in args {
                            write!(f, "{}", Display::GenericArgument(arg.clone())).unwrap()
                        }
                        write!(f, ">").unwrap();
                    }
                    Parenthesized(_args) => write!(f, "<>").unwrap(),
                }

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {}
