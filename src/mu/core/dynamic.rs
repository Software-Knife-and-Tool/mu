//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! function call frame
//!    Frame
//!    apply
//!    frame_push
//!    frame_pop
//!    frame_ref
use crate::{
    core::{
        env::Env,
        exception::{self},
        frame::Frame,
        types::Tag,
    },
    types::{
        cons::{Cons, Core as _},
        indirect_vector::Core as _,
        vector::Vector,
    },
};

use futures::executor::block_on;

impl Env {
    pub fn dynamic_push(&self, func: Tag, offset: usize) {
        let mut dynamic_ref = block_on(self.dynamic.write());

        dynamic_ref.push((func.as_u64(), offset));
    }

    pub fn dynamic_pop(&self) {
        let mut dynamic_ref = block_on(self.dynamic.write());

        dynamic_ref.pop();
    }

    #[allow(dead_code)]
    pub fn dynamic_ref(&self, index: usize) -> (Tag, usize) {
        let dynamic_ref = block_on(self.dynamic.read());

        let (func, offset) = dynamic_ref[index];

        ((&func.to_le_bytes()).into(), offset)
    }
}

pub trait CoreFunction {
    fn mu_frames(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn mu_frames(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let frames_ref = block_on(env.dynamic.read());
        let mut frames = Vec::new();

        frames.extend(frames_ref.iter().map(|(func, offset)| {
            let mut argv = Vec::new();

            Frame::frame_stack_ref(env, (&func.to_le_bytes()).into(), *offset, &mut argv);

            let vec: Vec<Tag> = argv
                .into_iter()
                .map(|f| (&f.to_le_bytes()).into())
                .collect();

            Cons::new((&func.to_le_bytes()).into(), Vector::from(vec).evict(env)).evict(env)
        }));

        fp.value = Cons::vlist(env, &frames);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dynamic() {
        assert_eq!(2 + 2, 4);
    }
}
