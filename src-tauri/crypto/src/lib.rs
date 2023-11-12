#![warn(missing_docs)]
//! Cryptographic functions for the digital signature app.

pub mod error;
mod key_pair;
pub mod keygen;
mod secret_file;
pub mod signing;

pub use error::Result;
