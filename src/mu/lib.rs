//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(unused_results)]

//!
//! The mu runtime library is the implementation surface for the [`mu programming environment`](<https://github.com/Software-Knife-and-Tool/mu>) and
//! provides the *mu* and *feature* namespaces.
//!
//! *mu* is an immutable, lexically scoped Lisp-1 runtime kernel and porting layer for an ascending tower of
//! Lisp languages. While it is possible to do some useful application work directly in the *mu*
//! language, *mu* defers niceties like macros, closures, and rest lambdas to libraries and
//! compilers layered on top of it. See the [`project`](<https://github.com/Software-Knife-and-Tool/mu>)
//! README for details.
//!
//! [`github`](<https://github.com/Software-Knife-and-Tool/mu>)
//!
//! About the library:
//!    - mostly-safe Rust
//!    - 64 bit tagged objects
//!    - garbage collected heap
//!    - lambda compiler
//!    - multiple independent execution contexts
//!    - thread safe
//!    - asynchronous I/O
//!    - s-expression reader/printer
//!    - symbol namespaces
//!
//! Data types:
//!    - 56 bit immediate signed fixnums
//!    - Lisp-1 namespaced symbols
//!    - character, string, and byte streams
//!    - immediate ASCII characters
//!    - conses
//!    - fixed arity functions
//!    - lambdas with lexical variables
//!    - specialized (byte, fixnum, single float, character) vectors
//!    - immediate strings (seven character limit)
//!    - immediate keywords (seven character limit)
//!    - immediate single float 32 bit IEEE float
//!    - structs
//!
//! [`documentation`](<https://github.com/Software-Knife-and-Tool/mu>)
//!
//!    see *doc/refcards* and *doc/rustdoc*
//!
#[macro_use]
extern crate modular_bitfield;

mod core;
mod features;
mod gc;
mod namespaces;
mod reader;
mod streams;
mod types;
mod vectors;

///
/// The library API exposes these types:
/// - Condition, enumeration of exceptional conditions
/// - Config, Env configuration
/// - Env, execution environment
/// - Exception, exception state
/// - Mu, environment and API namespace
/// - Result, specialized result for failable API functions
/// - Tag, tagged data representation
///   tagged data representation
pub type Mu = mu::Mu;
/// tagged data representation
pub type Tag = core::tag::Tag;
/// Environment
pub type Env = core::env::Env;
/// Exception condition enumeration
pub type Condition = core::exception::Condition;
/// Environment configuration
pub type Config = core::config::Config;
/// Exception representation
pub type Exception = core::exception::Exception;
/// API function Result
pub type Result<T> = core::exception::Result<T>;

/// API namespace
pub mod mu {
    use {
        crate::{
            core::{
                apply::Apply,
                compiler::Compiler,
                config::Config,
                core_::CORE,
                env::Env,
                exception::{self, Condition, Exception},
                tag::Tag,
            },
            reader::read::Reader,
            streams::{builder::StreamBuilder, writer::StreamWriter},
            types::stream::Stream,
        },
        std::fs,
    };

    pub struct Mu;

    impl Mu {
        /// version
        pub fn version() -> &'static str {
            env!("CARGO_PKG_VERSION")
        }

        /// Environment configuration constructor
        pub fn config(config: Option<String>) -> Config {
            Config::new(config)
        }

        /// env constructor
        pub fn env(config: &Config) -> Env {
            Env::new(config)
        }

        /// apply a function to a list of arguments
        pub fn apply(env: &Env, func: Tag, args: Tag) -> exception::Result<Tag> {
            Apply::apply(env, func, args)
        }

        /// compile an s-expression
        pub fn compile(env: &Env, expr: Tag) -> exception::Result<Tag> {
            Compiler::compile(env, expr, &mut vec![])
        }

        /// test tagged s-expressions for strict equality
        pub fn eq(tag: Tag, tag1: Tag) -> bool {
            tag.eq_(&tag1)
        }

        /// evaluate an s-expression
        pub fn eval(env: &Env, expr: Tag) -> exception::Result<Tag> {
            Apply::eval(env, expr)
        }

