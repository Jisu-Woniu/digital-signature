use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("PGP error: {0}")]
    Pgp(#[from] pgp::errors::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
