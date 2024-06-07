use {
    crate::{bindgen::BindGen, display::Display},
    syn::{self, Item},
};

pub trait Syntax {
    fn pprint(&self, _: &Item, _: u16);
    fn print_file(&self, path: &str);
}

impl Syntax for BindGen<'_> {
    fn print_file(&self, path: &str) {
        println!("{path}/");
        for item in &self.syntax.items {
            self.pprint(item, 0)
        }
    }

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
            Item::Fn(_) => {
                println!("Item::Fn/");
                indent(n + 2);

                println!("fn {}", Display::Item(item.clone()))
            }
            Item::ForeignMod(_mod) => println!("Item::ForeignMod/"),
            Item::Impl(_impl) => {
                println!("Item::Impl/");
                indent(n - 1);

                for impl_ in &_impl.items {
                    indent(n + 2);
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
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {}
