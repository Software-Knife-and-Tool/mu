//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features module
#[cfg(feature = "cpu_time")]
pub mod cpu_time;
#[cfg(feature = "env")]
pub mod env;
pub mod feature;
#[cfg(feature = "ffi")]
pub mod ffi;
#[cfg(feature = "nix")]
pub mod nix;
#[cfg(feature = "prof")]
pub mod prof;
#[cfg(feature = "semispace")]
pub mod semispace;
#[cfg(feature = "std")]
pub mod std;
#[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
pub mod sysinfo;
