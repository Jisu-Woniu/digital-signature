//! Utilities for signing files.

use chrono::Utc;
use pgp::{
    packet::{SignatureConfigBuilder, SignatureType, Subpacket, SubpacketData},
    types::{PublicKeyTrait, SecretKeyTrait},
    Signature,
};

use crate::Result;

/// Generate a signature of the given data.
pub fn sign(data: &[u8], secret_key: &impl SecretKeyTrait) -> Result<Signature> {
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

/// Verify a signature of the given data.
pub fn verify(data: &[u8], public_key: &impl PublicKeyTrait, signature: &Signature) -> Result<()> {
    Ok(signature.verify(public_key, data)?)
}

#[cfg(test)]
mod tests {
    use super::{sign, verify, Result};
    use crate::key_pair::KeyPair;

    #[test]
    fn test_sign() -> Result<()> {
        let key_pair = KeyPair::generate("DS", "ds@example.com")?;
        let (secret_key, public_key) = (key_pair.secret_key(), key_pair.public_key());
        let signature = sign(b"Hello", &secret_key)?;
        verify(b"Hello", &public_key, &signature)?;
        Ok(())
    }

    #[test]
    fn test_sign_error() -> Result<()> {
        let key_pair = KeyPair::generate("DS", "ds@example.com")?;
        let (secret_key, public_key) = (key_pair.secret_key(), key_pair.public_key());
        let signature = sign(b"Hello", &secret_key)?;
        eprintln!(
            "{:?}",
            verify(b"Help", &public_key, &signature).expect_err("Should not pass")
        );
        Ok(())
    }
}
