use std::path::Path;

use digital_signature_crypto::signing::sign_file_with_key;
use futures::future::join_all;

use crate::error::Result;

#[tauri::command]
pub async fn sign_files(file_paths: Vec<&Path>, key_path: &Path) -> Result<()> {
    let results = join_all(
        file_paths
            .into_iter()
            .map(|file_path| sign_file_with_key(file_path, key_path)),
    )
    .await;

    for ele in results {
        ele?;
    }

    // for &file_path in file_paths {
    //     sign_file_with_key(file_path, key_path);
    // }
    Ok(())
}
