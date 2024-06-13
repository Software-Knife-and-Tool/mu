#![allow(dead_code)]
use {
    crate::{bindings::Bindings, config::Config, format::Format, syntax::Syntax},
    std::{
        cell::RefCell,
        fmt,
        fs::File,
        io::{Error, Write},
        result::Result,
    },
    syn::{self, ImplItem, Item, Visibility},
};

pub struct SymbolTable {
    pub symbols: RefCell<Vec<SymbolDescription>>,
}

pub struct SymbolDescription {
    pub type_: String,
    pub name: String,
    pub value: String,
    pub attrs: String,
}

impl fmt::Display for SymbolDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:18}{:20}{:32}{}",
            self.type_, self.attrs, self.name, self.value,
        )
    }
}

impl SymbolTable {
    pub fn new(_config: &Config, bindgen: &Bindings) -> Self {
        let symbol_table = SymbolTable {
            symbols: RefCell::new(Vec::<SymbolDescription>::new()),
        };

        for item in bindgen.syntax.items.iter() {
            symbol_table.parse_item(item)
        }

        symbol_table
    }

    fn push(&self, type_: &str, name: &str, value: &str, attrs: &str) {
        self.symbols.borrow_mut().push(SymbolDescription {
            type_: type_.to_string(),
            name: name.to_string(),
            value: value.to_string(),
            attrs: attrs.to_string(),
        })
    }

    fn parse_implitem(&self, item: &ImplItem) {
        match item {
            ImplItem::Const(const_) => {
                let name = &const_.ident;
                let _is_public = matches!(const_.vis, Visibility::Public(_));

                self.push("ImplItem::Const", &name.to_string(), "<fill-me-out>", "")
            }
            ImplItem::Fn(fn_) => {
                let name = &fn_.sig.ident;

                let value = format!(
                    "({}){}",
                    Syntax::fn_arg_signature(&fn_.sig),
                    match Syntax::fn_return_type(&fn_.sig.output.clone()) {
                        std::option::Option::None => "".to_string(),
                        Some(str) => format!(" -> {}", str),
                    },
                );

                let attrs = Syntax::fn_attrs(&fn_.sig, &fn_.vis)
                    .iter()
                    .map(|val| val.to_string())
                    .collect::<String>();

                self.push("ImplItem::Fn", &name.to_string(), &value, &attrs)
            }
            ImplItem::Type(_impl) => (), // self.push("ImplItem::Type", "", "", ""),
            ImplItem::Macro(_impl) => (), // self.push("ImplItem::Macro", "", "", ""),
            ImplItem::Verbatim(_tokens) => (), // self.push("ImplItem::Verbatim", "", "", ""),
            _ => panic!(),
        }
    }

    fn parse_item(&self, item: &Item) {
        match item {
            Item::Const(_const) => self.push("Item::Const", "", "", ""),
            Item::Enum(_enum) => (), // self.push("Item::Enum", "", "", ""),
            Item::ExternCrate(_crate) => (), // self.push("Item::ExternCrate", "", "", ""),
            Item::Fn(fn_) => {
                let name = &fn_.sig.ident;

                let value = format!(
                    "({}){}",
                    Syntax::fn_arg_signature(&fn_.sig),
                    match Syntax::fn_return_type(&fn_.sig.output.clone()) {
                        std::option::Option::None => "".to_string(),
                        Some(str) => format!(" -> {}", str),
                    },
                );

                let attrs = Syntax::fn_attrs(&fn_.sig, &fn_.vis)
                    .iter()
                    .map(|val| val.to_string())
                    .collect::<String>();

                self.push("Item::Fn", &name.to_string(), &value, &attrs)
            }
            Item::ForeignMod(_mod) => self.push("Item::ForeignMod", "", "", ""),
            Item::Impl(_impl) => {
                for impl_ in &_impl.items {
                    self.parse_implitem(impl_)
                }
            }
            Item::Macro(_macro) => (), // self.push("Item::Macro", "", "", ""),
            Item::Mod(_mod) => (),     // self.push("Item::Mod", "", "", ""),
            Item::Static(_static) => (), // self.push("Item::Static", "", "", ""),
            Item::Struct(_struct) => (), // self.push("Item::Struct", "", "", ""),
            Item::Trait(_trait) => (), // self.push("Item::Trait", "", "", ""),
            Item::TraitAlias(_alias) => (), // self.push("Item::TraitAlias", "", "", ""),
            Item::Type(_type) => (),   // self.push("Item::Type", "", "", ""),
            Item::Union(_union) => (), // self.push("Item::Union", "", "", ""),
            Item::Use(_use) => (),     // self.push("Item::Use", "", "", ""),
            Item::Verbatim(_stream) => (), // self.push("Item::Vebatim", "", "", ""),
            _ => panic!(),
        }
    }

    pub fn write(&self, path: &str) -> Result<(), Error> {
        let mut out = File::create(path)?;

        for symbol in (*self.symbols.borrow()).iter() {
            out.write_all(format!("{}\n", symbol).as_bytes())?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
