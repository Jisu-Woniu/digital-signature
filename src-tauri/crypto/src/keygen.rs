//! Utilities for key generation.

use std::path::{Path, PathBuf};

use futures::future::try_join;
use pgp::{types::KeyTrait, ArmorOptions};
use serde::Serialize;
use tokio::fs::{write, DirBuilder};
use zeroize::Zeroizing;

use crate::{key_pair::KeyPair, secret_file::write_secret_file, Result};

/// Represents paths of key pair.
///
/// Rename to camelCase for frontend usage.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPairPaths {
    /// Path of secret key.
    pub secret_key_path: PathBuf,
    /// Path of public key.
    pub public_key_path: PathBuf,
}

/// Write a key pair generated from `name` and `email` to a given path, in armored form as RFC 4880.
///
/// # Errors
///
/// This function will return an error if:
/// - The destination directory cannot be created.
/// - Cannot generate key pair properly.
/// - Failed to write generated key pair.
pub async fn write_key_pair<F>(
    name: &str,
    email: &str,
    passwd_fn: F,
    path: impl AsRef<Path>,
) -> Result<KeyPairPaths>
where
    F: FnOnce() -> String + Clone,
{
    let path = path.as_ref();

    // Create output directory if not exist.
    DirBuilder::new().recursive(true).create(path).await?;

    let key_pair = KeyPair::generate(name, email, passwd_fn)?;
    let signed_secret_key = key_pair.secret_key();
    let signed_public_key = key_pair.public_key();
    let key_id = &hex::encode_upper(&signed_secret_key.key_id().as_ref()[4..]);

    let secret_key_path = path.join(format!("{}_0x{}_SECRET.asc", name, key_id));
    let public_key_path = path.join(format!("{}_0x{}_public.asc", name, key_id));

    let secret_key_armored =
        Zeroizing::new(signed_secret_key.to_armored_bytes(ArmorOptions::default())?);
    let public_key_armored = signed_public_key.to_armored_bytes(ArmorOptions::default())?;

    try_join(
        write_secret_file(&secret_key_path, &secret_key_armored),
        write(&public_key_path, &public_key_armored),
    )
    .await?;

    Ok(KeyPairPaths {
        secret_key_path,
        public_key_path,
    })
}

#[cfg(test)]
mod tests {

    use pgp::{types::KeyTrait, ArmorOptions, SignedSecretKey};

    use crate::{from_file::FromFile, key_pair::KeyPair, Result};

    #[test]
    fn test() -> Result<()> {
        let key_pair = KeyPair::generate("example", "example@example.com", String::new)?;
        let (secret_key, public_key) = (key_pair.secret_key(), key_pair.public_key());
        println!("{}", secret_key.to_armored_string(ArmorOptions::default())?);
        println!("{}", public_key.to_armored_string(ArmorOptions::default())?);
        dbg!(public_key);
        Ok(())
    }

    #[test]
    fn extract_key_info() -> Result<()> {
        let secret_key_str = KeyPair::generate("example", "example@example.com", String::new)?
            .secret_key()
            .to_armored_bytes(ArmorOptions::default())?;

        let secret_key = SignedSecretKey::try_from_armored_bytes(secret_key_str)?;
        dbg!(&secret_key);
        let key_id = secret_key.key_id();
        dbg!(&key_id);
        dbg!(&hex::encode_upper(&key_id.as_ref()[4..]));
        Ok(())
    }
}
