#![warn(missing_docs)]
//! Cryptographic functions for the digital signature app.

pub mod error;
mod from_file;
mod key_pair;
pub mod keygen;
mod secret_file;
pub mod signing;

pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use pgp::{Signature, SignedPublicKey};
    use temp_dir::TempDir;
    use tokio::fs::write;

    use crate::{
        error::Result,
        from_file::FromFile,
        keygen::{write_key_pair, KeyPairPaths},
        signing::sign_file_with_key,
    };

    #[tokio::test]
    async fn test_whole_process() -> Result<()> {
        let tmp_dir = TempDir::new()?;
        let KeyPairPaths {
            secret_key_path,
            public_key_path,
        } = write_key_pair(
            "example",
            "example@example.com",
            String::new,
            tmp_dir.path(),
        )
        .await?;

        let data = b"Hello, world!";
        let data_path = tmp_dir.path().join("data");

        write(&data_path, data).await?;

        let sig_path = sign_file_with_key(&data_path, &secret_key_path, String::new).await?;

        let signature = Signature::try_from_file(sig_path).await?;
        let public_key = SignedPublicKey::try_from_file(&public_key_path).await?;

        let verified = signature.verify(&public_key, &data[..]).is_ok();

        assert!(verified);

        Ok(())
    }
}
