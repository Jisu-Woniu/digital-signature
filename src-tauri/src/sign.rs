use std::path::{Path, PathBuf};

use digital_signature_crypto::signing::sign_file_with_key;
use futures::{future::join_all, TryFutureExt};

use crate::error::Result;

#[tauri::command]
pub async fn sign_files(file_paths: Vec<&Path>, key_path: &Path) -> Result<Vec<PathBuf>> {
    join_all(
        file_paths
            .into_iter()
            .map(|file_path| sign_file_with_key(file_path, key_path).err_into()),
    )
    .await
    .into_iter()
    .collect::<Result<_>>()
}
