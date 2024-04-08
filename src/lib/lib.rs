//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//!
//! The lib machine is the implementation surface for the [`mu programming environment`].
//!
//! As much as is practible, mu-core's functions and data types resemble Common Lisp in preference to
//! Scheme/Clojure in order to be immediately familiar to the traditional LISP programmer.
//!
//! lib is an immutable, lexically scoped LISP-1 kernel meant as a porting layer for an ascending
//! tower of LISP languages. While it is possible to do some useful application work directly in the
//! lib language, lib defers niceties like macros, closures, and rest functions to a compiler
//! layered on it. See [`mu programming environment`] for details.
//!
//! lib characteristics:
//! - mostly-safe Rust
//! - 64 bit tagged objects
//! - garbage collected heap
//! - lambda compiler
//! - minimal external dependencies
//! - multiple independent execution contexts
//! - s-expression reader/printer
//! - symbol namespaces
//!
//! lib data types:
//!    56 bit fixnums (immediate)
//!    Lisp-1 symbols
//!    character, string, and byte streams
//!    characters (ASCII immediate)
//!    conses
//!    fixed arity functions
//!    lambdas with lexical variables
//!    general and specialized vectors
//!    keywords (seven character immediate)
//!    single float 32 bit IEEE float (immediate)
//!    structs
//!
//! lib documentation:
//!    see doc/refcards and doc/rustdoc
//!
//! [`mu programming environment`]: <https://github.com/Software-Knife-and-Tool/mu>
//!
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate modular_bitfield;

mod allocators;
mod async_;
mod core;
mod features;
mod streams;
mod types;

use {
    crate::core::{
        compile::Compile,
        config::Config,
        exception::{self, Core as _},
        mu::{self, Core},
    },
    std::fs,
    streams::{read::Core as _, write::Core as _},
    types::{
        stream::{Core as _, Stream},
        streambuilder::StreamBuilder,
    },
};

/// The lib API
///
/// The lib API exposes these types:
/// - Condition, enumeration of possible exceptional conditions
/// - Exception, exception state
/// - Mu, environment and API namespace
/// - Result, specialized result for API functions that can fail
/// - Tag, tagged data representation

/// the tagged data representation
pub type Tag = core::types::Tag;
/// the API function Result
pub type Result = core::exception::Result<Tag>;
/// the condition enumeration
pub type Condition = core::exception::Condition;
/// the Exception representation
pub type Exception = core::exception::Exception;

/// the Mu struct abstracts the mu library struct
pub struct Mu(core::mu::Mu);

impl Mu {
    /// current version
    pub const VERSION: &'static str = core::mu::Mu::VERSION;

    /// init
    pub fn signal_exception() {
        Exception::signal_exception()
    }

    /// config
    pub fn config(config: Option<String>) -> Option<Config> {
        core::mu::Mu::config(config)
    }

    /// constructor
    pub fn new(config: &Config) -> Self {
        Mu(core::mu::Mu::new(config))
    }

    /// apply a function to a list of arguments
    pub fn apply(&self, func: Tag, args: Tag) -> exception::Result<Tag> {
        self.0.apply(func, args)
    }

    /// test tagged s-expressions for strict equality
    pub fn eq(&self, tag: Tag, tag1: Tag) -> bool {
        tag.eq_(&tag1)
    }

    /// evaluate a tagged s-expression
    pub fn eval(&self, expr: Tag) -> exception::Result<Tag> {
        self.0.eval(expr)
    }

    /// compile a tagged s-expression
    pub fn compile(&self, expr: Tag) -> exception::Result<Tag> {
        Compile::compile(&self.0, expr, &mut vec![])
    }

    /// read a tagged s-expression from a mu stream
    pub fn read(&self, stream: Tag, eof_error_p: bool, eof_value: Tag) -> exception::Result<Tag> {
        self.0.read_stream(stream, eof_error_p, eof_value, false)
    }

    /// convert a rust String to a tagged s-expression
    pub fn read_str(&self, str: &str) -> exception::Result<Tag> {
        match StreamBuilder::new()
            .string(str.to_string())
            .input()
            .build(&self.0)
        {
            Ok(stream) => self.0.read_stream(stream, true, Tag::nil(), false),
            Err(e) => Err(e),
        }
    }

    /// write an s-expression to a mu stream
    pub fn write(&self, expr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        self.0.write_stream(expr, escape, stream)
    }

    /// write a rust String to a mu stream
    pub fn write_str(&self, str: &str, stream: Tag) -> exception::Result<()> {
        self.0.write_string(str, stream)
    }

    /// write a tag to a String
    pub fn write_to_string(&self, expr: Tag, esc: bool) -> String {
        let str_stream = match StreamBuilder::new()
            .string("".to_string())
            .output()
            .build(&self.0)
        {
            Ok(stream) => {
                let str_tag = stream;

                self.write(expr, esc, str_tag).unwrap();
                str_tag
            }
            Err(_) => panic!(),
        };

        Stream::get_string(&self.0, str_stream).unwrap()
    }

    /// return the standard-input mu stream
    pub fn std_in(&self) -> Tag {
        self.0.stdin
    }

    /// return the standard-output mu stream
    pub fn std_out(&self) -> Tag {
        self.0.stdout
    }

    /// return the error-output mu stream
    pub fn err_out(&self) -> Tag {
        self.0.errout
    }

    // eval &str
    pub fn eval_str(&self, expr: &str) -> exception::Result<Tag> {
        match self.read_str(expr) {
            Ok(expr) => match self.compile(expr) {
                Ok(expr) => self.0.eval(expr),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
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

    pub fn load(&self, file_path: &str) -> exception::Result<bool> {
        if fs::metadata(file_path).is_ok() {
            let load_form = format!("(lib:open :file :input \"{}\")", file_path);
            let istream = self.0.eval(self.read_str(&load_form).unwrap()).unwrap();
            let eof_value = self.read_str(":eof").unwrap(); // need make_symbol here

            #[allow(clippy::while_let_loop)]
            loop {
                match self.read(istream, false, eof_value) {
                    Ok(form) => {
                        if self.eq(form, eof_value) {
                            return Ok(true);
                        }
                        match self.compile(form) {
                            Ok(form) => match self.eval(form) {
                                Ok(_) => (),
                                Err(e) => return Err(e),
                            },
                            Err(e) => return Err(e),
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        } else {
            Err(Exception::new(
                Condition::Open,
                "sys:lf",
                self.read_str(&format!("\"{}\"", file_path)).unwrap(),
            ))
        }
    }

    // image management
    pub fn load_image(&self, _file_path: &str) -> exception::Result<()> {
        Ok(())
    }

    pub fn save_and_exit(&self, _file_path: &str) -> exception::Result<()> {
        Ok(())
    }
}
