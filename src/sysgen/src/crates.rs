//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use {
    crate::{options::Options, symbols::Symbols},
    capitalize::Capitalize,
    public_api::PublicApi,
    std::{
        fs::File,
        io::{Error, ErrorKind, Read},
        result::Result,
    },
};

pub struct Crate {
    pub name: String,
    pub sysgen: String,
    pub rustdoc: String,
    pub symbols: PublicApi,
}

impl Crate {
    pub fn with_options(options: &Options, name: &str, sysgen: &str) -> Result<Crate, Error> {
        let rustdoc = rustdoc_json::Builder::default()
            .toolchain("nightly")
            .manifest_path(format!("{name}/Cargo.toml"))
            .build()
            .unwrap();

        let symbols = public_api::Builder::from_rustdoc_json(rustdoc.clone())
            .build()
            .unwrap();

        if options.is_opt("verbose") {
            for symbol in symbols.items() {
                println!("{symbol}");
            }
        }

        Ok(Crate {
            name: name.to_string(),
            sysgen: sysgen.to_string(),
            rustdoc: rustdoc.clone().display().to_string(),
            symbols,
        })
    }

    fn prototypes(&self, _stab: &Symbols) -> String {
        String::new()
    }

    fn functions(&self, _stab: &Symbols) -> String {
        String::new()
    }

    pub fn genbind(&self, _options: &Options) -> Result<(), Error> {
        let mut out = File::create(format!("{}/{}.rs", self.sysgen, self.name))?;

        let mut source = String::new();
        File::open("/opt/mu/lib/sysgen/ffi")?.read_to_string(&mut source)?;

        let mut engine = upon::Engine::new();
        match engine.add_template("ffi", source) {
            Ok(_) => (),
            Err(_) => panic!(),
        }

        let stab = Symbols::new(self);

        match engine
            .template("ffi")
            .render(upon::value! {
                crate: {
                    name: self.name.to_string(),
                    symbols: self.name.to_uppercase(),
                    struct_: self.name.capitalize(),
                    prototypes: self.prototypes(&stab),
                    functions: self.functions(&stab),
                }
            })
            .to_writer(&mut out)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

#[cfg(test)]
mod tests {}
