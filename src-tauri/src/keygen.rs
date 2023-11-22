//! Used for key generation.

use std::path::Path;

use digital_signature_crypto::keygen::write_key_pair;

use crate::error::Result;

#[tauri::command]
pub async fn generate_key_pair(name: &str, email: &str, path: &Path) -> Result<()> {
    write_key_pair(name, email, String::new, path).await?;
    Ok(())
}
