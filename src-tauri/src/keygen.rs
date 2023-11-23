//! Used for key generation.

use std::path::Path;

use digital_signature_crypto::keygen::{write_key_pair, KeyPairPaths};

use crate::error::Result;

#[tauri::command]
pub async fn generate_key_pair(
    name: &str,
    email: &str,
    passwd: &str,
    path: &Path,
) -> Result<KeyPairPaths> {
    Ok(write_key_pair(name, email, || String::from(passwd), path).await?)
}
