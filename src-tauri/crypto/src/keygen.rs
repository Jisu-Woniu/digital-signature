//! Utilities for key generation.

use std::path::{Path, PathBuf};

use pgp::types::KeyTrait;
use serde::Serialize;
use tokio::{
    fs::{self, DirBuilder},
    try_join,
};
use zeroize::Zeroizing;

use crate::{key_pair::KeyPair, secret_file::write_secret_file, Result};

/// Represents paths of key pair.
#[derive(Debug, Serialize)]
pub struct KeyPairPath {
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
) -> Result<KeyPairPath>
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

    let secret_key_armored = Zeroizing::new(signed_secret_key.to_armored_bytes(None)?);
    let public_key_armored = signed_public_key.to_armored_bytes(None)?;

    try_join!(
        write_secret_file(&secret_key_path, &secret_key_armored),
        fs::write(&public_key_path, &public_key_armored)
    )?;

    Ok(KeyPairPath {
        secret_key_path,
        public_key_path,
    })
}

#[cfg(test)]
mod tests {
    use pgp::{types::KeyTrait, Deserializable, SignedSecretKey};

    use crate::{key_pair::KeyPair, Result};

    #[tokio::test]
    #[ignore = "Manual testing for file generation."]
    async fn test() -> Result<()> {
        let key_pair = KeyPair::generate("example", "example@example.com", String::new)?;
        let (secret_key, public_key) = (key_pair.secret_key(), key_pair.public_key());
        println!("{}", secret_key.to_armored_string(None)?);
        println!("{}", public_key.to_armored_string(None)?);
        dbg!(public_key);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "Manual testing for file parsing."]
    async fn extract_key_info() -> Result<()> {
        let secret_key_str = KeyPair::generate("example", "example@example.com", String::new)?
            .secret_key()
            .to_armored_string(None)?;

        let secret_key = SignedSecretKey::from_string(&secret_key_str)?.0;
        dbg!(&secret_key);
        let key_id = secret_key.key_id();
        dbg!(&key_id);
        dbg!(&hex::encode_upper(&key_id.as_ref()[4..]));
        Ok(())
    }
}
