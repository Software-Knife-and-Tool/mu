//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use json::{self, object};

pub struct HeapInfo {
    pub config: String,
    pub image: Vec<u8>,
    pub meta: Vec<u8>,
}

impl HeapInfo {
    pub fn to_json(&self) -> Option<String> {
        let allocator = object! {
            config: self.config.clone(),
            image: self.image.clone(),  // this is an impressively bad idea
            meta: self.meta.clone(),
        };

        Some(allocator.dump())
    }

    /*
        pub fn from_json(_json: String) -> Option<Self> {
            None
    }
        */
}

pub struct HeapInfoBuilder {
    config: Option<String>,
    image: Option<Vec<u8>>,
    meta: Option<Vec<u8>>,
}

impl HeapInfoBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            image: None,
            meta: None,
        }
    }

    pub fn config(&mut self, config: String) -> &mut Self {
        self.config = Some(config);
        self
    }

    pub fn image(&mut self, image: Vec<u8>) -> &mut Self {
        self.image = Some(image);
        self
    }

    pub fn meta(&mut self, meta: Vec<u8>) -> &mut Self {
        self.meta = Some(meta);
        self
    }

    pub fn build(&self) -> HeapInfo {
        HeapInfo {
            config: match &self.config {
                Some(string) => string.to_string(),
                None => "no-config".to_string(),
            },
            image: match &self.image {
                Some(vec) => vec.to_vec(),
                None => vec![],
            },
            meta: match &self.meta {
                Some(vec) => vec.to_vec(),
                None => vec![],
            },
        }
    }
}

#[cfg(test)]
mod tests {}