        /// eval &str
        pub fn eval_str(env: &Env, expr: &str) -> exception::Result<Tag> {
            Self::eval(env, Self::compile(env, Self::read_str(env, expr)?)?)
        }

        /// read a mu s-expression from a core stream
        pub fn read(env: &Env, stream: Tag, err: bool, eof: Tag) -> exception::Result<Tag> {
            Self::read_(env, stream, err, eof)
        }

        /// read a mu s-expression from &str
        pub fn read_str(env: &Env, str: &str) -> exception::Result<Tag> {
            Self::read_str_(env, str)
        }

        /// write a mu s-expression to a core stream
        pub fn write(env: &Env, expr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
            Self::write_(env, expr, escape, stream)
        }

        /// write a mu &str to a core stream
        pub fn write_str(env: &Env, str: &str, stream: Tag) -> exception::Result<()> {
            Self::write_str_(env, str, stream)
        }

        /// write a mu s-expression to a String
        pub fn write_to_string(env: &Env, expr: Tag, esc: bool) -> String {
            Self::write_to_string_(env, expr, esc)
        }

        /// return the standard-input core stream
        pub fn std_in() -> Tag {
            CORE.stdio.0
        }

        /// return the standard-output core stream
        pub fn std_out() -> Tag {
            CORE.stdio.1
        }

        /// return the error-output core stream
        pub fn err_out() -> Tag {
            CORE.stdio.2
        }

        /// format exception
        pub fn exception_string(env: &Env, ex: &Exception) -> String {
            Self::exception_string_(env, ex)
        }

        /// load source file
        pub fn load(env: &Env, file_path: &str) -> exception::Result<bool> {
            if fs::metadata(file_path).is_ok() {
                let load_form = format!("(mu:open :file :input \"{file_path}\" :t)");
                let istream = Self::eval(env, Self::read_str(env, &load_form)?)?;
                let eof_value = Self::eval(env, Self::read_str(env, "(mu:make-symbol \"eof\")")?)?;

                loop {
                    let form = Self::read_(env, istream, false, eof_value)?;

                    if Self::eq(form, eof_value) {
                        break Ok(true);
                    }

                    Self::eval(env, Self::compile(env, form)?)?;
                }
            } else {
                Self::err_(
                    env,
                    Condition::Open,
                    "load",
                    Self::read_str(env, &format!("\"{file_path}\""))?,
                )
            }
        }

        pub fn read_(
            env: &Env,
            stream: Tag,
            eof_error_p: bool,
            eof_value: Tag,
        ) -> exception::Result<Tag> {
            env.read(stream, eof_error_p, eof_value, false)
        }

        pub fn read_str_(env: &Env, str: &str) -> exception::Result<Tag> {
            let stream = StreamBuilder::new()
                .string(str.into())
                .input()
                .build(env, &CORE)?;

            env.read(stream, true, Tag::nil(), false)
        }

        pub fn write_(env: &Env, expr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
            StreamWriter::write(env, expr, escape, stream)
        }

        pub fn write_str_(env: &Env, str: &str, stream: Tag) -> exception::Result<()> {
            StreamWriter::write_str(env, str, stream)
        }

        pub fn write_to_string_(env: &Env, expr: Tag, esc: bool) -> String {
            let stream = StreamBuilder::new()
                .string(String::new())
                .output()
                .build(env, &CORE)
                .unwrap();

            StreamWriter::write(env, expr, esc, stream).unwrap();
            Stream::get_string(env, stream).unwrap()
        }

        pub fn exception_string_(env: &Env, ex: &Exception) -> String {
            format!(
                "error: condition {:?} on {} raised by {}",
                ex.condition,
                Self::write_to_string(env, ex.object, true),
                Self::write_to_string(env, ex.source, true),
            )
        }

        pub fn err_(env: &Env, cond: Condition, source: &str, obj: Tag) -> exception::Result<bool> {
            Err(Exception::err(env, obj, cond, source))
        }
    }
}
