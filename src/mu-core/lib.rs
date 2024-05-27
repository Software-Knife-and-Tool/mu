//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//!
//! The core machine is the implementation surface for the [`mu programming environment`].
//!
//! As much as is practible, core's functions and data types resemble Common Lisp in preference to
//! Scheme/Clojure in order to be immediately familiar to the traditional LISP programmer.
//!
//! core is an immutable, lexically scoped LISP-1 kernel meant as a porting layer for an ascending
//! tower of LISP languages. While it is possible to do some useful application work directly in the
//! core language, core defers niceties like macros, closures, and rest functions to a compiler
//! layered on it. See [`mu programming environment`] for details.
//!
//! core characteristics:
//! - mostly-safe Rust
//! - 64 bit tagged objects
//! - garbage collected heap
//! - lambda compiler
//! - minimal external dependencies
//! - multiple independent execution contexts
//! - s-expression reader/printer
//! - symbol namespaces
//!
//! core data types:
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
//! core documentation:
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
mod core;
mod features;
mod streams;
mod system;
mod types;

use futures::executor::block_on;
use {
    crate::{
        core::{
            compile::Compile,
            env::Core as _,
            exception::{self, Core as _},
            lib::LIB,
        },
        streams::{read::Core as _, write::Core as _},
        system::{
            config::Config,
            image::{Core as _, Image},
        },
        types::{
            core_stream::{Core as _, Stream},
            stream::StreamBuilder,
        },
    },
    std::{fs, io::Write},
};

/// The core API
///
/// The core API exposes these types:
/// - Condition, enumeration of possible exceptional conditions
/// - Exception, exception state
/// - Lib, environment and API namespace
/// - Result, specialized result for API functions that can fail
/// - Tag, tagged data representation

/// tagged data representation
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
    pub const VERSION: &'static str = core::lib::Lib::VERSION;

    /// init
    pub fn signal_exception() {
        Exception::signal_exception()
    }

    /// config
    pub fn config(config: Option<String>) -> Option<Config> {
        Config::new(config)
    }

    /// constructor
    pub fn new(config: Config) -> Self {
        Env(<core::env::Env as core::lib::Core>::add_env(
            core::env::Env::new(config),
        ))
    }

    /// apply a function to a list of arguments
    pub fn apply(&self, func: Tag, args: Tag) -> exception::Result<Tag> {
        let env_ref = block_on(LIB.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.apply(func, args)
    }

    /// test tagged s-expressions for strict equality
    pub fn eq(&self, tag: Tag, tag1: Tag) -> bool {
        tag.eq_(&tag1)
    }

    /// evaluate a tagged s-expression
    pub fn eval(&self, expr: Tag) -> exception::Result<Tag> {
        let env_ref = block_on(LIB.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.eval(expr)
    }

    /// compile a tagged s-expression
    pub fn compile(&self, expr: Tag) -> exception::Result<Tag> {
        let env_ref = block_on(LIB.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        Compile::compile(env, expr, &mut vec![])
    }

    /// read a tagged s-expression from a core stream
    pub fn read(&self, stream: Tag, eof_error_p: bool, eof_value: Tag) -> exception::Result<Tag> {
        let env_ref = block_on(LIB.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.read_stream(stream, eof_error_p, eof_value, false)
    }

    /// convert a String to a tagged s-expression
    pub fn read_str(&self, str: &str) -> exception::Result<Tag> {
        let env_ref = block_on(LIB.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        let stream = StreamBuilder::new()
            .string(str.to_string())
            .input()
            .build(env, &LIB)?;

        env.read_stream(stream, true, Tag::nil(), false)
    }

    /// write an s-expression to a core stream
    pub fn write(&self, expr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let env_ref = block_on(LIB.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.write_stream(expr, escape, stream)
    }

    /// write a rust String to a core stream
    pub fn write_str(&self, str: &str, stream: Tag) -> exception::Result<()> {
        let env_ref = block_on(LIB.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        env.write_string(str, stream)
    }

    /// write a tag to a String
    pub fn write_to_string(&self, expr: Tag, esc: bool) -> String {
        let env_ref = block_on(LIB.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        let str_stream = match StreamBuilder::new()
            .string("".to_string())
            .output()
            .build(env, &LIB)
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
        LIB.stdin()
    }

    /// return the standard-output core stream
    pub fn std_out(&self) -> Tag {
        LIB.stdout()
    }

    /// return the error-output core stream
    pub fn err_out(&self) -> Tag {
        LIB.errout()
    }

    // eval &str
    pub fn eval_str(&self, expr: &str) -> exception::Result<Tag> {
        let env_ref = block_on(LIB.env_map.read());
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

    pub fn load(&self, file_path: &str) -> exception::Result<bool> {
        let env_ref = block_on(LIB.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        if fs::metadata(file_path).is_ok() {
            let load_form = format!("(core:open :file :input \"{}\")", file_path);
            let istream = env.eval(self.read_str(&load_form).unwrap()).unwrap();
            let eof_value = self.read_str(":eof").unwrap(); // need make_symbol here

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
                "core:open",
                self.read_str(&format!("\"{}\"", file_path)).unwrap(),
            ))
        }
    }

    // image management
    pub fn save_image(&self, path: &str) -> exception::Result<()> {
        let env_ref = block_on(LIB.env_map.read());
        let env = env_ref.get(&self.0.as_u64()).unwrap();

        match fs::File::create(path) {
            Ok(mut file) => match file.write(&Image::image(env)) {
                Ok(_) => Ok(()),
                Err(_) => Err(Exception::new(
                    env,
                    Condition::Write,
                    "save-image",
                    Tag::nil(),
                )),
            },
            Err(_) => Err(Exception::new(
                env,
                Condition::Open,
                "save-image",
                Tag::nil(),
            )),
        }
    }
}
