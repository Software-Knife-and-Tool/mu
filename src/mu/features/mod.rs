//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features crate
#[cfg(feature = "core")]
pub mod core;
#[cfg(feature = "env")]
pub mod env;
pub mod feature;
#[cfg(feature = "instrument")]
pub mod instrument;
#[cfg(feature = "system")]
pub mod system;
