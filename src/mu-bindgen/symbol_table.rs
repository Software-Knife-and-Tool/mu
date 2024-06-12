#![allow(dead_code)]
use {
    crate::{bindings::Bindings, config::Config, syntax::Syntax},
    std::{
        cell::RefCell,
        fmt,
        fs::File,
        io::{Error, Write},
        result::Result,
    },
    syn::{self, punctuated::Pair, token::Comma, FnArg, ImplItem, Item, Signature, Visibility},
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
            "{:32}{:16}{:40}{:16}",
            self.name, self.type_, self.value, self.attrs,
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

    fn fn_arg_signature(sig: &Signature) -> String {
        sig.inputs
            .pairs()
            .map(|pair: Pair<&FnArg, &Comma>| {
                let value = pair.value();
                let _punct = pair.punct();
                println!("{:#?}", value);
                "hello, ".to_string()
            })
            .collect::<String>()
    }

    fn fn_return_signature(sig: &Signature) -> String {
        match Syntax::return_type(&sig.output.clone()) {
            std::option::Option::None => "".to_string(),
            Some(str) => format!(" -> {}", str),
        }
    }

    fn parse_implitem(&self, item: &ImplItem) {
        match item {
            ImplItem::Const(const_) => {
                let name = &const_.ident;
                let _is_public = matches!(const_.vis, Visibility::Public(_));

                self.push("const", &name.to_string(), "<fill-me-out>", "")
            }
            ImplItem::Fn(fn_) => {
                let name = &fn_.sig.ident;

                let is_public = matches!(fn_.vis, Visibility::Public(_));
                let is_const = fn_.sig.constness.is_some();
                let is_async = fn_.sig.asyncness.is_some();

                let value = format!(
                    "({}){}",
                    Self::fn_arg_signature(&fn_.sig),
                    match Syntax::return_type(&fn_.sig.output.clone()) {
                        std::option::Option::None => "".to_string(),
                        Some(str) => format!(" -> {}", str),
                    },
                );

                let attrs = format!(
                    "{}{}{}",
                    if is_const { "const " } else { "" },
                    if is_async { "async " } else { "" },
                    if is_public { "pub " } else { "" },
                );

                self.push("fn", &name.to_string(), &value, &attrs)
            }
            ImplItem::Type(_impl) => self.push("ImplItem::Type", "", "", ""),
            ImplItem::Macro(_impl) => self.push("ImplItem::Macro", "", "", ""),
            ImplItem::Verbatim(_tokens) => self.push("ImplItem::Verbatim", "", "", ""),
            _ => panic!(),
        }
    }

    fn parse_item(&self, item: &Item) {
        match item {
            Item::Const(_const) => self.push("Item::Const", "", "", ""),
            Item::Enum(_enum) => self.push("Item::Enum", "", "", ""),
            Item::ExternCrate(_crate) => self.push("Item::ExternCrate", "", "", ""),
            Item::Fn(fn_) => {
                let name = &fn_.sig.ident;

                let is_public = matches!(fn_.vis, Visibility::Public(_));
                let is_const = fn_.sig.constness.is_some();
                let is_async = fn_.sig.asyncness.is_some();

                let value = format!(
                    "({}){}",
                    Self::fn_arg_signature(&fn_.sig),
                    match Syntax::return_type(&fn_.sig.output.clone()) {
                        std::option::Option::None => "".to_string(),
                        Some(str) => format!(" -> {}", str),
                    },
                );

                let attrs = format!(
                    "{}{}{}",
                    if is_const { "const " } else { "" },
                    if is_async { "async " } else { "" },
                    if is_public { "pub " } else { "" },
                );

                self.push("Item::Fn", &name.to_string(), &value, &attrs)
            }
            Item::ForeignMod(_mod) => self.push("Item::ForeignMod", "", "", ""),
            Item::Impl(_impl) => {
                for impl_ in &_impl.items {
                    self.parse_implitem(impl_)
                }
            }
            Item::Macro(_macro) => self.push("Item::Macro", "", "", ""),
            Item::Mod(_mod) => self.push("Item::Mod", "", "", ""),
            Item::Static(_static) => self.push("Item::Static", "", "", ""),
            Item::Struct(_struct) => self.push("Item::Struct", "", "", ""),
            Item::Trait(_trait) => self.push("Item::Trait", "", "", ""),
            Item::TraitAlias(_alias) => self.push("Item::TraitAlias", "", "", ""),
            Item::Type(_type) => self.push("Item::Type", "", "", ""),
            Item::Union(_union) => self.push("Item::Union", "", "", ""),
            Item::Use(_use) => self.push("Item::Use", "", "", ""),
            Item::Verbatim(_stream) => self.push("Item::Vebatim", "", "", ""),
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
