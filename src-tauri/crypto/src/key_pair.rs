use pgp::{
    crypto::{hash::HashAlgorithm, sym::SymmetricKeyAlgorithm},
    types::{CompressionAlgorithm, SecretKeyTrait as _},
    KeyType, SecretKeyParamsBuilder, SignedPublicKey, SignedSecretKey,
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
    pub(crate) fn generate(
        name: &str,
        email: &str,
        passwd_fn: impl FnOnce() -> String + Clone,
    ) -> Result<Self> {
        let secret_key = SecretKeyParamsBuilder::default()
            // Set keygen params.
            .key_type(KeyType::Ed25519)
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
            .generate(thread_rng())?;
        let signed_secret_key = secret_key.sign(thread_rng(), passwd_fn.clone())?;
        let public_key = signed_secret_key.public_key();
        let signed_public_key = public_key.sign(thread_rng(), &signed_secret_key, passwd_fn)?;

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
