#![allow(dead_code)]
#[allow(unused_imports)]
use {
    crate::bindings::Bindings,
    std::convert::From,
    syn::{
        self, FnArg, GenericArgument, Ident, ImplItem, Item,
        PathArguments::{AngleBracketed, None, Parenthesized},
        PathSegment, ReturnType, Signature, Type, Visibility,
    },
};

pub enum SymbolAttr {
    Pub(bool),
    Const(bool),
    Async(bool),
}

pub struct Symbol {
    type_: String,
    name: String,
    value: String,
    attrs: Vec<SymbolAttr>,
}

pub enum Syntax {
    GenericArgument(GenericArgument),
    Ident(Ident),
    Item(Item),
    ImplItem(ImplItem),
    PathSegment(PathSegment),
    Type(Type),
}

impl Syntax {
    pub fn return_type(arg: &ReturnType) -> Option<String> {
        match arg {
            ReturnType::Default => std::option::Option::None,
            ReturnType::Type(_, type_) => Some(format!("{}", Syntax::Type(*type_.clone()))),
        }
    }
}

pub trait PrettyPrint {
    #[allow(dead_code)]
    fn pprint(&self, _: &Item, _: u16);
}

impl PrettyPrint for Bindings<'_> {
    #[allow(dead_code)]
    fn pprint(&self, item: &Item, n: u16) {
        fn indent(n: u16) {
            for _ in 1..n {
                print!("  ")
            }
        }

        indent(n + 1);
        match item {
            Item::Const(_const) => println!("Item::Const/"),
            Item::Enum(_enum) => println!("Item::Enum/"),
            Item::ExternCrate(_crate) => println!("Item::ExternCrate/"),
            Item::Fn(fn_) => {
                println!("Item::Fn/");
                indent(n + 2);

                let name = &fn_.sig.ident;

                let is_public = matches!(fn_.vis, Visibility::Public(_));
                let is_const = fn_.sig.constness.is_some();
                let is_async = fn_.sig.asyncness.is_some();
                let is_unsafe = fn_.sig.unsafety.is_some();

                let output_type = match Syntax::return_type(&fn_.sig.output.clone()) {
                    std::option::Option::None => "".to_string(),
                    Some(str) => format!(" -> {}", str),
                };

                let value = format!("(...){}", output_type);

                let attrs = format!(
                    "{}{}{}{}",
                    if is_const { "const " } else { "" },
                    if is_async { "async " } else { "" },
                    if is_public { "pub " } else { "" },
                    if is_unsafe { "unsafe " } else { "" },
                );

                println!(
                    "{:20}{:8}{:40}{:16}",
                    "fn",
                    &name.to_string(),
                    &value,
                    &attrs
                )
            }
            Item::ForeignMod(_mod) => println!("Item::ForeignMod/"),
            Item::Impl(_impl) => {
                println!("Item::Impl/");
                indent(n - 1);

                for impl_ in &_impl.items {
                    indent(n + 2);
                    println!("{}", Syntax::ImplItem(impl_.clone()))
                }
            }
            Item::Macro(_macro) => println!("Item::Macro/"),
            Item::Mod(_mod) => println!("Item::Mod/"),
            Item::Static(_static) => println!("Item::Static/"),
            Item::Struct(_struct) => println!("Item::Struct/"),
            Item::Trait(_trait) => println!("Item::Trait/"),
            Item::TraitAlias(_alias) => println!("Item::TraitAlias/"),
            Item::Type(_type) => println!("Item::Type/"),
            Item::Union(_union) => println!("Item::Union/"),
            Item::Use(_use) => println!("Item::Use/"),
            Item::Verbatim(_stream) => println!("Item::Vebatim/"),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {}
