//! Error types wrapper

use std::io;

use thiserror::Error;

/// Error type
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Error propagated from [`io::Error`]
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Error propagated from [`pgp::errors::Error`]
    #[error("PGP error: {0}")]
    Pgp(#[from] pgp::errors::Error),
}

/// Result type for use in the crate.
pub type Result<T> = std::result::Result<T, Error>;
