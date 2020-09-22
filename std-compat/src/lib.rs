#![cfg_attr(not(feature = "std"), no_std)]

/// This crate needs to be public, because we expose the `to_bytes!` macro.
/// See the similar issue in [`smallvec#198`]
///
/// [`smallvec#198`]: https://github.com/servo/rust-smallvec/pull/198
#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub extern crate alloc;

#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub use alloc::*;

#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub use core::*;

#[cfg(feature = "std")]
#[allow(unused_imports)]
#[doc(hidden)]
pub use std::*;
