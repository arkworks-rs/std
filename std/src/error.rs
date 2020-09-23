use crate::boxed::Box;

#[cfg(feature = "std")]
pub use std::error::Error;

#[cfg(not(feature = "std"))]
pub trait Error: core::fmt::Debug + core::fmt::Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[cfg(not(feature = "std"))]
impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a> {
    fn from(err: E) -> Self {
        Box::new(err)
    }
}

#[cfg(not(feature = "std"))]
impl<T: Error> Error for Box<T> {}

#[cfg(not(feature = "std"))]
impl Error for crate::string::String {}

#[cfg(not(feature = "std"))]
impl Error for crate::io::Error {}
