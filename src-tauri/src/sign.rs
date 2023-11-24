use std::path::{Path, PathBuf};

use digital_signature_crypto::signing::sign_file_with_key;
use futures::future::try_join_all;

use crate::error::Result;

#[tauri::command]
pub async fn sign_files(
    file_paths: Vec<&Path>,
    private_key_path: &Path,
    passwd: &str,
) -> Result<Vec<PathBuf>> {
    Ok(try_join_all(
        file_paths.into_iter().map(|file_path| {
            sign_file_with_key(file_path, private_key_path, || String::from(passwd))
        }),
    )
    .await?)
}
