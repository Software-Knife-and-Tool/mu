use {
    crate::{bindgen::BindGen, config::Config, display::Display},
    std::{
        cell::RefCell,
        fs::File,
        io::{Error, Write},
        result::Result,
    },
    syn::{self, Item},
};

pub struct SymbolTable {
    symbols: RefCell<Vec<(String, String, String)>>,
}

impl SymbolTable {
    pub fn new(_config: &Config, bindgen: &BindGen) -> Self {
        let symbol_table = SymbolTable {
            symbols: RefCell::new(Vec::<(String, String, String)>::new()),
        };

        for item in bindgen.syntax.items.iter() {
            symbol_table
                .symbols
                .borrow_mut()
                .push(symbol_table.parse_item(item));
        }

        symbol_table
    }

    fn parse_item(&self, item: &Item) -> (String, String, String) {
        let empty = "".to_string();

        match item {
            Item::Const(_const) => ("Item::Const".to_string(), empty.clone(), empty.clone()),
            Item::Enum(_enum) => ("Item::Enum".to_string(), empty.clone(), empty.clone()),
            Item::ExternCrate(_crate) => (
                "Item::ExternCrate".to_string(),
                empty.clone(),
                empty.clone(),
            ),
            Item::Fn(_) => (
                "Item::Fn".to_string(),
                empty.clone(),
                Display::Item(item.clone()).to_string(),
            ),
            Item::ForeignMod(_mod) => {
                ("Item::ForeignMod".to_string(), empty.clone(), empty.clone())
            }
            Item::Impl(_impl) => (empty.clone(), empty.clone(), empty.clone()),
            /*
                    for impl_ in &_impl.items {
                        println!("{}", Display::ImplItem(impl_.clone()))
            }
            }
            */
            Item::Macro(_macro) => ("Item::Macro".to_string(), empty.clone(), empty.clone()),
            Item::Mod(_mod) => ("Item::Mod".to_string(), empty.clone(), empty.clone()),
            Item::Static(_static) => ("Item::Static".to_string(), empty.clone(), empty.clone()),
            Item::Struct(_struct) => ("Item::Struct".to_string(), empty.clone(), empty.clone()),
            Item::Trait(_trait) => ("Item::Trait".to_string(), empty.clone(), empty.clone()),
            Item::TraitAlias(_alias) => {
                ("Item::TraitAlias".to_string(), empty.clone(), empty.clone())
            }
            Item::Type(_type) => ("Item::Type".to_string(), empty.clone(), empty.clone()),
            Item::Union(_union) => ("Item::Union".to_string(), empty.clone(), empty.clone()),
            Item::Use(_use) => ("Item::Use".to_string(), empty.clone(), empty.clone()),
            Item::Verbatim(_stream) => ("Item::Vebatim".to_string(), empty.clone(), empty.clone()),
            _ => panic!(),
        }
    }

    pub fn write(&self, path: &str) -> Result<(), Error> {
        let mut out = File::create(path)?;

        for (type_, name, symbol) in (*self.symbols.borrow()).iter() {
            out.write_all(format!("{}|{}|{}\n", type_, name, symbol).as_bytes())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
