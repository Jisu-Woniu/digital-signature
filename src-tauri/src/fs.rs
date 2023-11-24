use std::{ffi::OsStr, path::Path};

use serde::Serialize;
use tokio::fs;

use crate::error::Result;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FileType {
    File = 0,
    Dir = 1,
    Other = 2,
    Inexist = 3,
}

impl Serialize for FileType {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[tauri::command]
pub async fn detect_file_type(path: &Path) -> Result<FileType> {
    Ok(match fs::metadata(path).await {
        Ok(metadata) => {
            if metadata.is_dir() {
                FileType::Dir
            } else if metadata.is_file() {
                FileType::File
            } else {
                FileType::Other
            }
        }
        Err(_) => FileType::Inexist,
    })
}

#[tauri::command]
pub fn get_file_names(files: Vec<&Path>) -> Vec<&OsStr> {
    files
        .into_iter()
        .filter_map(|file| file.file_name())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::detect_file_type;

    #[tokio::test]
    async fn test() {
        println!("{:?}", detect_file_type(&PathBuf::from("/home/")).await);
        println!("{:?}", detect_file_type(&PathBuf::from("/etc/fstab")).await);
        println!(
            "{:?}",
            detect_file_type(&PathBuf::from("/home/inexist")).await
        );
        println!("{:?}", detect_file_type(&PathBuf::from("/bin/sh")).await);
    }
}
