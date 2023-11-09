use crate::{error::Result, secret_file::write_secret_file};
use std::{
    io::{self},
    path::Path,
};

use ed25519::{
    pkcs8::{
        spki::der::{pem::LineEnding, zeroize::Zeroizing},
        EncodePrivateKey, EncodePublicKey,
    },
    KeypairBytes, PublicKeyBytes,
};
use ed25519_dalek::SigningKey;
use rand::thread_rng;
use tokio::{
    fs::{self, DirBuilder},
    try_join,
};

pub async fn write_key_pair(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        let name = path
            .file_name()
            .ok_or(io::Error::from(io::ErrorKind::InvalidInput))?;
        DirBuilder::new().recursive(true).create(parent).await?;
        let mut priv_name = name.to_os_string();
        priv_name.push("_priv.pem");

        let mut pub_name = name.to_os_string();
        pub_name.push("_pub.pem");

        let priv_key = path.with_file_name(priv_name);
        let pub_key = path.with_file_name(pub_name);

        let (priv_key_pem, pub_key_pem) = gen_keypair()?;

        try_join!(
            write_secret_file(&priv_key, priv_key_pem.as_bytes()),
            fs::write(&pub_key, pub_key_pem.as_bytes())
        )?;
    } else {
        Err(io::Error::from(io::ErrorKind::InvalidInput))?
    }
    Ok(())
}

fn gen_keypair() -> Result<(Zeroizing<String>, String)> {
    let key_pair =
        KeypairBytes::from_bytes(&SigningKey::generate(&mut thread_rng()).to_keypair_bytes());
    let priv_key_pem = key_pair
        .to_pkcs8_der()?
        .to_pem("PRIVATE KEY", LineEnding::default())?;
    let pub_key_pem = PublicKeyBytes::try_from(&key_pair)?
        .to_public_key_der()?
        .to_pem("PUBLIC KEY", LineEnding::default())?;
    Ok((priv_key_pem, pub_key_pem))
}

#[cfg(test)]
mod tests {

    use super::{gen_keypair, Result};

    #[tokio::test]
    #[ignore = "Manual testing for file generation."]
    async fn test() -> Result<()> {
        let (priv_key, pub_key) = gen_keypair()?;
        println!("{}", *priv_key);
        println!("{}", pub_key);
        Ok(())
    }
}
