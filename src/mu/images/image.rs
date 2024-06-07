//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image management
use crate::core::env::Env;
use futures::executor::block_on;

use json::{self, object};

pub struct Image {
    image: Vec<u8>,
}

impl Image {
    fn to_json(&self) -> Option<String> {
        let image = object! {
            image: self.image.clone(),
        };

        Some(image.dump())
    }

    pub fn from_json(_json: String) -> Option<Self> {
        None
    }
}

pub trait Core {
    fn image(&self) -> Vec<u8>;
}

impl Core for Env {
    // let image_data: Vec<u8> = self.image.to_json().unwrap().into_bytes();
    fn image(&self) -> Vec<u8> {
        let heap_ref = block_on(self.heap.write());
        let image = heap_ref.heap_slice();

        image.to_vec()
    }
}

#[cfg(test)]
mod tests {}
