use std::io;

use ed25519::pkcs8::{self, spki};

pub enum Error {
    Io(io::Error),
    Pkcs8(pkcs8::Error),
    Spki(spki::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<pkcs8::Error> for Error {
    fn from(value: pkcs8::Error) -> Self {
        Self::Pkcs8(value)
    }
}

impl From<spki::Error> for Error {
    fn from(value: spki::Error) -> Self {
        Self::Spki(value)
    }
}
