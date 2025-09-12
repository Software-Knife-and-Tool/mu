//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// crate bindings
use {
    crate::{
        core::{
            apply::Apply,
            compiler::Compiler,
            config::Config,
            core::{Core, CORE},
            env,
            exception::{self, Condition, Exception},
            reader::Reader,
            tag::Tag,
            writer::Writer,
        },
        streams::{builder::StreamBuilder, writer::StreamWriter},
        types::stream::Stream,
    },
    futures_lite::future::block_on,
};

pub type Env = Tag;
pub struct Mu;

impl Mu {
    pub fn make_env_(config: &Config) -> Env {
        let env = env::Env::new(config);

        Core::add_env(env)
    }

    pub fn apply_(env: Env, func: Tag, args: Tag) -> exception::Result<Tag> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[&env.as_u64()];

        Apply::apply(env_, func, args)
    }

    pub fn eval_(env: Env, expr: Tag) -> exception::Result<Tag> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[&env.as_u64()];

        Apply::eval(env_, expr)
    }

    pub fn compile_(env: Env, expr: Tag) -> exception::Result<Tag> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[&env.as_u64()];

        Compiler::compile(env_, expr, &mut vec![])
    }

    pub fn read_(
        env: Env,
        stream: Tag,
        eof_error_p: bool,
        eof_value: Tag,
    ) -> exception::Result<Tag> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[&env.as_u64()];

        env_.read(stream, eof_error_p, eof_value, false)
    }

    pub fn read_str_(env: Env, str: &str) -> exception::Result<Tag> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[&env.as_u64()];

        let stream = StreamBuilder::new()
            .string(str.into())
            .input()
            .build(env_, &CORE)?;

        env_.read(stream, true, Tag::nil(), false)
    }

    pub fn write_(env: Env, expr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[&env.as_u64()];

        env_.write(expr, escape, stream)
    }

    pub fn write_str_(env: Env, str: &str, stream: Tag) -> exception::Result<()> {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[&env.as_u64()];

        StreamWriter::write_str(env_, str, stream)
    }

    pub fn write_to_string_(env: Env, expr: Tag, esc: bool) -> String {
        let envs_ref = block_on(CORE.envs.read());
        let env_: &env::Env = &envs_ref[&env.as_u64()];

        let stream = StreamBuilder::new()
            .string("".into())
            .output()
            .build(env_, &CORE)
            .unwrap();

        env_.write(expr, esc, stream).unwrap();
        Stream::get_string(env_, stream).unwrap()
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
        let env_: &env::Env = &envs_ref[&env.as_u64()];

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
