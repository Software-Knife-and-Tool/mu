//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! lib bindings
#![allow(unused_imports)]
use {
    crate::{
        core::{
            apply::Apply,
            compile::Compile,
            config::Config,
            core::{Core, CORE, CORE_FUNCTIONS},
            dynamic::Dynamic,
            env,
            exception::{self, Condition, Exception},
            frame::Frame,
            heap::HeapAllocator,
            mu,
            namespace::Namespace,
            types::Tag,
        },
        streams::{read::Read as _, stream::StreamBuilder, write::Write as _},
        types::stream::Stream,
        vectors::cache::VecCacheMap,
    },
    std::collections::HashMap,
};
use {futures_lite::future::block_on, futures_locks::RwLock};

pub type Env = (u64,);
pub struct Mu;

impl Mu {
    pub fn apply_(env: Env, func: Tag, args: Tag) -> exception::Result<Tag> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[env.0 as usize];

        Apply::apply(env_, func, args)
    }

    pub fn make_env_(config: &Config) -> Env {
        (env::Env::make(config) as u64,)
    }

    pub fn eval_(env: Env, expr: Tag) -> exception::Result<Tag> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[env.0 as usize];

        Apply::eval(env_, expr)
    }

    pub fn compile_(env: Env, expr: Tag) -> exception::Result<Tag> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[env.0 as usize];

        Compile::compile(env_, expr, &mut vec![])
    }

    pub fn read_(
        env: Env,
        stream: Tag,
        eof_error_p: bool,
        eof_value: Tag,
    ) -> exception::Result<Tag> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[env.0 as usize];

        env_.read_stream(stream, eof_error_p, eof_value, false)
    }

    pub fn read_str_(env: Env, str: &str) -> exception::Result<Tag> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[env.0 as usize];

        let stream = StreamBuilder::new()
            .string(str.into())
            .input()
            .build(env_, &CORE)?;

        env_.read_stream(stream, true, Tag::nil(), false)
    }

    pub fn write_(env: Env, expr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[env.0 as usize];

        env_.write_stream(expr, escape, stream)
    }

    pub fn write_str_(env: Env, str: &str, stream: Tag) -> exception::Result<()> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[env.0 as usize];

        env_.write_string(str, stream)
    }

    pub fn write_to_string_(env: Env, expr: Tag, esc: bool) -> String {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[env.0 as usize];

        let str_stream = match StreamBuilder::new()
            .string("".into())
            .output()
            .build(env_, &CORE)
        {
            Ok(stream) => {
                env_.write_stream(expr, esc, stream).unwrap();

                stream
            }
            Err(_) => panic!(),
        };

        Stream::get_string(env_, str_stream).unwrap()
    }

    pub fn exception_string_(env: Env, ex: Exception) -> String {
        format!(
            "error: condition {:?} on {} raised by {}",
            ex.condition,
            Self::write_to_string(env, ex.object, true),
            Self::write_to_string(env, ex.source, true),
        )
    }

    pub fn err_(env: Env, cond: Condition, reason: &str, obj: Tag) -> exception::Result<bool> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[env.0 as usize];

        Err(Exception::new(env_, cond, reason, obj))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn env() {
        assert_eq!(2 + 2, 4);
    }
}
