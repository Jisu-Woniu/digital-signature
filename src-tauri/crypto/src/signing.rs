//! Utilities for signing files.

use std::path::{Path, PathBuf};

use chrono::Utc;
use pgp::{
    packet::{self, SignatureConfigBuilder, SignatureType, Subpacket, SubpacketData},
    types::{PublicKeyTrait, SecretKeyTrait},
    Signature, SignedSecretKey,
};
use tokio::fs::{read, File};

use crate::{from_file::FromFile, Result};

/// Generate a signature of the given data.
fn sign(data: &[u8], secret_key: &impl SecretKeyTrait) -> Result<Signature> {
    let now = Utc::now();
    let sig_conf = SignatureConfigBuilder::default()
        .pub_alg(secret_key.algorithm())
        .typ(SignatureType::Binary)
        .issuer(Some(secret_key.key_id()))
        .created(Some(now))
        .hashed_subpackets(vec![
            Subpacket::regular(SubpacketData::SignatureCreationTime(now)),
            Subpacket::regular(SubpacketData::Issuer(secret_key.key_id())),
        ])
        .unhashed_subpackets(vec![])
        .build()?;
    Ok(sig_conf.sign(secret_key, String::new, data)?)
}
#[allow(dead_code)]
/// Verify a signature of the given data.
fn verify(data: &[u8], public_key: &impl PublicKeyTrait, signature: &Signature) -> Result<()> {
    Ok(signature.verify(public_key, data)?)
}

/// Sign the given file with the given secret key.
///
/// # Errors
///
/// This function will return an error if:
/// - The file or secret key cannot be read.
/// - Secret key is invalid.
/// - Failed to write to signature file.
pub async fn sign_file_with_key(
    file_path: impl AsRef<Path>,
    secret_key_path: impl AsRef<Path>,
) -> Result<PathBuf> {
    let file_data = read(&file_path).await?;
    let secret_key = SignedSecretKey::try_from_file(secret_key_path.as_ref()).await?;
    let signature = sign(&file_data, &secret_key)?;
    let mut signature_path = file_path.as_ref().to_owned();
    signature_path.as_mut_os_string().push(".sig");
    let mut signature_file = File::options()
        .create(true)
        .write(true)
        .open(&signature_path)
        .await?
        .into_std()
        .await;
    packet::write_packet(&mut signature_file, &signature)?;
    Ok(signature_path)
}

#[cfg(test)]
mod tests {

    use pgp::{Signature, SignedPublicKey};
    use temp_dir::TempDir;
    use tokio::fs::write;

    use super::{sign, sign_file_with_key, verify};
    use crate::{
        from_file::FromFile,
        key_pair::KeyPair,
        keygen::{write_key_pair, KeyPairPath},
        Result,
    };

    #[test]
    fn test_sign() -> Result<()> {
        let key_pair = KeyPair::generate("example", "example@example.com", String::new)?;
        let (secret_key, public_key) = (key_pair.secret_key(), key_pair.public_key());
        let signature = sign(b"Hello", &secret_key)?;
        verify(b"Hello", &public_key, &signature)?;
        Ok(())
    }

    #[test]
    fn test_sign_error() -> Result<()> {
        let key_pair = KeyPair::generate("example", "example@example.com", String::new)?;
        let (secret_key, public_key) = (key_pair.secret_key(), key_pair.public_key());
        let signature = sign(b"Hello", &secret_key)?;
        eprintln!(
            "{:?}",
            verify(b"Help", &public_key, &signature).expect_err("Should not pass")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_sign_file() -> Result<()> {
        let tmp_dir = TempDir::new()?;
        let KeyPairPath {
            secret_key_path,
            public_key_path,
        } = write_key_pair(
            "example",
            "example@example.com",
            String::new,
            tmp_dir.path(),
        )
        .await?;

        dbg!(&secret_key_path);

        let data = b"Hello, world!";

        let data_path = tmp_dir.path().join("data");
        write(&data_path, data).await?;

        let public_key = SignedPublicKey::try_from_file(&public_key_path).await?;

        let sig_path = sign_file_with_key(&data_path, secret_key_path).await?;

        let signature = Signature::try_from_file(sig_path).await?;

        dbg!(&signature);

        let verified = verify(data, &public_key, &signature).is_ok();

        dbg!(verified);

        Ok(())
    }
}
