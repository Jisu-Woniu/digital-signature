// use crate::error::Result;
use std::io::Result;
use std::path::Path;

#[cfg(unix)]
pub(crate) async fn write_secret_file(path: impl AsRef<Path>, data: &[u8]) -> Result<()> {
    use tokio::{fs, io::AsyncWriteExt};

    /// File permissions for secret data
    #[cfg(unix)]
    const SECRET_FILE_PERMS: u32 = 0o600;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(SECRET_FILE_PERMS)
        .open(path)
        .await?;
    file.write_all(data).await?;

    Ok(())
}

#[cfg(not(unix))]
pub(crate) async fn write_secret_file(path: impl AsRef<Path>, data: &[u8]) -> Result<()> {
    use tokio::fs;

    fs::write(path, data).await?;
    Ok(())
}
