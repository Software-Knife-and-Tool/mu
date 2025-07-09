//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//!
//! The mu library is the implementation surface for the [`mu programming environment`] and
//! implements the *mu* and *features* namespaces.
//!
//! As much as is practible, *mu's* functions and data types resemble Common Lisp in order to be
//! familiar to the traditional Lisp programmer.
//!
//! mu is an immutable, lexically scoped Lisp-1 kernel porting layer for an ascending tower of
//! Lisp languages. While it is possible to do some useful application work directly in the *mu*
//! language, *mu* defers niceties like macros, closures, and rest functions to libraries and
//! compilers layered on top of it. See [`mu programming environment`] for details.
//!
//! library characteristics:
//! - mostly-safe Rust
//! - 64 bit tagged objects
//! - garbage collected heap
//! - lambda compiler
//! - multiple independent execution contexts
//! - s-expression reader/printer
//! - symbol namespaces
//!
//! library data types:
//!    56 bit immediate signed fixnums
//!    Lisp-1 namespaced symbols
//!    character, string, and byte streams
//!    immediate ASCII characters
//!    conses (can be immediate)
//!    fixed arity functions
//!    lambdas with lexical variables
//!    general and specialized vectors
//!    immediate strings (seven character limit)
//!    immediate keywords (seven character limit)
//!    immediate single float 32 bit IEEE float
//!    structs
//!
//! documentation:
//!    see doc/refcards and doc/rustdoc
//!
//! [`mu programming environment`]: <https://github.com/Software-Knife-and-Tool/mu>
//!
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate modular_bitfield;

mod features;
mod mu;
mod streams;
mod types;
mod vectors;

use {
    crate::{
        mu::{
            apply::Apply as _,
            compile::Compile,
            config::Config,
            core::{CORE, VERSION},
            exception,
        },
        streams::{read::Read as _, stream::StreamBuilder, write::Write as _},
        types::stream::Stream,
    },
    std::fs,
};

/// The library API
///
/// The library API exposes these types:
/// - Condition, enumeration of exceptional conditions
/// - Core, CORE runtime state
/// - Env, execution environment
/// - Exception, exception state
/// - Mu, environment and API namespace
/// - Result, specialized result for API functions that can fail
/// - Tag, tagged data representation
///   tagged data representation
pub type Tag = mu::types::Tag;
/// environment
pub type Env = mu::env::Env;
/// API function Result
pub type Result = mu::exception::Result<Tag>;
/// condition enumeration
pub type Condition = mu::exception::Condition;
/// Exception representation
pub type Exception = mu::exception::Exception;
/// Core representation
pub type Core = mu::core::Core;

/// environment and API namespace
pub struct Mu;

impl Mu {
    /// version
    pub const VERSION: &'static str = VERSION;

    /// core image
    pub fn core() -> &'static Core {
        &CORE
    }

    /// environment configuration
    pub fn config(config: Option<String>) -> Option<Config> {
        Config::new(config)
    }

    /// env constructor
    pub fn make_env(config: &Config) -> Env {
        mu::env::Env::new(config)
    }

    /// apply a function to a list of arguments
    pub fn apply(env: &Env, func: Tag, args: Tag) -> exception::Result<Tag> {
        env.apply(func, args)
    }

    /// test tagged s-expressions for strict equality
    pub fn eq(tag: Tag, tag1: Tag) -> bool {
        tag.eq_(&tag1)
    }

    /// evaluate an s-expression
    pub fn eval(env: &Env, expr: Tag) -> exception::Result<Tag> {
        env.eval(expr)
    }

    /// compile an s-expression
    pub fn compile(env: &Env, expr: Tag) -> exception::Result<Tag> {
        env.compile(expr, &mut vec![])
    }

    /// read an s-expression from a core stream
    pub fn read(
        env: &Env,
        stream: Tag,
        eof_error_p: bool,
        eof_value: Tag,
    ) -> exception::Result<Tag> {
        env.read_stream(stream, eof_error_p, eof_value, false)
    }

    /// convert a &str to a tagged s-expression
    pub fn read_str(env: &Env, str: &str) -> exception::Result<Tag> {
        let stream = StreamBuilder::new()
            .string(str.into())
            .input()
            .build(env, &CORE)?;

        env.read_stream(stream, true, Tag::nil(), false)
    }

    /// write an s-expression to a core stream
    pub fn write(env: &Env, expr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        env.write_stream(expr, escape, stream)
    }

    /// write an &str to a core stream
    pub fn write_str(env: &Env, str: &str, stream: Tag) -> exception::Result<()> {
        env.write_string(str, stream)
    }

    /// write an s-expression to a String
    pub fn write_to_string(env: &Env, expr: Tag, esc: bool) -> String {
        let str_stream = match StreamBuilder::new()
            .string("".into())
            .output()
            .build(env, &CORE)
        {
            Ok(stream) => {
                let str_tag = stream;

                Self::write(env, expr, esc, str_tag).unwrap();
                str_tag
            }
            Err(_) => panic!(),
        };

        Stream::get_string(env, str_stream).unwrap()
    }

    /// return the standard-input core stream
    pub fn std_in() -> Tag {
        CORE.stdin()
    }

    /// return the standard-output core stream
    pub fn std_out() -> Tag {
        CORE.stdout()
    }

    /// return the error-output core stream
    pub fn err_out() -> Tag {
        CORE.errout()
    }

    /// eval &str
    pub fn eval_str(env: &Env, expr: &str) -> exception::Result<Tag> {
        env.eval(Self::compile(env, Self::read_str(env, expr)?)?)
    }

    /// format exception
    pub fn exception_string(env: &Env, ex: Exception) -> String {
        format!(
            "error: condition {:?} on {} raised by {}",
            ex.condition,
            Self::write_to_string(env, ex.object, true),
            Self::write_to_string(env, ex.source, true),
        )
    }

    /// load source file
    pub fn load(env: &Env, file_path: &str) -> exception::Result<bool> {
        if fs::metadata(file_path).is_ok() {
            let load_form = format!("(mu:open :file :input \"{file_path}\" :t)");
            let istream = env.eval(Self::read_str(env, &load_form)?)?;
            let eof_value = Self::read_str(env, ":eof")?; // need make_symbol here

            loop {
                let form = Self::read(env, istream, false, eof_value)?;

                if Self::eq(form, eof_value) {
                    break Ok(true);
                }

                let compiled_form = Self::compile(env, form)?;

                env.eval(compiled_form)?;
            }
        } else {
            Err(Exception::new(
                env,
                Condition::Open,
                "load",
                Self::read_str(env, &format!("\"{file_path}\""))?,
            ))
        }
    }
}
