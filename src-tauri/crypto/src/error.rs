use std::io;

use ed25519::pkcs8::{
    self,
    spki::{self, der},
};

use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Pkcs8(#[from] pkcs8::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<spki::Error> for Error {
    fn from(error: spki::Error) -> Self {
        Self::Pkcs8(pkcs8::Error::PublicKey(error))
    }
}

impl From<der::Error> for Error {
    fn from(error: der::Error) -> Self {
        Self::Pkcs8(pkcs8::Error::Asn1(error))
    }
}
