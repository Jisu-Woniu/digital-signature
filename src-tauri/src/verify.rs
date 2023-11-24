use std::{
    collections::HashMap,
    iter::zip,
    path::{Path, PathBuf},
};

use digital_signature_crypto::signing::verify_file_with_key;
use futures::future::try_join_all;

use crate::error::Result;
#[tauri::command]
pub async fn verify_signatures(
    signature_paths: Vec<PathBuf>,
    public_key_path: &Path,
) -> Result<HashMap<PathBuf, bool>> {
    let result = try_join_all(
        signature_paths
            .iter()
            .map(|sig_path| verify_file_with_key(sig_path, public_key_path)),
    )
    .await?;
    Ok(zip(signature_paths, result).collect::<HashMap<PathBuf, bool>>())
}

#[cfg(test)]
mod tests {
    use std::path;

    #[test]
    fn test() {
        dbg!(path::is_separator('/'));
        dbg!(path::is_separator('\\'));
    }
}
