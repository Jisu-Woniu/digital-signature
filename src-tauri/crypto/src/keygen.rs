use crate::error::Result;
use std::{
    fs::DirBuilder,
    io::{self},
    path::Path,
};

use ed25519::{
    pkcs8::{EncodePrivateKey, EncodePublicKey},
    KeypairBytes,
};
use ed25519_dalek::SigningKey;
use rand::thread_rng;
use rsa::pkcs8::LineEnding;

pub fn write_key_pair(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        let name = path
            .file_name()
            .ok_or(io::Error::from(io::ErrorKind::InvalidInput))?;
        DirBuilder::new().recursive(true).create(parent)?;
        let mut priv_name = name.to_os_string();
        priv_name.push("_priv.pem");

        let mut pub_name = name.to_os_string();
        pub_name.push("_pub.pem");

        let priv_key = path.with_file_name(priv_name);
        let pub_key = path.with_file_name(pub_name);

        let key_pair =
            KeypairBytes::from_bytes(&SigningKey::generate(&mut thread_rng()).to_keypair_bytes());
        key_pair.write_pkcs8_pem_file(priv_key, LineEnding::default())?;
        key_pair
            .public_key
            .unwrap()
            .write_public_key_pem_file(pub_key, LineEnding::default())?;
    } else {
        Err(io::Error::from(io::ErrorKind::InvalidInput))?
    }
    Ok(())
}
