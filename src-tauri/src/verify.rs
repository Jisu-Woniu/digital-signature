use std::{convert::identity, path::Path};

use digital_signature_crypto::signing::verify_file_with_key;
use futures::future::try_join_all;

use crate::error::Result;
#[tauri::command]
pub async fn verify_signatures(sig_paths: Vec<&Path>, public_key_path: &Path) -> Result<bool> {
    let result = try_join_all(
        sig_paths
            .into_iter()
            .map(|sig_path| verify_file_with_key(sig_path, public_key_path)),
    )
    .await?;
    Ok(result.into_iter().all(identity))
}
