//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//!
//! The core library, *mu*, is the implementation surface for the [`mu programming environment`].
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

mod core;
mod features;
mod heaps;
mod streams;
mod types;
mod vectors;

use futures::executor::block_on;
use {
    crate::{
        core::{
            apply::Apply as _,
            compile::Compile,
            config::Config,
            core::{Core, CORE},
            exception,
        },
        heaps::image::Image,
        streams::{read::Read as _, stream::StreamBuilder, write::Write as _},
        types::stream::Stream,
    },
    std::fs,
};

/// The core API
///
/// The core API exposes these types:
/// - Condition, enumeration of exceptional conditions
/// - Exception, exception state
/// - Lib, environment and API namespace
/// - Result, specialized result for API functions that can fail
/// - Tag, tagged data representation
///   tagged data representation
pub type Tag = core::types::Tag;
/// API function Result
pub type Result = core::exception::Result<Tag>;
/// condition enumeration
pub type Condition = core::exception::Condition;
/// Exception representation
pub type Exception = core::exception::Exception;

/// the Env struct abstracts the library struct
pub struct Env(Tag);

impl Env {
    /// current version
    pub const VERSION: &'static str = core::core::Core::VERSION;

    /// turn on ^C exception signalling
    pub fn signal_exception() {
        // Exception::signal_exception()
    }

    /// environment configuration
    pub fn config(config: Option<String>) -> Option<Config> {
        Config::new(config)
    }

    /// constructor
    pub fn new(config: Config, image: Option<(Vec<u8>, Vec<u8>)>) -> Self {
        Env(Core::add_env(core::env::Env::new(config, image)))
    }

    /// apply a function to a list of arguments
    pub fn apply(&self, func: Tag, args: Tag) -> exception::Result<Tag> {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.apply(func, args)
    }

    /// test tagged s-expressions for strict equality
    pub fn eq(&self, tag: Tag, tag1: Tag) -> bool {
        tag.eq_(&tag1)
    }

    /// evaluate an s-expression
    pub fn eval(&self, expr: Tag) -> exception::Result<Tag> {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.eval(expr)
    }

    /// compile an s-expression
    pub fn compile(&self, expr: Tag) -> exception::Result<Tag> {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.compile(expr, &mut vec![])
    }

    /// read an s-expression from a core stream
    pub fn read(&self, stream: Tag, eof_error_p: bool, eof_value: Tag) -> exception::Result<Tag> {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.read_stream(stream, eof_error_p, eof_value, false)
    }

    /// convert a &str to a tagged s-expression
    pub fn read_str(&self, str: &str) -> exception::Result<Tag> {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        let stream = StreamBuilder::new()
            .string(str.into())
            .input()
            .build(env, &CORE)?;

        env.read_stream(stream, true, Tag::nil(), false)
    }

    /// write an s-expression to a core stream
    pub fn write(&self, expr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.write_stream(expr, escape, stream)
    }

    /// write an &str to a core stream
    pub fn write_str(&self, str: &str, stream: Tag) -> exception::Result<()> {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.write_string(str, stream)
    }

    /// write an s-expression to a String
    pub fn write_to_string(&self, expr: Tag, esc: bool) -> String {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        let str_stream = match StreamBuilder::new()
            .string("".into())
            .output()
            .build(env, &CORE)
        {
            Ok(stream) => {
                let str_tag = stream;

                self.write(expr, esc, str_tag).unwrap();
                str_tag
            }
            Err(_) => panic!(),
        };

        Stream::get_string(env, str_stream).unwrap()
    }

    /// return the standard-input core stream
    pub fn std_in(&self) -> Tag {
        CORE.stdin()
    }

    /// return the standard-output core stream
    pub fn std_out(&self) -> Tag {
        CORE.stdout()
    }

    /// return the error-output core stream
    pub fn err_out(&self) -> Tag {
        CORE.errout()
    }

    /// eval &str
    pub fn eval_str(&self, expr: &str) -> exception::Result<Tag> {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.eval(self.compile(self.read_str(expr)?)?)
    }

    /// format exception
    pub fn exception_string(&self, ex: Exception) -> String {
        format!(
            "error: condition {:?} on {} raised by {}",
            ex.condition,
            self.write_to_string(ex.object, true),
            self.write_to_string(ex.source, true),
        )
    }

    /// load source file
    pub fn load(&self, file_path: &str) -> exception::Result<bool> {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        if fs::metadata(file_path).is_ok() {
            let load_form = format!("(mu:open :file :input \"{}\" :t)", file_path);
            let istream = env.eval(self.read_str(&load_form)?)?;
            let eof_value = self.read_str(":eof")?; // need make_symbol here

            drop(env_ref);

            loop {
                let form = self.read(istream, false, eof_value)?;

                if self.eq(form, eof_value) {
                    break Ok(true);
                }

                let compiled_form = self.compile(form)?;

                self.eval(compiled_form)?;
            }
        } else {
            Err(Exception::new(
                env,
                Condition::Open,
                "load",
                self.read_str(&format!("\"{}\"", file_path))?,
            ))
        }
    }

    /// get environment image
    pub fn image(&self) -> exception::Result<(Vec<u8>, Vec<u8>)> {
        let env_ref = block_on(CORE.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        Ok(Image::image(env))
    }
}
