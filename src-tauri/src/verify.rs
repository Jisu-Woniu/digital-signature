use std::{
    iter::zip,
    path::{Path, PathBuf},
};

use digital_signature_crypto::signing::verify_file_with_key;
use futures::future::try_join_all;

use crate::error::Result;
#[tauri::command]
pub async fn verify_signatures(
    sig_paths: Vec<PathBuf>,
    public_key_path: &Path,
) -> Result<Vec<(PathBuf, bool)>> {
    let result = try_join_all(
        sig_paths
            .iter()
            .map(|sig_path| verify_file_with_key(sig_path, public_key_path)),
    )
    .await?;
    Ok(zip(sig_paths, result).collect())
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
