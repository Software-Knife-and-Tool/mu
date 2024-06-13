use {
    crate::{format::Format, syntax::Syntax},
    std::fmt,
    syn::{self, Item},
};

impl fmt::Debug for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
                        "<Item::ItemFn:{}{} ({}){}>",
                        attrs,
                        &fn_.sig.ident,
                        Syntax::fn_arg_signature(&fn_.sig),
                        Syntax::fn_return_signature(&fn_.sig),
                    )
                }
                Item::ForeignMod(_mod) => f.pad("<Item::ForeignMod>"),
                Item::Impl(_impl) => {
                    for impl_ in &_impl.items {
                        write!(f, "{:?}", Syntax::ImplItem(impl_.clone())).unwrap()
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
            _ => panic!(),
        }
    }
}
