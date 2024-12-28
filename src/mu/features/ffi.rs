//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! ffi interface
use crate::features::feature::Feature;

pub trait Ffi {
    fn feature() -> Feature;
}

impl Ffi for Feature {
    fn feature() -> Feature {
        Feature {
            symbols: Vec::new(),
            namespace: "ffi".into(),
        }
    }
}

#[cfg(test)]
mod tests {}
