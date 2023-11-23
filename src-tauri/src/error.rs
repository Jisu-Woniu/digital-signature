use std::{
    fmt::{self, Debug, Display, Formatter},
    io,
};

use serde::Serialize;

pub struct Error(anyhow::Error);

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl From<digital_signature_crypto::Error> for Error {
    fn from(value: digital_signature_crypto::Error) -> Self {
        Self(anyhow::Error::from(value))
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self(anyhow::Error::from(value))
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
