//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use {
    crate::{
        crate_::Crate,
        item::Item,
        options::Options,
        parser::Parser,
        parsers::{enum_::Enum, fn_::Function, impl_::Impl, mod_::Mod, type_::TypeAlias},
    },
    capitalize::Capitalize,
    public_api::{tokens::Token, PublicItem},
    std::{
        cell::RefCell,
        fs::File,
        io::{Error, ErrorKind, Read, Write},
        result::Result,
    },
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum BindingItem<'a> {
    Function(Function),
    Implementation(Impl),
    Enumeration(Enum<'a>),
    Module(Mod),
    TypeAlias(TypeAlias),
}

#[allow(dead_code)]
pub struct Binding<'a> {
    pub qualifiers: Option<Vec<Token>>,
    pub name: Option<String>,
    pub item: Option<BindingItem<'a>>,
}

impl Binding<'_> {
    pub fn prototypes(_: &Vec<Binding>) -> String {
        String::new()
    }

    pub fn functions(_: &Vec<Binding>) -> String {
        String::new()
    }

    pub fn emit(crate_: &Crate, options: &Options) -> Result<(), Error> {
        let mut out = File::create(format!("{}/{}.rs", crate_.sysgen, crate_.name))?;
        let mut source = String::new();

        File::open("/opt/mu/lib/sysgen/ffi")?.read_to_string(&mut source)?;

        let mut engine = upon::Engine::new();
        match engine.add_template("ffi", source) {
            Ok(_) => (),
            Err(_) => panic!(),
        }

        let mut bindings = Vec::new();
        loop {
            match crate_.parse_next_item()? {
                Some((item, public_item)) => {
                    if options.is_opt("verbose") {
                        println!("sysgen parse: {public_item:?}")
                    }
                    bindings.push(<Item as Parser>::parse(item)?)
                }
                None => break,
            }
        }

        match engine
            .template("ffi")
            .render(upon::value! {
                crate: {
                    name: crate_.name.to_string(),
                    symbols: crate_.name.to_uppercase(),
                    struct_: crate_.name.capitalize(),
                    prototypes: Self::prototypes(&bindings),
                    functions: Self::functions(&bindings),
                }
            })
            .to_writer(&mut out)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{e:?}");
                // Err<(), Error>(Error::new(ErrorKind::Other, e)));

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {}
