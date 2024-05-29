//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! features module
pub mod feature;

#[cfg(feature = "nix")]
use crate::features::nix::nix_::Nix;
#[cfg(feature = "std")]
use crate::features::std::std_::Std;
#[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
use crate::features::sysinfo::sysinfo_::Sysinfo;

#[cfg(feature = "nix")]
pub mod nix;
#[cfg(feature = "std")]
pub mod std;
#[cfg(all(feature = "sysinfo", not(target_os = "macos")))]
pub mod sysinfo;
