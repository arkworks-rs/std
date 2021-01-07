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
    pub use alloc::fmt::*;
    pub use core::fmt::*;
}

#[cfg(not(feature = "std"))]
pub mod borrow {
    pub use alloc::borrow::*;
    pub use core::borrow::*;
}

#[cfg(not(feature = "std"))]
pub mod slice {
    pub use alloc::slice::*;
    pub use core::slice::*;
}

#[cfg(not(feature = "std"))]
pub mod str {
    pub use alloc::str::*;
    pub use core::str::*;
}

#[cfg(not(feature = "std"))]
pub mod io;

#[cfg(not(feature = "std"))]
pub mod error;

#[cfg(feature = "std")]
#[doc(hidden)]
pub use std::*;

mod rand;
pub use self::rand::*;

#[cfg(feature = "parallel")]
pub use rayon;

/// Returns the base-2 logarithm of `x`.
///
/// ```
/// use ark_std::log2;
///
/// assert_eq!(log2(16), 4);
/// assert_eq!(log2(17), 5);
/// assert_eq!(log2(1), 0);
/// assert_eq!(log2(0), 0);
/// assert_eq!(log2(usize::MAX), (core::mem::size_of::<usize>() * 8) as u32);
/// assert_eq!(log2(1 << 15), 15);
/// assert_eq!(log2(2usize.pow(18)), 18);
/// ```
pub fn log2(x: usize) -> u32 {
    if x == 0 {
        0
    } else if x.is_power_of_two() {
        1usize.leading_zeros() - x.leading_zeros()
    } else {
        0usize.leading_zeros() - x.leading_zeros()
    }
}

/// Creates parallel iterator over refs if `parallel` feature is enabled.
#[cfg(feature = "parallel")]
#[macro_export]
macro_rules! cfg_iter {
    ($e: expr) => {{
        use $crate::rayon::prelude::*;
        $e.par_iter()
    }};
}
#[cfg(not(feature = "parallel"))]
#[macro_export]
macro_rules! cfg_iter {
    ($e: expr) => {{
        $e.iter()
    }};
}

/// Creates parallel iterator over mut refs if `parallel` feature is enabled.
#[cfg(feature = "parallel")]
#[macro_export]
macro_rules! cfg_iter_mut {
    ($e: expr) => {{
        use $crate::rayon::prelude::*;
        $e.par_iter_mut()
    }};
}
#[cfg(not(feature = "parallel"))]
#[macro_export]
macro_rules! cfg_iter_mut {
    ($e: expr) => {{
        $e.iter_mut()
    }};
}

/// Creates parallel iterator if `parallel` feature is enabled.
#[cfg(feature = "parallel")]
#[macro_export]
macro_rules! cfg_into_iter {
    ($e: expr) => {{
        use $crate::rayon::prelude::*;
        $e.into_par_iter()
    }};
}
#[cfg(not(feature = "parallel"))]
#[macro_export]
macro_rules! cfg_into_iter {
    ($e: expr) => {{
        $e.into_iter()
    }};
}

/// Returns an iterator over `chunk_size` elements of the slice at a
/// time.
#[cfg(feature = "parallel")]
#[macro_export]
macro_rules! cfg_chunks {
    ($e:expr, $size:expr) => {{
        use $crate::rayon::prelude::*;
        $e.par_chunks($size)
    }};
}
#[cfg(not(feature = "parallel"))]
#[macro_export]
macro_rules! cfg_chunks {
    ($e:expr, $size:expr) => {{
        $e.chunks($size)
    }};
}

/// Returns an iterator over `chunk_size` mutable elements of the slice at a
/// time.
#[cfg(feature = "parallel")]
#[macro_export]
macro_rules! cfg_chunks_mut {
    ($e: expr, $size: expr) => {{
        use $crate::rayon::prelude::*;
        $e.par_chunks_mut($size)
    }};
}

#[cfg(not(feature = "parallel"))]
#[macro_export]
macro_rules! cfg_chunks_mut {
    ($e: expr, $size: expr) => {{
        $e.chunks_mut($size)
    }};
}
