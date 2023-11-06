use std::io;

use ed25519::pkcs8::{self, spki};
use rsa::pkcs8::der;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Pkcs8(#[from] pkcs8::Error),
    #[error(transparent)]
    Spki(#[from] spki::Error),
    #[error(transparent)]
    Der(#[from] der::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
