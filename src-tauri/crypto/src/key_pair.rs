use pgp::{
    composed::{KeyType, SecretKeyParamsBuilder, SignedPublicKey, SignedSecretKey},
    crypto::{hash::HashAlgorithm, sym::SymmetricKeyAlgorithm},
    types::CompressionAlgorithm,
};
use rand::thread_rng;
use smallvec::smallvec;

use crate::Result;

/// Represent a generated key pair.
///
/// The key pair is already signed for usage.
pub(crate) struct KeyPair {
    secret_key: SignedSecretKey,
    public_key: SignedPublicKey,
}

impl KeyPair {
    /// Generate a [`KeyPair`] instance from the given name and email, and the password function.
    pub(crate) fn generate(name: &str, email: &str, passwd_str: &str) -> Result<Self> {
        let signed_secret_key = SecretKeyParamsBuilder::default()
            // Set keygen params.
            .key_type(KeyType::Ed25519)
            .primary_user_id(format!("{name} <{email}>"))
            .preferred_symmetric_algorithms(smallvec![
                SymmetricKeyAlgorithm::AES256,
                SymmetricKeyAlgorithm::AES192,
                SymmetricKeyAlgorithm::AES128,
                SymmetricKeyAlgorithm::TripleDES,
            ])
            .preferred_hash_algorithms(smallvec![
                HashAlgorithm::Sha512,
                HashAlgorithm::Sha384,
                HashAlgorithm::Sha256,
                HashAlgorithm::Sha224,
                HashAlgorithm::Sha1,
            ])
            .preferred_compression_algorithms(smallvec![
                CompressionAlgorithm::ZLIB,
                CompressionAlgorithm::BZip2,
                CompressionAlgorithm::ZIP
            ])
            .passphrase(Some(passwd_str.to_string()))
            .can_sign(true)
            .build()
            .expect("Failed to build secret key parameters")
            .generate(thread_rng())?;
        let signed_public_key = signed_secret_key.to_public_key();

        Ok(KeyPair::from_keys(signed_secret_key, signed_public_key))
    }

    pub(crate) fn from_keys(secret_key: SignedSecretKey, public_key: SignedPublicKey) -> Self {
        Self {
            secret_key,
            public_key,
        }
    }
    pub(crate) fn secret_key(&self) -> &SignedSecretKey {
        &self.secret_key
    }

    pub(crate) fn public_key(&self) -> &SignedPublicKey {
        &self.public_key
    }
}
