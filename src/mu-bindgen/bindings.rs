use {
    crate::{config::Config, symbol_table::SymbolTable},
    capitalize::Capitalize,
    json::{self, JsonValue},
    std::{
        fs::{self, File},
        io::{Error, ErrorKind, Read},
        result::Result,
    },
};

pub struct Bindings<'a> {
    pub bindmap: Option<JsonValue>,
    pub config: &'a Config,
    pub syntax: syn::File,
}

impl Bindings<'_> {
    fn parse(path: &str) -> Result<syn::File, Error> {
        let source = fs::read(path)?;

        match syn::parse_file(&String::from_utf8(source).unwrap()) {
            Ok(syntax) => Ok(syntax),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    pub fn with_config<'a>(config: &'a Config, src_path: &str) -> Result<Bindings<'a>, Error> {
        let bindmap = match config.bindmap_path() {
            Some(path) => {
                let bindmap = fs::read(&path)?;

                match json::parse(&String::from_utf8(bindmap).unwrap()) {
                    Ok(bindmap) => Some(bindmap),
                    Err(_) => return Err(Error::new(ErrorKind::Other, path.to_string())),
                }
            }
            None => None,
        };

        let syntax = Self::parse(src_path)?;

        Ok(Bindings {
            config,
            bindmap,
            syntax,
        })
    }

    fn prototypes(&self, stab: &SymbolTable) -> String {
        let mut defs = String::new();

        for symbol in (stab.symbols.borrow()).iter() {
            if symbol.type_ == "Item::Fn" {
                defs.push_str(&format!(
                    "    {}fn {}{};\n",
                    symbol.attrs, symbol.name, symbol.value
                ));
            }
        }

        defs
    }

    fn functions(&self, _stab: &SymbolTable) -> String {
        String::new()
    }

    pub fn gencode(&self, path: &str) -> Result<(), Error> {
        let mut out = File::create(path)?;

        let mut source = String::new();
        File::open("src/mu-bindgen/templates/mu")?
            .read_to_string(&mut source)
            .unwrap();

        let mut engine = upon::Engine::new();
        match engine.add_template("ffi", source) {
            Ok(_) => (),
            Err(_) => panic!(),
        }

        /*
            let ns = match self.config.namespace() {
                Some(ns) => ns
                    .chars()
                    .filter_map(|ch| Some(if ch == '-' { '_' } else { ch }))
                    .collect(),
                None => "_".to_string(),
        };
             */

        let ns = match self.config.namespace() {
            Some(ns) => ns
                .chars()
                .map(|ch| if ch == '-' { '_' } else { ch })
                .collect(),
            None => "_".to_string(),
        };

        let stab = SymbolTable::new(self.config, self);
        match engine
            .template("ffi")
            .render(upon::value! {
                crate: {
                    name: ns.clone(),
                    symbols: ns.clone().to_uppercase(),
                    struct_: ns.clone().capitalize(),
                    prototypes: self.prototypes(&stab),
                    functions: self.functions(&stab),
                }
            })
            .to_writer(&mut out)
        {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(ErrorKind::Other, "template parse error")),
        }
    }
}

#[cfg(test)]
mod tests {}
