//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features module
pub mod feature;
#[cfg(feature = "ffi")]
pub mod ffi;
#[cfg(feature = "nix")]
pub mod nix;
#[cfg(feature = "std")]
pub mod std;
#[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
pub mod sysinfo;
