//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(unused_results)]

//! # Mu
//!
//! The *mu* library is the implementation surface for the [`mu programming environment`](<https://github.com/Software-Knife-and-Tool/mu>) and
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
//!    - quasiquote reader syntax
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

    /// the Mu namespace
    pub struct Mu;

            /// Create an Env configuration from a JSON string.
        ///
        /// returns an initialized Config struct, see CONFIG.md for
        /// details. a Config is needed to create an Env to operate
        /// the interesting parts of the API.
        ///
        /// # Panics
        ///
        /// will panic for unrecognized config keywords.
        /// (think about an mforge config string checker)
        ///
        /// # Examples
        ///
        /// ```
        /// // create an Env configuration. the JSON string argument
        /// // is an Option<String>, None returns the library default
        /// // configuration.
        ///
        /// use mu::{Mu, Config};
        ///
        /// // capture default Config
        ///
        /// let config: Config = Mu::config(None);
        /// ```
        pub fn config(config: Option<String>) -> Config {
            Config::new(config)
        }

    /// About the Mu API
    ///
    /// Using the API:
    ///
    ///   1. create an Env configuration.
    ///
    ///      - from a JSON string
    ///      - the default
    ///
    ///      see CONFIG.md for details. a Config is needed to create
    ///      an Env to operate the interesting parts of the API.
    ///
    ///    2. create an Env from that Config for use in subsequent API
    ///       calls.
    ///
    ///    3. Many API functions return a specialized Result. The Err return
    ///       supplies an Exception that can be printed with the supplied
    ///       convenience function.
    ///
    ///    4. The API supports the reading, compilation of special forms,
    ///       evaluation, and printing of mu forms from/to Rust strings And
    ///       core streams.
    ///
    /// #Example
    ///
    /// ```
    /// use mu::{Mu, Env, Exception, Condition, Config};
    ///
    /// // capture default Config
    /// let config: Config = Mu::config(None);
    ///
    /// // capture default Env
    /// let env: Env = Mu::env(config);
    /// ```
    impl Mu {
        /// Returns the library version as a &str.
        ///
        /// # Example
        ///
        /// ```
        /// // capture library semver string
        /// let version: &str = Mu::version();
        /// ```
        pub fn version() -> &'static str {
            env!("CARGO_PKG_VERSION")
        }

        /// Returns the nil tagged mu form.
        ///
        /// # Example
        ///
        /// ```
        /// // capture ()
        /// let nil: Tag = Mu::nil();
        /// ```
        pub fn nil() -> Tag {
            Tag::nil()
        }
        
        /// Create an Env configuration from a JSON string.
        ///
        /// returns an initialized Config struct, see CONFIG.md for
        /// details. a Config is needed to create an Env to operate
        /// the interesting parts of the API.
        ///
        /// # Panics
        ///
        /// will panic for unrecognized config keywords.
        /// (think about an mforge config string checker)
        ///
        /// # Example
        ///
        /// ```
        /// // create an Env configuration. the JSON string argument
        /// // is an Option<String>, None returns the library default
        /// // configuration.
        ///
        /// // capture default Config
        /// let config: Config = Mu::config(None);
        /// ```
        pub fn config(config: Option<String>) -> Config {
            Config::new(config)
        }

        /// Create an Env from a Config.
        ///
        /// returns an Env struct.
        ///
        /// # Panics
        ///
        /// will panic for a variety of reasons, mostly heap-related
        /// allocation and initialization problems.
        ///
        /// # Example
        ///
        /// create an Env with the library default configuration.
        ///
        /// ```
        /// // capture default Env
        /// let env: Env = Mu::env(Mu::config(None));
        /// ```
        pub fn env(config: &Config) -> Env {
            Env::new(config)
        }

        /// Compile a tagged mu form to a tagged form Result.
        ///
        /// returns a tagged mu form Result.
        ///
        /// # Panics
        ///
        /// will panic for internal consistency problems or a heap exhausted condition.
        ///
        /// # Example
        ///
        /// Read a lambda definition and compile it to a function..
        ///
        /// ```
        /// // capture identity anonymous function
        /// let identity: Tag = Mu::compile(env, Mu::read_str(env, "(:lambda (a) a)").umwrap()).unwrap();
        /// ```
        pub fn compile(env: &Env, expr: Tag) -> exception::Result<Tag> {
            Compiler::compile(env, expr, &mut vec![])
        }

        /// Test two compiled mu forms for identity.
        ///
        /// returns a bool.
        ///
        /// # Example
        ///
        /// ```
        /// // affirm () eq itself
        /// let nil: Tag = Mu::nil(env);
        /// let is_eq: bool = Mu::eq(nil, nil);
        ///
        /// assert(is_eq);
        /// ```
        pub fn eq(tag: Tag, tag1: Tag) -> bool {
            tag.eq_(&tag1)
        }

        /// Compile and evaluate a mu form.
        ///
        /// returns a tagged mu form Result.
        ///
        /// Note: only special forms and forms containing special forms
        /// require prior compilation. Compiling function calls and
        /// constant forms simply returns the form at a slight mu
        /// function call overhead.
        ///
        /// # Errors
        ///
        /// - compile exception
        /// - eval exception
        ///
        /// # Panics
        ///
        /// will panic for a variety of reasons, mostly heap-related
        /// allocation and internal consistency problems.
        ///
        /// # Example
        ///
        /// ```
        /// // capture mu fixnum form 2 by calling mu add function
        /// let two: Tag = Mu::eval(Mu::read_str(env, "(mu:add 1 1)").unwrap()).unwrap();
        /// ```
        pub fn eval(env: &Env, expr: Tag) -> exception::Result<Tag> {
            Apply::eval(env, Self::compile(env, expr)?)
        }
        
        /// Read a mu form from a str, compile and evaluate it.
        ///
        /// returns a tagged mu form Result.
        ///
        /// # Errors
        ///
        /// - reader exception
        /// - compiler exception
        /// - eval exception
        ///
        /// # Panics
        ///
        /// will panic for a variety of reasons, mostly heap-related
        /// allocation and internal consistency problems.
        ///
        /// # Example
        ///
        /// ```
        /// // compute 2 and capture fixnum tag
        /// let two = Mu::eval_str(env, "(mu:add 1 1)").unwrap();
        /// ```
        pub fn eval_str(env: &Env, expr: &str) -> exception::Result<Tag> {
            Self::eval(env, Self::read_str(env, expr)?)
        }

        /// Read a mu form from a core stream.
        ///
        /// returns a tagged mu form Result. Optionally assert exception on stream EOF.
        ///
        /// # Errors
        ///
        /// - stream exception
        /// - reader exception
        ///
        /// # Panics
        ///
        /// will panic for a variety of reasons, mostly heap-related
        /// allocation and initialization problems.
        ///
        /// # Example
        ///
        /// ```
        /// // read a form from stdin and capture it
        /// let form = Mu::read(env, Mu::std_in(env), true, Mu::nil()).unwrap();
        /// ```
        pub fn read(env: &Env, stream: Tag, err: bool, eof: Tag) -> exception::Result<Tag> {
            env.read(stream, err, eof, false)
        }
            
        /// Read a mu tagged form from a &str.
        ///
        /// returns a mu tagged form Result. leaks a string stream.
        /// (persistant string streamn vaeiant?)
        ///
        /// # Panics
        ///
        /// will panic for a variety of reasons, mostly heap-related
        /// allocation and core consistency problems.
        ///
        /// # Example
        ///
        /// ```
        /// // capture the mu car symbol
        /// let car: Tag = Mu::read_str(env, "mu:car").unwrap();
        /// ```
        pub fn read_str(env: &Env, str: &str) -> exception::Result<Tag> {
            let stream = StreamBuilder::new()
                .string(str.into())
                .input()
                .build(env, &CORE)?;

            env.read(stream, true, Tag::nil(), false)
        }

        /// Write a mu tagged form to a core stream..
        ///
        /// returns the argument in a Result.
        ///
        /// # Panic
        ///
        /// - stream exception
        /// - writer exception
        ///
        /// # Example
        ///
        /// ```
        /// // write :nil to stdout, capture ()
        /// let nil: Tag = Mu::write(env, Mu::nil(), false, Mu::std_out()).unwrap();
        /// ```
        pub fn write(env: &Env, expr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
            StreamWriter::write(env, expr, escape, stream)
        }

        /// Write a mu tagged form to a core stream..
        ///
        /// returns () in a Result.
        ///
        /// # Panics
        ///
        /// - stream exception
        /// - writer exception
        ///
        /// # Example
        ///
        /// ```
        /// // write "foo" to stdout
        /// Mu::write_str(env, "foo", Mu::std_out()).unwrap();
        /// ```
        pub fn write_str(env: &Env, str: &str, stream: Tag) -> exception::Result<()> {
            StreamWriter::write_str(env, str, stream)
        }

        /// Write a mu tagged form to a String.
        ///
        /// returns a String. leaks a string stream.
        ///
        /// # Panics
        ///
        /// will panic for a variety of reasons, mostly heap-related
        /// allocation and initialization problems.
        ///
        /// # Example
        ///
        /// ```
        /// // write a mu tagged string to a String with quotes.
        /// let quoted_string = Mu::write_to_string(env, Mu::read_str(env, "\"astring\"").unwrap(), true);
        /// ```
        pub fn write_to_string(env: &Env, expr: Tag, esc: bool) -> String {
            let stream = StreamBuilder::new()
                .string(String::new())
                .output()
                .build(env, &CORE)
                .unwrap();

            StreamWriter::write(env, expr, esc, stream).unwrap();
            Stream::get_string(env, stream).unwrap()
        }
        
        /// Return Env's standard-input core stream binding.
        ///
        /// returns a mu tagged form.
        ///
        /// ```
        /// let std_in = Mu::std_in(env);
        /// ```
        pub fn std_in() -> Tag {
            CORE.stdio.0
        }

        /// Return Env's standard-output core stream binding.
        ///
        /// returns a mu tagged form.
        ///
        /// ```
        /// let std_out = Mu::std_out(env);
        /// ```
        pub fn std_out() -> Tag {
            CORE.stdio.1
        }

        /// Return Env's error-out core stream binding.
        ///
        /// returns a mu tagged form.
        ///
        /// ```
        /// let err_out = Mu::err_out(env);
        /// ```
        pub fn err_out() -> Tag {
            CORE.stdio.2
        }

        /// Create a string from an Exception.
        ///
        /// returns a String
        ///
        /// # Example
        ///
        /// ```
        /// eprintln!("{}", Mu::exception_string(env: &Envl, ex: &Exception));
        /// ```
        pub fn exception_string(env: &Env, ex: &Exception) -> String {
            format!(
                "error: condition {:?} on {} raised by {}",
                ex.condition,
                Self::write_to_string(env, ex.object, true),
                Self::write_to_string(env, ex.source, true),
            )
        }

        /// Load a file by filename.
        ///
        /// returns a speecialized Result of type bool.
        ///
        /// # Panic
        ///
        /// will panic for a variety of reasons, mostly heap-related
        /// allocation and initialization problems.
        ///
        /// # Example
        ///
        /// ```
        /// // load foo.l, return true for success
        /// Mu::load(env, "foo.l").unwrap();
        /// ```
        pub fn load(env: &Env, file_path: &str) -> exception::Result<bool> {
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
                Err(Exception::err(
                    env,
                    Self::read_str(env, &format!("\"{file_path}\""))?,
                    Condition::Open,
                    "load",
                ))?
            }
        }
    }
}
