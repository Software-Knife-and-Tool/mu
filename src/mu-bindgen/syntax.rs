use {
    crate::{bindgen::BindGen, display::Display},
    // json::{self, object::Object, JsonValue},
    syn::{self, Item},
};

pub trait Syntax {
    fn indent(_: u16);
    fn print_item(&self, _: &Item, _: u16);
}

impl Syntax for BindGen<'_> {
    fn indent(indent: u16) {
        for _ in 0..indent {
            print!("  ")
        }
    }

    fn print_item(&self, item: &Item, indent: u16) {
        Self::indent(indent + 1);
        match item {
            Item::Const(_const) => println!("Item::Const/"),
            Item::Enum(_enum) => println!("Item::Enum/"),
            Item::ExternCrate(_crate) => println!("Item::ExternCrate/"),
            Item::Fn(_) => {
                println!("Item::Fn/");
                Self::indent(indent + 2);

                println!("fn {}", Display::Item(item.clone()))
            }
            Item::ForeignMod(_mod) => println!("Item::ForeignMod/"),
            Item::Impl(_impl) => {
                println!("Item::Impl/");
                Self::indent(indent - 1);

                for impl_ in &_impl.items {
                    Self::indent(indent + 2);
                    println!("{}", Display::ImplItem(impl_.clone()))
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
            _ => println!("what?"),
        }
    }
}

#[cfg(test)]
mod tests {}
