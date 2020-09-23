#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[doc(hidden)]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub use alloc::*;

#[cfg(not(feature = "std"))]
pub use core::*;

#[cfg(not(feature = "std"))]
pub mod fmt {
    pub use core::fmt::*;
    pub use alloc::fmt::*;

}

#[cfg(not(feature = "std"))]
pub mod borrow {
    pub use core::borrow::*;
    pub use alloc::borrow::*;
}

#[cfg(not(feature = "std"))]
pub mod slice {
    pub use core::slice::*;
    pub use alloc::slice::*;
}

#[cfg(not(feature = "std"))]
pub mod sync {
    pub use core::sync::*;
    pub use alloc::sync::*;
}

#[cfg(not(feature = "std"))]
pub mod str {
    pub use core::str::*;
    pub use alloc::str::*;
}

#[cfg(not(feature = "std"))]
pub mod io;

#[cfg(not(feature = "std"))]
pub mod error;

#[cfg(feature = "std")]
#[doc(hidden)]
pub use std::*;




/// Creates parallel iterator over refs if `parallel` feature is enabled.
#[macro_export]
macro_rules! cfg_iter {
    ($e: expr) => {{
        #[cfg(feature = "parallel")]
        let result = $e.par_iter();

        #[cfg(not(feature = "parallel"))]
        let result = $e.iter();

        result
    }};
}

/// Creates parallel iterator over mut refs if `parallel` feature is enabled.
#[macro_export]
macro_rules! cfg_iter_mut {
    ($e: expr) => {{
        #[cfg(feature = "parallel")]
        let result = $e.par_iter_mut();

        #[cfg(not(feature = "parallel"))]
        let result = $e.iter_mut();

        result
    }};
}

/// Creates parallel iterator if `parallel` feature is enabled.
#[macro_export]
macro_rules! cfg_into_iter {
    ($e: expr) => {{
        #[cfg(feature = "parallel")]
        let result = $e.into_par_iter();

        #[cfg(not(feature = "parallel"))]
        let result = $e.into_iter();

        result
    }};
}

/// Returns an iterator over `chunk_size` elements of the slice at a
/// time.
#[macro_export]
macro_rules! cfg_chunks {
    ($e: expr, $size: expr) => {{
        #[cfg(feature = "parallel")]
        let result = $e.par_chunks($size);

        #[cfg(not(feature = "parallel"))]
        let result = $e.chunks($size);

        result
    }};
}

/// Returns an iterator over `chunk_size` mutable elements of the slice at a
/// time.
#[macro_export]
macro_rules! cfg_chunks_mut {
    ($e: expr, $size: expr) => {{
        #[cfg(feature = "parallel")]
        let result = $e.par_chunks_mut($size);

        #[cfg(not(feature = "parallel"))]
        let result = $e.chunks_mut($size);

        result
    }};
}
