use {
    crate::{format::Format, syntax::Syntax},
    std::fmt,
    syn::{
        self, GenericArgument, ImplItem, Item,
        PathArguments::{AngleBracketed, None, Parenthesized},
        Type,
    },
};

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Syntax::ImplItem(impl_item) => match impl_item {
                ImplItem::Const(_impl) => write!(f, "ImplItem::Const"),
                ImplItem::Fn(fn_) => {
                    let attrs = Syntax::fn_attrs(&fn_.sig, &fn_.vis)
                        .iter()
                        .map(|val| val.to_string())
                        .collect::<String>();

                    write!(
                        f,
                        "{}fn {}({}){}",
                        attrs,
                        &fn_.sig.ident,
                        Syntax::fn_arg_signature(&fn_.sig),
                        Syntax::fn_return_signature(&fn_.sig),
                    )
                }
                ImplItem::Type(_impl) => write!(f, "ImplItem::Type"),
                ImplItem::Macro(_impl) => write!(f, "ImplItem::Macro"),
                ImplItem::Verbatim(_tokens) => write!(f, "ImplItem::Verbatim"),
                _ => panic!(),
            },
            Syntax::Ident(ident) => write!(f, "{}", ident),
            Syntax::Item(item) => match item {
                Item::Const(_const) => f.pad("<Item::Const>"),
                Item::Enum(_enum) => f.pad("<Item::Enum>"),
                Item::ExternCrate(_crate) => f.pad("<Item::ExternCrate>"),
                Item::Fn(fn_) => {
                    let attrs = Syntax::fn_attrs(&fn_.sig, &fn_.vis)
                        .iter()
                        .map(|val| val.to_string())
                        .collect::<String>();

                    write!(
                        f,
                        "{}fn {}({}){}",
                        attrs,
                        &fn_.sig.ident,
                        Syntax::fn_arg_signature(&fn_.sig),
                        Syntax::fn_return_signature(&fn_.sig),
                    )
                }
                Item::ForeignMod(_mod) => f.pad("<Item::ForeignMod>"),
                Item::Impl(_impl) => {
                    for impl_ in &_impl.items {
                        write!(f, "{}", Syntax::ImplItem(impl_.clone())).unwrap()
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
            Syntax::GenericArgument(arg) => match arg {
                GenericArgument::Lifetime(lifetime) => {
                    write!(f, "{}", Syntax::Ident(lifetime.ident.clone()))
                }
                GenericArgument::Type(type_) => write!(f, "{}", Syntax::Type(type_.clone())),
                GenericArgument::Const(_expr) => f.pad("<GenericArgument::Const>"),
                GenericArgument::AssocType(_type) => f.pad("<GenericArgument::AssocType>"),
                GenericArgument::AssocConst(_const) => f.pad("<GenericArgument::AssocConst>"),
                GenericArgument::Constraint(_constraint) => f.pad("<GenericArgument::Constraint>"),
                _ => panic!(),
            },
            Syntax::Type(type_) => match type_ {
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
                        write!(f, "{}", Syntax::PathSegment(segment.clone())).unwrap();
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
            Syntax::PathSegment(segment) => {
                write!(f, "{}", segment.ident).unwrap();

                match &segment.arguments {
                    None => (),
                    AngleBracketed(generic) => {
                        let args = &generic.args;

                        write!(f, "<").unwrap();
                        for arg in args {
                            write!(f, "{}", Syntax::GenericArgument(arg.clone())).unwrap()
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
