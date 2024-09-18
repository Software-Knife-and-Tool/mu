//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
use {
    crate::{item::Item, options::Options},
    public_api::PublicItem,
    std::{cell::RefCell, collections::VecDeque, io::Error, result::Result},
};

#[derive(Clone, Debug)]
pub struct Crate {
    pub name: String,
    pub sysgen: String,
    pub rustdoc: String,
    pub items: RefCell<VecDeque<PublicItem>>,
}

impl Crate {
    pub fn with_options(_: &Options, name: &str, sysgen: &str) -> Result<Crate, Error> {
        let rustdoc = rustdoc_json::Builder::default()
            .toolchain("nightly")
            .manifest_path(format!("{name}/Cargo.toml"))
            .build()
            .unwrap();

        let item = public_api::Builder::from_rustdoc_json(rustdoc.clone())
            .build()
            .unwrap();

        Ok(Crate {
            name: name.to_string(),
            sysgen: sysgen.to_string(),
            rustdoc: rustdoc.clone().display().to_string(),
            items: RefCell::new(item.items().cloned().collect()),
        })
    }

    pub fn next_item(&self) -> Option<PublicItem> {
        let mut items_ref = self.items.borrow_mut();

        items_ref.pop_front()
    }

    pub fn push_item(&self, item: PublicItem) {
        let mut items_ref = self.items.borrow_mut();

        items_ref.push_front(item)
    }

    pub fn parse_next_item(&self) -> Result<Option<(Item, PublicItem)>, Error> {
        match self.next_item() {
            Some(public_item) => Ok(Some((
                Item::with_public_item(self, &public_item)?,
                public_item,
            ))),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {}
