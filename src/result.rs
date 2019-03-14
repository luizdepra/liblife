//! Result and errors.

use std::result;

use failure::Fail;

/// Internal results representation.
pub type Result<T> = result::Result<T, Error>;

/// Internal error representation.
#[derive(Fail, Debug)]
pub enum Error {
    /// Error thrown when a Generation is initialized with one or more invalid dimension.
    #[fail(display = "Generation's width and height must be equal or greater than 3.")]
    InvalidDimensionError,
}
