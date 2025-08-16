//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//!
//! The mu library is the implementation surface for the [`mu programming environment`] and
//! implements the *mu*, null, and *features* namespaces.
//!
//! mu is an immutable, lexically scoped Lisp-1 kernel porting layer for an ascending tower of
//! Lisp languages. While it is possible to do some useful application work directly in the *mu*
//! language, *mu* defers niceties like macros, closures, and rest lambdas to libraries and
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
mod streams;
mod types;
mod vectors;

use {
    crate::core::{config::Config, core::CORE, exception},
    std::fs,
};

/// The library API
///
/// The library API exposes these types:
/// - Condition, enumeration of exceptional conditions
/// - Config, Env configuration
/// - Core, CORE runtime state
/// - Env, execution environment
/// - Exception, exception state
/// - Mu, environment and API namespace
/// - Result, specialized result for API functions that can fail
/// - Tag, tagged data representation
///   tagged data representation
pub type Tag = core::types::Tag;
/// Mu library API
pub type Mu = core::mu::Mu;
/// Core library state representation
pub type Core = core::core::Core;
/// environment
pub type Env = core::mu::Env;
/// exception Condition enumeration
pub type Condition = core::exception::Condition;
/// Exception representation
pub type Exception = core::exception::Exception;
/// Mu function Result
pub type Result = core::exception::Result<Tag>;

/// API namespace
impl Mu {
    /// core image
    pub fn core() -> &'static Core {
        &CORE
    }

    /// version
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// environment configuration
    pub fn config(config: Option<String>) -> Option<Config> {
        Config::new(config)
    }

    /// env constructor
    pub fn make_env(config: &Config) -> Env {
        Self::make_env_(config)
    }

    /// apply a function to a list of arguments
    pub fn apply(env: Env, func: Tag, args: Tag) -> exception::Result<Tag> {
        Self::apply_(env, func, args)
    }

    /// compile an s-expression
    pub fn compile(env: Env, expr: Tag) -> exception::Result<Tag> {
        Self::compile_(env, expr)
    }

    /// test tagged s-expressions for strict equality
    pub fn eq(tag: Tag, tag1: Tag) -> bool {
        tag.eq_(&tag1)
    }

    /// evaluate an s-expression
    pub fn eval(env: Env, expr: Tag) -> exception::Result<Tag> {
        Self::eval_(env, expr)
    }

    /// eval &str
    pub fn eval_str(env: Env, expr: &str) -> exception::Result<Tag> {
        Self::eval(env, Self::compile(env, Self::read_str(env, expr)?)?)
    }

    /// read a mu s-expression from a core stream
    pub fn read(
        env: Env,
        stream: Tag,
        eof_error_p: bool,
        eof_value: Tag,
    ) -> exception::Result<Tag> {
        Self::read_(env, stream, eof_error_p, eof_value)
    }

    /// read a mu s-expression from &str
    pub fn read_str(env: Env, str: &str) -> exception::Result<Tag> {
        Self::read_str_(env, str)
    }

    /// write a mu s-expression to a core stream
    pub fn write(env: Env, expr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        Self::write_(env, expr, escape, stream)
    }

    /// write a mu &str to a core stream
    pub fn write_str(env: Env, str: &str, stream: Tag) -> exception::Result<()> {
        Self::write_str_(env, str, stream)
    }

    /// write a mu s-expression to a String
    pub fn write_to_string(env: Env, expr: Tag, esc: bool) -> String {
        Self::write_to_string_(env, expr, esc)
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

    /// format exception
    pub fn exception_string(env: Env, ex: Exception) -> String {
        Self::exception_string_(env, ex)
    }

    /// load source file
    pub fn load(env: Env, file_path: &str) -> exception::Result<bool> {
        if fs::metadata(file_path).is_ok() {
            let load_form = format!("(mu:open :file :input \"{file_path}\" :t)");
            let istream = Self::eval(env, Self::read_str(env, &load_form)?)?;
            let eof_value = Self::eval(env, Self::read_str(env, "(mu:make-symbol \"eof\")")?)?;

            loop {
                let form = Self::read(env, istream, false, eof_value)?;

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
}
