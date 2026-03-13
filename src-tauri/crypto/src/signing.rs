//! Utilities for signing files.

use std::{
    io::{self, Read},
    path::{Path, PathBuf},
};

use pgp::{
    composed::{SignedPublicKey, SignedSecretKey},
    packet::{PacketTrait, Signature, SignatureConfig, SignatureType, Subpacket, SubpacketData},
    types::{Password, SigningKey, Timestamp},
};
use tokio::fs::File;

use crate::{from_file::FromFile, Result};

/// Generate a signature of the given data.
fn sign(data: impl Read, secret_key: &impl SigningKey, passwd: &Password) -> Result<Signature> {
    let mut sig_conf = SignatureConfig::v4(
        SignatureType::Binary,
        secret_key.algorithm(),
        secret_key.hash_alg(),
    );

    sig_conf.hashed_subpackets = vec![
        Subpacket::regular(SubpacketData::SignatureCreationTime(Timestamp::now()))?,
        Subpacket::regular(SubpacketData::IssuerKeyId(secret_key.legacy_key_id()))?,
    ];

    Ok(sig_conf.sign(secret_key, passwd, data)?)
}

/// Sign the given file with the specified secret key and password function.
///
/// # Errors
///
/// This function will return an error if:
/// - The file or secret key cannot be read.
/// - Secret key is invalid.
/// - Failed to write to signature file.
pub async fn sign_file_with_key<F, S>(
    file_path: F,
    secret_key_path: S,
    passwd: &Password,
) -> Result<PathBuf>
where
    F: AsRef<Path>,
    S: AsRef<Path> + Send,
{
    let file = File::open(&file_path).await?;
    let secret_key = SignedSecretKey::try_from_file(secret_key_path).await?;
    // let secret_key = ;
    let signature = sign(file.into_std().await, &*secret_key, passwd)?;
    let mut signature_path = file_path.as_ref().to_owned();
    signature_path.as_mut_os_string().push(".sig");
    let mut signature_file = File::create(&signature_path).await?.into_std().await;
    // packet::write_packet(&mut signature_file, &signature)?;
    signature.to_writer_with_header(&mut signature_file)?;
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

    use pgp::types::Password;

    use super::sign;
    use crate::{key_pair::KeyPair, Result};

    #[test]
    fn test_sign() -> Result<()> {
        let key_pair = KeyPair::generate("example", "example@example.com", "")?;
        let (secret_key, public_key) = (key_pair.secret_key(), key_pair.public_key());
        let signature = sign(&b"Hello"[..], &**secret_key, &Password::empty())?;
        signature.verify(public_key, &b"Hello"[..])?;
        Ok(())
    }

    #[test]
    fn test_sign_error() -> Result<()> {
        let key_pair = KeyPair::generate("example", "example@example.com", "")?;
        let (secret_key, public_key) = (key_pair.secret_key(), key_pair.public_key());
        let signature = sign(&b"Hello"[..], &**secret_key, &Password::empty())?;
        eprintln!(
            "{:?}",
            signature
                .verify(&public_key, &b"Help"[..])
                .expect_err("Should not pass")
        );
        Ok(())
    }
}
