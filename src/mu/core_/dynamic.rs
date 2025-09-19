//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// dynamic state
use {
    crate::core_::{env::Env, tag::Tag},
    futures_lite::future::block_on,
};

pub struct Dynamic;

impl Dynamic {
    pub fn dynamic_push(env: &Env, func: Tag, offset: usize) {
        let mut dynamic_ref = block_on(env.dynamic.write());

        dynamic_ref.push((func.as_u64(), offset));
    }

    pub fn dynamic_pop(env: &Env) {
        let mut dynamic_ref = block_on(env.dynamic.write());

        dynamic_ref.pop();
    }

    #[allow(dead_code)]
    pub fn dynamic_ref(env: &Env, index: usize) -> (Tag, usize) {
        let dynamic_ref = block_on(env.dynamic.read());

        let (func, offset) = dynamic_ref[index];

        ((&func.to_le_bytes()).into(), offset)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dynamic() {
        assert!(true);
    }
}
