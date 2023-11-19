use std::{
    fmt::{self, Display, Formatter},
    io,
};

use serde::Serialize;

#[derive(Debug)]
pub struct Error(anyhow::Error);

pub type Result<T> = std::result::Result<T, Error>;

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl From<digital_signature_crypto::error::Error> for Error {
    fn from(value: digital_signature_crypto::error::Error) -> Self {
        Self(anyhow::Error::from(value))
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self(anyhow::Error::from(value))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
