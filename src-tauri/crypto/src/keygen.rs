use crate::{error::Result, secret_file::write_secret_file};
use std::path::Path;

use pgp::{
    crypto::{hash::HashAlgorithm, sym::SymmetricKeyAlgorithm},
    types::{CompressionAlgorithm, KeyTrait, SecretKeyTrait as _},
    KeyType, SecretKeyParamsBuilder, SignedPublicKey, SignedSecretKey,
};
use smallvec::smallvec;
use tokio::{
    fs::{self, DirBuilder},
    try_join,
};
use zeroize::Zeroizing;

pub async fn write_key_pair(name: &str, email: &str, path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    // Create output directory if not exist.
    DirBuilder::new().recursive(true).create(path).await?;

    let (signed_secret_key, signed_public_key) = gen_key_pair(name, email)?;
    let keyid = &hex::encode_upper(&signed_secret_key.key_id().as_ref()[4..]);

    let secret_key = path.join(format!("{}_0x{}_SECRET.asc", name, keyid));
    let public_key = path.join(format!("{}_0x{}_public.asc", name, keyid));

    let secret_key_pem = Zeroizing::new(signed_secret_key.to_armored_bytes(None)?);
    let pub_key_pem = signed_public_key.to_armored_bytes(None)?;

    try_join!(
        write_secret_file(&secret_key, &secret_key_pem),
        fs::write(&public_key, &pub_key_pem)
    )?;

    Ok(())
}

fn gen_key_pair(name: &str, email: &str) -> Result<(SignedSecretKey, SignedPublicKey)> {
    let secret_key = SecretKeyParamsBuilder::default()
        // Set keygen params.
        .key_type(KeyType::EdDSA)
        .primary_user_id(format!("{} <{}>", name, email))
        .preferred_symmetric_algorithms(smallvec![
            SymmetricKeyAlgorithm::AES256,
            SymmetricKeyAlgorithm::AES192,
            SymmetricKeyAlgorithm::AES128,
            SymmetricKeyAlgorithm::TripleDES,
        ])
        .preferred_hash_algorithms(smallvec![
            HashAlgorithm::SHA2_512,
            HashAlgorithm::SHA2_384,
            HashAlgorithm::SHA2_256,
            HashAlgorithm::SHA2_224,
            HashAlgorithm::SHA1
        ])
        .preferred_compression_algorithms(smallvec![
            CompressionAlgorithm::ZLIB,
            CompressionAlgorithm::BZip2,
            CompressionAlgorithm::ZIP
        ])
        .can_sign(true)
        .build()
        .expect("msg")
        .generate()
        .expect("Failed to generate a plain key.");
    let passwd_fn = String::new;
    let signed_secret_key = secret_key.sign(passwd_fn)?;
    let public_key = signed_secret_key.public_key();
    let signed_public_key = public_key.sign(&signed_secret_key, passwd_fn)?;

    Ok((signed_secret_key, signed_public_key))
}

#[cfg(test)]
mod tests {

    use pgp::{types::KeyTrait, Deserializable, SignedSecretKey};
    use tokio::fs::read_to_string;

    use super::{gen_key_pair, Result};

    #[tokio::test]
    #[ignore = "Manual testing for file generation."]
    async fn test() -> Result<()> {
        let (secret_key, public_key) = gen_key_pair("极速蜗牛", "jswn@jswn9945.xyz")?;
        println!("{}", secret_key.to_armored_string(None)?);
        println!("{}", public_key.to_armored_string(None)?);
        dbg!(public_key);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "Manual testing for file parsing."]
    async fn extract_key_info() -> Result<()> {
        let secret_key_str =
            read_to_string("/home/jswn/GpgPlayground/极速蜗牛_0x21B55C62_SECRET.asc").await?;

        let secret_key = SignedSecretKey::from_string(&secret_key_str)?.0;
        dbg!(&secret_key);
        let key_id = secret_key.key_id();

        dbg!(&key_id);

        dbg!(&hex::encode_upper(&key_id.as_ref()[4..]));

        Ok(())
    }
}
