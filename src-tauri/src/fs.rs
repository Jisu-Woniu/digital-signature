use std::{ffi::OsStr, path::Path};

use serde::Serialize;
use tokio::fs::metadata;

use crate::error::Result;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FileType {
    File = 0,
    Dir = 1,
    Other = 2,
    Unavailable = 3,
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
    Ok(match metadata(path).await {
        Ok(metadata) => {
            if metadata.is_dir() {
                FileType::Dir
            } else if metadata.is_file() {
                FileType::File
            } else {
                FileType::Other
            }
        }
        Err(_) => FileType::Unavailable,
    })
}

#[tauri::command]
pub fn get_file_names(files: Vec<&Path>) -> Vec<&OsStr> {
    files
        .into_iter()
        .filter_map(|file| file.file_name())
        .collect()
}
