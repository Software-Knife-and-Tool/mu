//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! core module
#![allow(clippy::module_inception)]
pub mod apply;
pub mod compile;
pub mod config;
pub mod core;
pub mod direct;
pub mod dynamic;
pub mod env;
pub mod exception;
pub mod frame;
pub mod gc;
pub mod heap;
pub mod indirect;
pub mod mu;
pub mod namespace;
pub mod quasi;
pub mod reader;
pub mod readtable;
pub mod type_image;
pub mod types;
pub mod writer;
