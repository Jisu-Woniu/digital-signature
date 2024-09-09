//! Utilities for signing files.

use std::{
    io::{self, Read},
    path::{Path, PathBuf},
};

use chrono::Utc;
use pgp::{
    packet::{self, SignatureConfigBuilder, SignatureType, Subpacket, SubpacketData},
    types::SecretKeyTrait,
    Signature, SignedPublicKey, SignedSecretKey,
};
use tokio::fs::File;

use crate::{from_file::FromFile, Result};

/// Generate a signature of the given data.
fn sign(
    data: impl Read,
    secret_key: &impl SecretKeyTrait,
    passwd_fn: impl FnOnce() -> String + Clone,
) -> Result<Signature> {
    let now = Utc::now();
    let sig_conf = SignatureConfigBuilder::default()
        .pub_alg(secret_key.algorithm())
        .hash_alg(secret_key.hash_alg())
        .typ(SignatureType::Binary)
        .issuer(Some(secret_key.key_id()))
        .created(Some(now))
        .hashed_subpackets(vec![
            Subpacket::regular(SubpacketData::SignatureCreationTime(now)),
            Subpacket::regular(SubpacketData::Issuer(secret_key.key_id())),
        ])
        .unhashed_subpackets(vec![])
        .build()?;
    Ok(sig_conf.sign(secret_key, passwd_fn, data)?)
}

/// Sign the given file with the specified secret key and password function.
///
/// # Errors
///
/// This function will return an error if:
/// - The file or secret key cannot be read.
/// - Secret key is invalid.
/// - Failed to write to signature file.
pub async fn sign_file_with_key<F, S, PF>(
    file_path: F,
    secret_key_path: S,
    passwd_fn: PF,
) -> Result<PathBuf>
where
    F: AsRef<Path>,
    S: AsRef<Path> + Send,
    PF: FnOnce() -> String + Clone,
{
    let file = File::open(&file_path).await?;
    let secret_key = SignedSecretKey::try_from_file(secret_key_path).await?;
    let signature = sign(file.into_std().await, &secret_key, passwd_fn)?;
    let mut signature_path = file_path.as_ref().to_owned();
    signature_path.as_mut_os_string().push(".sig");
    let mut signature_file = File::create(&signature_path).await?.into_std().await;
    packet::write_packet(&mut signature_file, &signature)?;
    Ok(signature_path)
}

/// Verify the given signature with the specified public key.
///
/// # Errors
///
/// This function will return an error if:
/// - The file, signature or public key cannot be read.
/// - The signature or public key's format is invalid.
///
/// However, these will not be considered an error, an `Ok(false)` is returned instead:
/// - The signature is signed with a secret key having a key ID different from that of public key.
/// - The file's hash does not match the one in signature.
pub async fn verify_file_with_key<S, P>(signature_path: S, public_key_path: P) -> Result<bool>
where
    S: AsRef<Path> + Send,
    P: AsRef<Path> + Send,
{
    let signature_path = signature_path.as_ref();
    let signature = Signature::try_from_file(signature_path).await?;
    let public_key = SignedPublicKey::try_from_file(public_key_path).await?;
    if let Some(file_path) = signature_path
        .extension()
        .is_some_and(|ext| ext == "sig")
        .then(|| signature_path.with_extension(""))
    {
        Ok(signature
            .verify(&public_key, File::open(file_path).await?.into_std().await)
            .is_ok())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Original file not found.",
        ))?
    }
}

#[cfg(test)]
mod tests {

    use super::sign;
    use crate::{key_pair::KeyPair, Result};

    #[test]
    fn test_sign() -> Result<()> {
        let key_pair = KeyPair::generate("example", "example@example.com", String::new)?;
        let (secret_key, public_key) = (key_pair.secret_key(), key_pair.public_key());
        let signature = sign(&b"Hello"[..], &secret_key, String::new)?;
        signature.verify(public_key, &b"Hello"[..])?;
        Ok(())
    }

    #[test]
    fn test_sign_error() -> Result<()> {
        let key_pair = KeyPair::generate("example", "example@example.com", String::new)?;
        let (secret_key, public_key) = (key_pair.secret_key(), key_pair.public_key());
        let signature = sign(&b"Hello"[..], &secret_key, String::new)?;
        eprintln!(
            "{:?}",
            signature
                .verify(&public_key, &b"Help"[..])
                .expect_err("Should not pass")
        );
        Ok(())
    }
}
